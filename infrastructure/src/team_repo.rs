use crate::error::RepoError;
use entity::models::Team;
use entity::team::{ActiveModel, Column, Entity, Model};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, QueryFilter, Set, DeleteResult};
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

    pub async fn update_one(&self, id: i32, model: Team) -> Result<Model, RepoError> {
        ActiveModel {
            id: Set(id),
            ..model.into_active_model()
        }
        .update(&self.0)
        .await
        .map_err(RepoError::DbErr)
    }

    pub async fn delete_one_by_id(&self, id: i32) -> Result<DeleteResult, RepoError> {
        Entity::delete_by_id(id)
            .exec(&self.0)
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
