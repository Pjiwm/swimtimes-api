pub mod team;
pub mod competition;
pub mod swimmer;
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(team::TeamQuery, competition::CompetitionQuery, swimmer::SwimmerQuery);
