pub mod team;
pub mod competition;
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(team::TeamMutation, competition::CompetitionMutation);
