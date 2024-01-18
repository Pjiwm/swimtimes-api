pub mod team;
pub mod competition;
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(team::TeamQuery, competition::CompetitionQuery);
