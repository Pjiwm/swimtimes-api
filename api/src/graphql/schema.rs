use super::{mutation::Mutation, query::Query};
use async_graphql::{EmptySubscription, Schema};
use infrastructure::team_repo::TeamRepo;
use sea_orm::DatabaseConnection;
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema(db: DatabaseConnection) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(TeamRepo::new(db))
        .finish()
}