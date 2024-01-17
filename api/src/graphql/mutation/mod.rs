pub mod team;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(team::TeamMutation);
