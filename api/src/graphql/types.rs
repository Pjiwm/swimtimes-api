use async_graphql::types::Json;
pub type TeamJson = Json<entity::team::Model>;
pub type PopulatedCompetitionJson = Json<entity::records::PopulatedCompetition>;
pub type CompetitionJson = Json<entity::competition::Model>;