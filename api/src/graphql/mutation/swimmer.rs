use async_graphql::{Context, Object, Result};
use repository::swimmer_repo::SwimmerRepo;
use entity::{records::Swimmer, swimmer::Model as SwimmerModel};

#[derive(Default)]
pub struct SwimmerMutation;

#[Object]
impl SwimmerMutation {
    async fn create_swimmer(
        &self,
        ctx: &Context<'_>,
        input: Swimmer,
    ) -> Result<SwimmerModel> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.insert_one(input.into())
            .await
            .map_err(Into::into)
    }

    async fn update_swimmer(
        &self,
        ctx: &Context<'_>,
        id: i32,
        input: Swimmer,
    ) -> Result<SwimmerModel> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.update_one(id, input.into())
            .await
            .map_err(Into::into)
    }

    async fn delete_swimmer(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.delete_one_by_id(id)
            .await
            .map_err(Into::into)
            .map(|x| x.rows_affected == 1)
    }
}
