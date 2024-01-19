pub mod competition;
pub mod swim_time;
pub mod swimmer;
pub mod team;
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    team::TeamMutation,
    competition::CompetitionMutation,
    swimmer::SwimmerMutation,
    swim_time::SwimTimeMutation,
);
