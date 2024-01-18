use crate::result::{map_find, RepoError};
use entity::records::{PopulatedSwimmer, Swimmer};
use entity::{swimmer, team};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, QueryFilter, Set};
use sea_orm::{EntityTrait, IntoActiveModel};

pub struct SwimmerRepo(DatabaseConnection);

impl SwimmerRepo {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        SwimmerRepo(db_conn)
    }

    pub async fn insert_one(&self, model: Swimmer) -> Result<swimmer::Model, RepoError> {
        let active_model = model.into_active_model();
        active_model.insert(&self.0).await.map_err(RepoError::DbErr)
    }

    pub async fn find_one_by_id(&self, id: i32) -> Result<swimmer::Model, RepoError> {
        let result = swimmer::Entity::find_by_id(id).one(&self.0).await;
        map_find(result)
    }

    pub async fn find_one_by_id_populated(&self, id: i32) -> Result<PopulatedSwimmer, RepoError> {
        let swimmer = self.find_one_by_id(id).await?;
        let result = team::Entity::find_by_id(swimmer.team).one(&self.0).await;
        let team = map_find(result)?;
        Ok(PopulatedSwimmer(swimmer, team))
    }

    pub async fn find_many_by_name(&self, name: &str) -> Result<Vec<swimmer::Model>, RepoError> {
        swimmer::Entity::find()
            .filter(swimmer::Column::Name.contains(name))
            .all(&self.0)
            .await
            .map_err(RepoError::DbErr)
    }

    pub async fn find_many_by_name_populated(
        &self,
        name: &str,
    ) -> Result<Vec<PopulatedSwimmer>, RepoError> {
        let swimmers = self.find_many_by_name(name).await?;

        let mut populated_buffer = Vec::new();
        for s in swimmers {
            let team = team::Entity::find_by_id(s.team).one(&self.0).await;
            match team {
                Ok(Some(t)) => populated_buffer.push(PopulatedSwimmer(s, t)),
                Ok(None) => return Err(RepoError::ItemNotFound),
                Err(e) => return Err(RepoError::DbErr(e)),
            }
        }
        Ok(populated_buffer)
    }

    pub async fn update_one(&self, id: i32, model: Swimmer) -> Result<swimmer::Model, RepoError> {
        swimmer::ActiveModel {
            id: Set(id),
            ..model.into_active_model()
        }
        .update(&self.0)
        .await
        .map_err(RepoError::DbErr)
    }

    pub async fn delete_one_by_id(&self, id: i32) -> Result<DeleteResult, RepoError> {
        swimmer::Entity::delete_by_id(id)
            .exec(&self.0)
            .await
            .map_err(RepoError::DbErr)
    }
}
