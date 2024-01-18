use crate::error::RepoError;
use entity::records::{Competition, PopulatedCompetition};
use entity::{competition, team};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, QueryFilter, Set, DeleteResult};
use sea_orm::{EntityTrait, IntoActiveModel};
use tokio_stream::StreamExt;

pub struct CompetitionRepo(DatabaseConnection);

impl CompetitionRepo {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        CompetitionRepo(db_conn)
    }

    pub async fn insert_one(&self, model: Competition) -> Result<competition::Model, RepoError> {
        let active_model = model.into_active_model();
        active_model.insert(&self.0).await.map_err(RepoError::DbErr)
    }

    pub async fn find_one_by_id(&self, id: i32) -> Result<PopulatedCompetition, RepoError> {
        let result = competition::Entity::find_by_id(id).one(&self.0).await;
        let competition = map_find(result)?;
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

        let competition_populator: Vec<_> = tokio_stream::iter(competitions)
            .map(|c| async move {
                let team = team::Entity::find_by_id(c.host).one(&self.0).await;
                match team {
                    Ok(Some(t)) => Ok(PopulatedCompetition(c, t)),
                    Ok(None) => Err(RepoError::ItemNotFound),
                    Err(e) => Err(RepoError::DbErr(e)),
                }
            })
            .collect()
            .await;

        // competition populator is a vector of futures, so we need to await them all
        futures::future::join_all(competition_populator)
            .await
            .into_iter()
            .collect()
    }

    pub async fn update_one(&self, id: i32, model: Competition) -> Result<competition::Model, RepoError> {
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

fn map_find<T>(result: Result<Option<T>, DbErr>) -> Result<T, RepoError> {
    match result {
        Ok(Some(model)) => Ok(model),
        Ok(None) => Err(RepoError::ItemNotFound),
        Err(e) => Err(RepoError::DbErr(e)),
    }
}