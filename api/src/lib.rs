use axum::Router;
use graphql::schema::AppSchema;
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
    pub use_auth: bool,
    pub use_playground: bool,
}

pub async fn server(settings: &ServerSettings) -> Router {
    let app_data = AppData {
        schema: build_schema(settings.db_connection.clone()).await,
        jwk_auth: jwk::JwkAuth::new().await,
    };
    let graphql = handlers::graphql_router(settings.use_auth);
    Router::new().nest("/api", graphql).with_state(app_data)
}
