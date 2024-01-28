use async_graphql::types::Json;

pub type PopulatedCompetitionJson = Json<entity::records::PopulatedCompetition>;
pub type PopulatedSwimmerJson = Json<entity::records::PopulatedSwimmer>;
pub type PopulatedSwimTimeJson = Json<entity::records::PopulatedSwimTime>;
pub type SwimmerModel = entity::swimmer::Model;
pub type CompetitionModel = entity::competition::Model;
pub type SwimTimeModel = entity::swim_time::Model;
pub type TeamModel = entity::team::Model;
