use sea_orm::DeriveIntoActiveModel;
use crate::team::ActiveModel;

#[derive(DeriveIntoActiveModel, Clone, Debug, PartialEq, Eq, async_graphql::InputObject)]
pub struct Team {
    pub name: String,
    pub founding_date: chrono::NaiveDate,
    pub address: String,
    pub zip_code: String,
}