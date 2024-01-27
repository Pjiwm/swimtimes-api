use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State, http::HeaderValue, response::{Html, IntoResponse}, routing::{get, post}, Router
};
use axum_macros::debug_handler;
use graphql::schema::AppSchema;
use hyper::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN}, HeaderMap, Method};
use log::info;
use sea_orm::DatabaseConnection;
use tower_http::{cors::CorsLayer, services::{ServeDir, ServeFile}};

use crate::graphql::schema::build_schema;
pub mod graphql;
pub mod jwt;

#[derive(Clone)]
pub struct AppData {
    schema: AppSchema,
}

pub struct ServerSettings {
    pub db_connection: DatabaseConnection,
    pub use_auth: bool,
    pub use_playground: bool,
}

pub async fn server(settings: &ServerSettings) -> Router {
    let app_data = AppData {
        schema: build_schema(settings.db_connection.clone(), settings.use_auth).await,
    };

    let cors = CorsLayer::new()
    .allow_credentials(true)
    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT, CONTENT_TYPE])
    .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap());


    let graphql = graphql_router();
    let web = web_router();

    Router::new()
        .nest("/api", graphql)
        .with_state(app_data)
        .layer(cors)
        .nest("/", web)
}

const ROUTE: &str = "/graphql";
pub fn graphql_router() -> Router<AppData> {
    info!("GraphQL playground: http://localhost:8000/api{}", ROUTE);
    Router::new()
        .route(ROUTE, get(playground))
        .route(ROUTE, post(graphql_handler))
}

pub async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

#[debug_handler]
pub async fn graphql_handler(
    data: State<AppData>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    data.schema
        .execute(req.into_inner().data(headers))
        .await
        .into()
}

fn web_router() -> Router {
    Router::new().nest_service(
        "/",
        ServeDir::new("static").not_found_service(ServeFile::new("static/index.html")),
    )
}
