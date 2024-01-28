use std::sync::Arc;

use async_graphql::{
    extensions::{Extension, ExtensionContext, ExtensionFactory, NextParseQuery},
    parser::types::{ExecutableDocument, OperationType},
    ServerError, ServerResult, Variables,
};
use async_trait::async_trait;
use axum::http;
use hyper::HeaderMap;

use crate::jwt::{self, Claims};

pub struct Auth;

impl ExtensionFactory for Auth {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(AuthExtension::default())
    }
}

#[derive(Default)]
struct AuthExtension {
    jwt_verifier: jwt::JwtVerifier,
}

#[async_trait]
impl Extension for AuthExtension {
    async fn parse_query(
        &self,
        ctx: &ExtensionContext<'_>,
        query: &str,
        variables: &Variables,
        next: NextParseQuery<'_>,
    ) -> ServerResult<ExecutableDocument> {
        let document = next.run(ctx, query, variables).await?;
        if !has_mutation(&document) {
            Ok(document)
        } else {
            let auth_header = ctx
                .data::<HeaderMap>()
                .map_err(|_| ServerError::new("Could not obtain headers", None))?
                .get(http::header::AUTHORIZATION)
                .and_then(|header| header.to_str().ok())
                .ok_or(ServerError::new("No authorization header", None))?;

            let jwt = get_token_from_header(auth_header).ok_or(ServerError::new(
                "Authorization header is formatted invalid.",
                None,
            ))?;

            let claims = self
                .jwt_verifier
                .check_claims(jwt)
                .map_err(|_| ServerError::new("Authorization header is invalid.", None))?;

            check_role_permission(claims)?;
            Ok(document)
        }
    }
}

fn has_mutation(doc: &ExecutableDocument) -> bool {
    doc.operations
        .iter()
        .any(|x| x.1.node.ty == OperationType::Mutation)
}

fn get_token_from_header(header: &str) -> Option<String> {
    let prefix_len = "Bearer ".len();

    match header.len() {
        l if l < prefix_len => None,
        _ => Some(header[prefix_len..].to_string()),
    }
}

fn check_role_permission(claims: Claims) -> ServerResult<()> {
    match claims.role.as_str() {
        "manager" => Ok(()),
        other_role => Err(ServerError::new(
            format!("'{other_role}' user is not authorized for this action."),
            None,
        )),
    }
}
