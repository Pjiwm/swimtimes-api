use axum::{routing::get, Router};
use graphql::schema::AppSchema;
use log::info;
use sea_orm::DatabaseConnection;

use crate::graphql::schema::build_schema;
pub mod graphql;
pub mod handlers;
pub mod jwk;

#[derive(Clone)]
pub struct AppData {
    schema: AppSchema,
    jwk_auth: jwk::JwkAuth,
}

pub struct ServerSettings {
    pub db_connection: DatabaseConnection,
}

pub async fn server(settings: &ServerSettings) -> Router {
    let app_data = AppData {
        schema: build_schema(settings.db_connection.clone()).await,
        jwk_auth: jwk::JwkAuth::new().await,
    };

    info!("GraphQL Playground: http://localhost:8000/api/graphql",);
    Router::new()
        .route(
            "/api/graphql",
            get(handlers::graphql::playground).post(handlers::graphql::auth_handler),
        )
        .with_state(app_data)
}
