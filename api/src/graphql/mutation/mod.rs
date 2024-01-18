pub mod team;
pub mod competition;
pub mod swimmer;
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(team::TeamMutation, competition::CompetitionMutation, swimmer::SwimmerMutation);
