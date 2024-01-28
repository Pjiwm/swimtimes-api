use async_graphql::{CustomValidator, InputValueError};
use chrono::Datelike;

#[derive(Default)]
pub struct AgeValidator;

impl CustomValidator<chrono::NaiveDate> for AgeValidator {
    fn check(&self, value: &chrono::NaiveDate) -> Result<(), InputValueError<chrono::NaiveDate>> {
        let curr_year = chrono::Utc::now().year();
        if value.year() > curr_year - 110 && value.year() < curr_year - 5 {
            Ok(())
        } else {
            Err(InputValueError::custom(format!(
                "Date of birth must be between {} and {}",
                curr_year - 110,
                curr_year - 5
            )))
        }
    }
}

#[derive(Default)]
pub struct StrokeValidator;

impl CustomValidator<String> for StrokeValidator {
    fn check(&self, value: &String) -> Result<(), InputValueError<String>> {
        match value.as_str() {
            "freestyle" => Ok(()),
            "backstroke" => Ok(()),
            "breaststroke" => Ok(()),
            "butterfly" => Ok(()),
            "medley" => Ok(()),
            _ => Err(InputValueError::custom(format!(
                "invalid stroke: {}",
                value
            ))),
        }
    }
}

#[derive(Default)]
pub struct DistanceValidator;

impl CustomValidator<i32> for DistanceValidator {
    fn check(&self, value: &i32) -> Result<(), InputValueError<i32>> {
        if value % 25 == 0 {
            Ok(())
        } else {
            Err(InputValueError::custom(format!(
                "invalid distance: {}",
                value
            )))
        }
    }
}
