use std::sync::Arc;

use async_graphql::{
    extensions::{Extension, ExtensionContext, NextParseQuery, ExtensionFactory},
    parser::types::{ExecutableDocument, OperationType},
    ServerError, ServerResult, Variables,
};
use async_trait::async_trait;
use axum::http;
use hyper::HeaderMap;

use crate::jwk;

pub struct Auth;

impl ExtensionFactory for Auth {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(AuthExtension)
    }
}


struct AuthExtension;

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

            let jwk_auth = ctx
                .data::<jwk::JwkAuth>()
                .map_err(|_| ServerError::new("Could not find any authentication.", None))?;
            parse_and_verify_auth_header(auth_header, jwk_auth).await?;
            Ok(document)
        }
    }
}

fn has_mutation(doc: &ExecutableDocument) -> bool {
    doc.operations
        .iter()
        .any(|x| x.1.node.ty == OperationType::Mutation)
}

async fn parse_and_verify_auth_header(
    header: &str,
    auth: &jwk::JwkAuth,
) -> Result<(), ServerError> {
    let token = match get_token_from_header(header) {
        Some(token) => verify_token(&token, auth).await,
        None => None,
    };
    match token {
        Some(_) => Ok(()),
        None => Err(ServerError::new("Authorization header is invalid.", None)),
    }
}

fn get_token_from_header(header: &str) -> Option<String> {
    let prefix_len = "Bearer ".len();

    match header.len() {
        l if l < prefix_len => None,
        _ => Some(header[prefix_len..].to_string()),
    }
}

async fn verify_token(token: &String, auth: &jwk::JwkAuth) -> Option<String> {
    let verified_token = auth.verify(&token).await;
    match verified_token {
        Some(token) => Some(token.claims.sub),
        None => None,
    }
}
