use crate::graphql::json_types::PopulatedSwimmerJson;
use async_graphql::{Context, Object, Result};
use entity::swimmer::Model as SwimmerModel;
use repository::swimmer_repo::SwimmerRepo;

#[derive(Default)]
pub struct SwimmerQuery;

#[Object]
impl SwimmerQuery {
    async fn get_swimmer_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<SwimmerModel> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.find_one_by_id(id).await.map_err(Into::into)
    }

    async fn get_swimmer_by_id_populated(
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

    async fn get_swimmers_by_name(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<Vec<SwimmerModel>> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.find_many_by_name(&name).await.map_err(Into::into)
    }

    async fn get_swimmers_by_name_populated(
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
