use crate::graphql::types::{PopulatedSwimmerJson, SwimmerJson};
use async_graphql::{Context, Object, Result};
use repository::swimmer_repo::SwimmerRepo;

#[derive(Default)]
pub struct SwimmerQuery;

#[Object]
impl SwimmerQuery {
    async fn get_competition_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<SwimmerJson> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.find_one_by_id(id)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    async fn get_competition_by_id_populated(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<PopulatedSwimmerJson> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.find_one_by_id_populated(id)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    async fn get_competitions_by_name(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<Vec<SwimmerJson>> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.find_many_by_name(&name)
            .await
            .map_err(Into::into)
            .map(|x| x.into_iter().map(Into::into).collect())
    }

    async fn get_competitions_by_name_populated(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<Vec<PopulatedSwimmerJson>> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.find_many_by_name_populated(&name)
            .await
            .map_err(Into::into)
            .map(|x| x.into_iter().map(Into::into).collect())
    }
}
