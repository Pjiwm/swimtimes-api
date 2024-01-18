use crate::result::{map_find, RepoError};
use entity::records::{Competition, PopulatedCompetition};
use entity::{competition, team};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, QueryFilter, Set};
use sea_orm::{EntityTrait, IntoActiveModel};

pub struct CompetitionRepo(DatabaseConnection);

impl CompetitionRepo {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        CompetitionRepo(db_conn)
    }

    pub async fn insert_one(&self, model: Competition) -> Result<competition::Model, RepoError> {
        let active_model = model.into_active_model();
        active_model.insert(&self.0).await.map_err(RepoError::DbErr)
    }

    pub async fn find_one_by_id(&self, id: i32) -> Result<competition::Model, RepoError> {
        let result = competition::Entity::find_by_id(id).one(&self.0).await;
        map_find(result)
    }

    pub async fn find_one_by_id_populated(
        &self,
        id: i32,
    ) -> Result<PopulatedCompetition, RepoError> {
        let competition = self.find_one_by_id(id).await?;
        let result = team::Entity::find_by_id(competition.host)
            .one(&self.0)
            .await;
        let team = map_find(result)?;
        Ok(PopulatedCompetition(competition, team))
    }

    pub async fn find_many_by_name(
        &self,
        name: &str,
    ) -> Result<Vec<competition::Model>, RepoError> {
        competition::Entity::find()
            .filter(competition::Column::Name.contains(name))
            .all(&self.0)
            .await
            .map_err(RepoError::DbErr)
    }

    pub async fn find_many_by_name_populated(
        &self,
        name: &str,
    ) -> Result<Vec<PopulatedCompetition>, RepoError> {
        let competitions = self.find_many_by_name(name).await?;

        let mut populated_buffer = Vec::new();
        for c in competitions {
            let team = team::Entity::find_by_id(c.host).one(&self.0).await;
            match team {
                Ok(Some(t)) => populated_buffer.push(PopulatedCompetition(c, t)),
                Ok(None) => return Err(RepoError::ItemNotFound),
                Err(e) => return Err(RepoError::DbErr(e)),
            }
        }
        Ok(populated_buffer)
    }

    pub async fn update_one(
        &self,
        id: i32,
        model: Competition,
    ) -> Result<competition::Model, RepoError> {
        competition::ActiveModel {
            id: Set(id),
            ..model.into_active_model()
        }
        .update(&self.0)
        .await
        .map_err(RepoError::DbErr)
    }

    pub async fn delete_one_by_id(&self, id: i32) -> Result<DeleteResult, RepoError> {
        competition::Entity::delete_by_id(id)
            .exec(&self.0)
            .await
            .map_err(RepoError::DbErr)
    }
}
