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
            RepoError::DbErr(err) => self.parse_db_err(err),
            RepoError::ItemNotFound => "Item not found".to_string(),
            RepoError::SpecifiedError(msg) => msg.to_string(),
        }
    }

    fn parse_db_err(&self, err: &DbErr) -> String {
        match err {
            DbErr::Exec(_) => "Execution error".to_string(),
            DbErr::Query(_) => "Query error".to_string(),
            DbErr::Type(_) => "Type error".to_string(),
            DbErr::Json(err) => format!("Json error: {}", err),
            DbErr::RecordNotUpdated => "Record not found".to_string(),
            DbErr::RecordNotFound(_) => "Record not found".to_string(),
            _ => "an internal error occured".to_string(),
        }
    }
}
