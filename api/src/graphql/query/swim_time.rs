use crate::graphql::json_types::{PopulatedSwimTimeJson, SwimTimeJson};
use async_graphql::{Context, Object, Result};
use repository::swim_time_repo::SwimTimeRepo;

#[derive(Default)]
pub struct SwimTimeQuery;

#[Object]
impl SwimTimeQuery {
    async fn get_swim_time_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<SwimTimeJson> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.find_one_by_id(id)
            .await
            .map_err(Into::into)
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
    ) -> Result<Vec<SwimTimeJson>> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.find_many_by_competition(competition_id)
            .await
            .map_err(Into::into)
            .map(|x| x.into_iter().map(Into::into).collect())
    }

    async fn get_swim_times_by_competition_populated(
        &self,
        ctx: &Context<'_>,
        competition_id: i32,
    ) -> Result<Vec<PopulatedSwimTimeJson>> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.find_many_by_competition_populated(competition_id)
            .await
            .map_err(Into::into)
            .map(|x| x.into_iter().map(Into::into).collect())
    }
}
