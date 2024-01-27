use async_graphql::{Context, Object, Result};
use entity::records::SwimTime;
use entity::swim_time::Model as SwimTimeModel;
use repository::swim_time_repo::SwimTimeRepo;

#[derive(Default)]
pub struct SwimTimeMutation;

#[Object]
impl SwimTimeMutation {
    async fn create_swim_time(&self, ctx: &Context<'_>, input: SwimTime) -> Result<SwimTimeModel> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.insert_one(input.into()).await.map_err(Into::into)
    }

    async fn update_swim_time(
        &self,
        ctx: &Context<'_>,
        id: i32,
        input: SwimTime,
    ) -> Result<SwimTimeModel> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.update_one(id, input.into()).await.map_err(Into::into)
    }

    pub async fn delete_swim_time(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let repo = ctx.data::<SwimTimeRepo>()?;
        repo.delete_one_by_id(id)
            .await
            .map_err(Into::into)
            .map(|x| x.rows_affected == 1)
    }
}
