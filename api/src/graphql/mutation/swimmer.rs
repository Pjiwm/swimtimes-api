use crate::graphql::json_types::SwimmerJson;
use async_graphql::{Context, Object, Result};
use entity::records::Swimmer;
use repository::swimmer_repo::SwimmerRepo;

#[derive(Default)]
pub struct SwimmerMutation;

#[Object]
impl SwimmerMutation {
    pub async fn create_swimmer(
        &self,
        ctx: &Context<'_>,
        input: Swimmer,
    ) -> Result<SwimmerJson> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.insert_one(input)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    pub async fn update_swimmer(
        &self,
        ctx: &Context<'_>,
        id: i32,
        input: Swimmer,
    ) -> Result<SwimmerJson> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.update_one(id, input)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    pub async fn delete_swimmer(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let repo = ctx.data::<SwimmerRepo>()?;
        repo.delete_one_by_id(id)
            .await
            .map_err(Into::into)
            .map(|x| x.rows_affected == 1)
    }
}
