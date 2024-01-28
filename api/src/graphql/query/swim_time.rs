use crate::graphql::types::PopulatedSwimTimeJson;
use async_graphql::{Context, Object, Result};
use repository::swim_time_repo::SwimTimeRepo;
use crate::graphql::types::SwimTimeModel;

#[derive(Default)]
pub struct SwimTimeQuery;

#[Object]
impl SwimTimeQuery {
    async fn get_swim_time_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<SwimTimeModel> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.find_one_by_id(id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
            .map(Into::into)
    }

    async fn get_swim_time_by_id_populated(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<PopulatedSwimTimeJson> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.find_one_by_id_populated(id)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    async fn get_swim_times_by_competition(
        &self,
        ctx: &Context<'_>,
        competition_id: i32,
        index: u64
    ) -> Result<Vec<SwimTimeModel>> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.find_many_by_competition(competition_id, index)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))
            .map(|x| x.into_iter().map(Into::into).collect())
    }

    async fn get_swim_times_by_competition_populated(
        &self,
        ctx: &Context<'_>,
        competition_id: i32,
        index: u64,
    ) -> Result<Vec<PopulatedSwimTimeJson>> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.find_many_by_competition_populated(competition_id, index)
            .await
            .map_err(Into::into)
            .map(|x| x.into_iter().map(Into::into).collect())
    }
}
