pub mod team;
pub mod competition;
pub mod swimmer;
pub mod swim_time;
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(team::TeamMutation, competition::CompetitionMutation, swimmer::SwimmerMutation);
