use axum::{
    routing::{get, post},
    Router,
};
use log::{info, warn};

use crate::AppData;

mod graphql;

const ROUTE: &str = "/graphql";
pub fn graphql_router(enable_authentication: bool) -> Router<AppData> {
    let mut router = Router::new().route(ROUTE, get(graphql::playground));

    if enable_authentication {
        router = router.route(ROUTE, post(graphql::auth_handler));
    } else {
        warn!("Authentication is disabled for GraphQL, this shoud only be used in a development environment");
        router = router.route(ROUTE, post(graphql::local_handler));
    }
    info!("GraphQL Playground: http://localhost:8000/api/graphql");
    router
}
