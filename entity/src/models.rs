use sea_orm::DeriveIntoActiveModel;
use crate::team::ActiveModel;

#[derive(DeriveIntoActiveModel)]
pub struct Team {
    pub name: String,
    pub founding_date: chrono::NaiveDate,
    pub address: String,
    pub zip_code: String,
}