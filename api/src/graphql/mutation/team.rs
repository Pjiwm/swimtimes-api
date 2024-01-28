use crate::graphql::types::TeamModel;
use async_graphql::{Context, Object, Result};
use entity::records::Team;
use repository::team_repo::TeamRepo;

#[derive(Default)]
pub struct TeamMutation;

#[Object]
impl TeamMutation {
    async fn create_team(&self, ctx: &Context<'_>, input: Team) -> Result<TeamModel> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.insert_one(input.into()).await.map_err(Into::into)
    }

    async fn update_team(&self, ctx: &Context<'_>, id: i32, input: Team) -> Result<TeamModel> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.update_one(id, input.into()).await.map_err(Into::into)
    }

    pub async fn delete_team(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.delete_one_by_id(id)
            .await
            .map_err(Into::into)
            .map(|x| x.rows_affected == 1)
    }
}
