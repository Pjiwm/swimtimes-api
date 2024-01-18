use crate::{competition, team, swimmer};
use serde::{ser::SerializeStruct, Serialize, Serializer};

pub use team_mod::Team;
mod team_mod {
    use crate::team::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    #[derive(DeriveIntoActiveModel, Clone, Debug, PartialEq, Eq, async_graphql::InputObject)]
    pub struct Team {
        pub name: String,
        pub founding_date: chrono::NaiveDate,
        pub address: String,
        pub zip_code: String,
    }
}

pub use swimmer_mod::Swimmer;
mod swimmer_mod {
    use crate::swimmer::ActiveModel;
    use sea_orm::DeriveIntoActiveModel;
    #[derive(DeriveIntoActiveModel, Clone, Debug, PartialEq, Eq, async_graphql::InputObject)]
    pub struct Swimmer {
        pub name: String,
        pub date_of_birth: chrono::NaiveDate,
        pub team: i32,
    }
}

pub use competition_mod::Competition;
mod competition_mod {
    use crate::competition::ActiveModel;
    use chrono::naive::NaiveDate;
    use sea_orm::DeriveIntoActiveModel;

    #[derive(DeriveIntoActiveModel, Clone, Debug, PartialEq, Eq, async_graphql::InputObject)]
    pub struct Competition {
        pub name: String,
        pub date: NaiveDate,
        pub location: String,
        pub r#type: String,
        pub host: i32,
    }
}

pub struct PopulatedCompetition(pub competition::Model, pub team::Model);
impl Serialize for PopulatedCompetition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let competition = &self.0;
        let team = &self.1;
        let mut state = serializer.serialize_struct("Competition", 5)?;

        state.serialize_field("id", &competition.id)?;
        state.serialize_field("name", &competition.name)?;
        state.serialize_field("date", &competition.date)?;
        state.serialize_field("location", &competition.location)?;
        state.serialize_field("type", &competition.r#type)?;
        state.serialize_field("hostId", &competition.host)?;
        state.serialize_field("host", &team)?;
        state.end()
    }
}

pub struct PopulatedSwimmer(pub swimmer::Model, pub team::Model);
impl Serialize for PopulatedSwimmer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let swimmer = &self.0;
        let team = &self.1;
        let mut state = serializer.serialize_struct("Swimmer", 5)?;

        state.serialize_field("id", &swimmer.id)?;
        state.serialize_field("name", &swimmer.name)?;
        state.serialize_field("dateOfBirth", &swimmer.date_of_birth)?;
        state.serialize_field("teamId", &swimmer.team)?;
        state.serialize_field("team", &team)?;
        state.end()
    }
}
