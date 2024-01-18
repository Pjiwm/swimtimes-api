use async_graphql::types::Json;
pub type TeamJson = Json<entity::team::Model>;
pub type PopulatedCompetitionJson = Json<entity::records::PopulatedCompetition>;
pub type CompetitionJson = Json<entity::competition::Model>;
pub type PopulatedSwimmerJson = Json<entity::records::PopulatedSwimmer>;
pub type SwimmerJson = Json<entity::swimmer::Model>;
pub type SwimTimeJson = Json<entity::swim_time::Model>;
pub type PopulatedSwimTimeJson = Json<entity::records::PopulatedSwimTime>;