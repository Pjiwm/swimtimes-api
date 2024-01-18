use crate::result::{map_find, RepoError};
use entity::records::{PopulatedSwimTime, SwimTime};
use entity::{competition, swim_time, swimmer};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DeleteResult, ModelTrait, Set};
use sea_orm::{EntityTrait, IntoActiveModel};

pub struct SwimTimeRepo(DatabaseConnection);

impl SwimTimeRepo {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        SwimTimeRepo(db_conn)
    }

    pub async fn insert_one(&self, model: SwimTime) -> Result<swim_time::Model, RepoError> {
        let active_model = model.into_active_model();
        active_model.insert(&self.0).await.map_err(RepoError::DbErr)
    }

    pub async fn find_one_by_id(&self, id: i32) -> Result<swim_time::Model, RepoError> {
        let result = swim_time::Entity::find_by_id(id).one(&self.0).await;
        map_find(result)
    }

    pub async fn find_one_by_id_populated(&self, id: i32) -> Result<PopulatedSwimTime, RepoError> {
        let swim_time = self.find_one_by_id(id).await?;
        let result = competition::Entity::find_by_id(swim_time.competition)
            .one(&self.0)
            .await;
        let competition = map_find(result)?;

        let result = swimmer::Entity::find_by_id(swim_time.swimmer)
            .one(&self.0)
            .await;
        let swimmer = map_find(result)?;

        Ok(PopulatedSwimTime(swim_time, competition, swimmer))
    }

    pub async fn find_many_by_competition(
        &self,
        comp_id: i32,
    ) -> Result<Vec<swim_time::Model>, RepoError> {
        let result = competition::Entity::find_by_id(comp_id).one(&self.0).await;
        let competition = map_find(result)?;

        competition
            .find_related(swim_time::Entity)
            .all(&self.0)
            .await
            .map_err(RepoError::DbErr)
    }

    pub async fn find_many_by_competition_populated(
        &self,
        comp_id: i32,
    ) -> Result<Vec<PopulatedSwimTime>, RepoError> {
        let swim_times = self.find_many_by_competition(comp_id).await?;

        let mut populated_buffer = Vec::new();
        for st in swim_times {
            let result = competition::Entity::find_by_id(st.competition)
                .one(&self.0)
                .await;
            let competition = match result {
                Ok(Some(t)) => t,
                Ok(None) => return Err(RepoError::ItemNotFound),
                Err(e) => return Err(RepoError::DbErr(e)),
            };

            let result = swimmer::Entity::find_by_id(st.swimmer).one(&self.0).await;
            let swimmer = match result {
                Ok(Some(t)) => t,
                Ok(None) => return Err(RepoError::ItemNotFound),
                Err(e) => return Err(RepoError::DbErr(e)),
            };
            populated_buffer.push(PopulatedSwimTime(st, competition, swimmer))
        }
        Ok(populated_buffer)
    }

    pub async fn update_one(
        &self,
        id: i32,
        model: SwimTime,
    ) -> Result<swim_time::Model, RepoError> {
        swim_time::ActiveModel {
            id: Set(id),
            ..model.into_active_model()
        }
        .update(&self.0)
        .await
        .map_err(RepoError::DbErr)
    }

    pub async fn delete_one_by_id(&self, id: i32) -> Result<DeleteResult, RepoError> {
        swim_time::Entity::delete_by_id(id)
            .exec(&self.0)
            .await
            .map_err(RepoError::DbErr)
    }
}
