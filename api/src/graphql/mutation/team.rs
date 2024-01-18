use crate::graphql::types::TeamJson;
use async_graphql::{Context, Object, Result};
use entity::records::Team;
use infrastructure::team_repo::TeamRepo;

#[derive(Default)]
pub struct TeamMutation;

#[Object]
impl TeamMutation {
    pub async fn create_team(&self, ctx: &Context<'_>, input: Team) -> Result<TeamJson> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.insert_one(input)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into())
    }

    pub async fn update_team(&self, ctx: &Context<'_>, id: i32, input: Team) -> Result<TeamJson> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.update_one(id, input)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into())
    }

    pub async fn delete_team(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.delete_one_by_id(id)
            .await
            .map_err(|e| e.into())
            .map(|x| x.rows_affected == 1)
    }
}
