use crate::error::RepoError;
// use entity::models::Team;
use entity::models::Team;
use entity::team::{Column, Entity, Model};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, QueryFilter};
use sea_orm::{EntityTrait, IntoActiveModel};

pub struct TeamRepo(DatabaseConnection);

impl TeamRepo {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        TeamRepo(db_conn)
    }

    pub async fn insert_one(&self, model: Team) -> Result<Model, RepoError> {
        let active_model = model.into_active_model();
        active_model.insert(&self.0).await.map_err(RepoError::DbErr)
    }

    pub async fn find_one(&self, id: i32) -> Result<Model, RepoError> {
        let result = Entity::find_by_id(id).one(&self.0).await;
        map_find(result)
    }

    pub async fn find_many_by_name(&self, name: &str) -> Result<Vec<Model>, RepoError> {
        Entity::find()
            .filter(Column::Name.contains(name))
            .all(&self.0)
            .await
            .map_err(RepoError::DbErr)
    }
}

fn map_find<T>(result: Result<Option<T>, DbErr>) -> Result<T, RepoError> {
    match result {
        Ok(Some(model)) => Ok(model),
        Ok(None) => Err(RepoError::ItemNotFound),
        Err(e) => Err(RepoError::DbErr(e)),
    }
}
