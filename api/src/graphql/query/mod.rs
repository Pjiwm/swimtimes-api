pub mod team;

#[derive(async_graphql::MergedObject, Default)]
pub struct Query(team::TeamQuery);