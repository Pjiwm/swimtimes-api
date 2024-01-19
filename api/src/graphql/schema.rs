use super::{mutation::Mutation, query::Query};
use async_graphql::extensions::Logger;
use async_graphql::{EmptySubscription, Schema};
use repository::competition_repo::CompetitionRepo;
use repository::swim_time_repo::SwimTimeRepo;
use repository::swimmer_repo::SwimmerRepo;
use repository::team_repo::TeamRepo;
use sea_orm::DatabaseConnection;
pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema(db: DatabaseConnection) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(Logger)
        .data(TeamRepo::new(db.clone()))
        .data(CompetitionRepo::new(db.clone()))
        .data(SwimmerRepo::new(db.clone()))
        .data(SwimTimeRepo::new(db))
        .finish()
}
