use crate::graphql::json_types::SwimTimeJson;
use async_graphql::{Context, Object, Result};
use entity::records::SwimTime;
use repository::swim_time_repo::SwimTimeRepo;

#[derive(Default)]
pub struct SwimmerMutation;

#[Object]
impl SwimmerMutation {
    pub async fn create_swim_time(
        &self,
        ctx: &Context<'_>,
        input: SwimTime,
    ) -> Result<SwimTimeJson> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.insert_one(input)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    pub async fn update_swim_time(
        &self,
        ctx: &Context<'_>,
        id: i32,
        input: SwimTime,
    ) -> Result<SwimTimeJson> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.update_one(id, input)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    pub async fn delete_swim_time(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.delete_one_by_id(id)
            .await
            .map_err(Into::into)
            .map(|x| x.rows_affected == 1)
    }
}
