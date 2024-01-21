use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    parser::types::{ExecutableDocument, OperationType},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http,
    response::{Html, IntoResponse},
};
use axum_macros::debug_handler;
use hyper::HeaderMap;
use log::warn;

use crate::{jwk, AppData};

pub async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

#[debug_handler]
pub async fn local_handler(data: State<AppData>, req: GraphQLRequest) -> GraphQLResponse {
    data.schema.execute(req.into_inner()).await.into()
}

#[debug_handler]
pub async fn auth_handler(
    data: State<AppData>,
    headers: HeaderMap,
    mut req: GraphQLRequest,
) -> GraphQLResponse {
    if let Ok(doc) = req.0.parsed_query() {
        if req_has_mutation(doc) {
            match handle_mutation_request(&data.jwk_auth, headers).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Authorization failed on graphql mutation");
                    return e.into();
                }
            }
        }
    } else {
        warn!("Error parsing query");
        return async_graphql::Response::new("Internal server error").into();
    }

    data.schema.execute(req.into_inner()).await.into()
}

fn req_has_mutation(doc: &ExecutableDocument) -> bool {
    doc.operations
        .iter()
        .any(|x| x.1.node.ty == OperationType::Mutation)
}

async fn handle_mutation_request(
    auth: &jwk::JwkAuth,
    headers: HeaderMap,
) -> Result<(), async_graphql::Response> {
    let auth_header = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(h) => parse_and_verify_auth_header(h, auth).await,
        None => Err(async_graphql::Response::new(
            "Authorization header must be set for mutations",
        )),
    }
}

async fn parse_and_verify_auth_header(
    header: &str,
    auth: &jwk::JwkAuth,
) -> Result<(), async_graphql::Response> {
    let token = match get_token_from_header(header) {
        Some(token) => verify_token(&token, auth).await,
        None => None,
    };
    match token {
        Some(_) => Ok(()),
        None => Err(async_graphql::Response::new(
            "No valid authorization header",
        )),
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
