use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    parser::types::OperationType,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_macros::debug_handler;
use graphql::schema::AppSchema;
use hyper::HeaderMap;
use log::info;
use sea_orm::DatabaseConnection;

use crate::graphql::schema::build_schema;
pub mod graphql;
pub mod jwk;

#[debug_handler]
async fn graphql_handler(
    schema: State<AppSchema>,
    headers: HeaderMap,
    mut req: GraphQLRequest,
) -> GraphQLResponse {
    let parsed_query = req.0.parsed_query();
    match parsed_query {
        Ok(doc) => {
            let has_mutation = doc
                .operations
                .iter()
                .any(|x| x.1.node.ty == OperationType::Mutation);
            if has_mutation {
                let _auth_header = headers
                    .get(http::header::AUTHORIZATION)
                    .and_then(|header| header.to_str().ok());
                info!("Mutation detected");
            }
        }
        Err(e) => {
            info!("Error parsing query: {:?}", e);
        }
    }
    // let x = parsed_query.parsed_query().unwrap().operations;
    schema.execute(req.into_inner()).await.into()
}

// async fn handle_mutation_request(
//     schema: State<AppSchema>,
//     headers: HeaderMap,
//     mut req: GraphQLRequest,
// ) -> GraphQLResponse {
// }

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

pub struct ServerSettings {
    pub db_connection: DatabaseConnection,
}

pub async fn server(settings: &ServerSettings) -> Router {
    let schema = build_schema(settings.db_connection.clone()).await;
    info!("GraphQL Playground: http://localhost:8000/api/graphql",);
    Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(schema)
}
