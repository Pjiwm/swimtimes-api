use async_graphql::types::Json;
pub type TeamJson = Json<entity::team::Model>;
pub type CompetitionJson = Json<entity::records::PopulatedCompetition>;