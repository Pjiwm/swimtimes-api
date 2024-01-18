use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_macros::debug_handler;
use graphql::schema::AppSchema;
use log::info;
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

use crate::graphql::schema::build_schema;
pub mod graphql;

#[debug_handler]
async fn graphql_handler(schema: State<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

pub struct ServerSettings {
    pub db_connection: DatabaseConnection,
    pub port: u16,
}

pub async fn run_server(settings: ServerSettings) {
    #[cfg(debug_assertions)]
    let schema = build_schema(settings.db_connection.clone()).await;


    env_logger::init();
    let app = Router::new()
        .route(
            "/api/graphql",
            get(graphql_playground).post(graphql_handler),
        )
        .with_state(schema);
        // .layer(axum::middleware::from_fn(middleware::log_middleware));
    info!("Starting server on port {}", settings.port);
    info!("GraphQL Playground: http://localhost:{}/api/graphql", settings.port);
    let bind_addr = format!("0.0.0.0:{}", settings.port);
    let listener = TcpListener::bind(bind_addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
