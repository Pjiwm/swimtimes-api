use std::error::Error;
use sea_orm::DbErr;

#[derive(Debug)]
pub enum RepoError {
    DbErr(DbErr),
    ItemNotFound,
    SpecifiedError(String),
}

impl std::fmt::Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg())
    }
}

impl Error for RepoError {}

impl RepoError {
    fn msg(&self) -> String {
        match self {
            RepoError::DbErr(err) => err.to_string(),
            RepoError::ItemNotFound => "Item not found".to_string(),
            RepoError::SpecifiedError(msg) => msg.to_string(),
        }
    }
}

pub fn map_find<T>(result: Result<Option<T>, DbErr>) -> Result<T, RepoError> {
    match result {
        Ok(Some(model)) => Ok(model),
        Ok(None) => Err(RepoError::ItemNotFound),
        Err(e) => Err(RepoError::DbErr(e)),
    }
}
