use async_graphql::{Context, Object, Result};
use entity::models::Team;
use infrastructure::team_repo::TeamRepo;
use crate::graphql::types::TeamJson;

#[derive(Default)]
pub struct TeamMutation;

#[Object]
impl TeamMutation {
    pub async fn create_team(&self, ctx: &Context<'_>, input: Team) -> Result<TeamJson> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.insert_one(input.into())
            .await
            .map_err(|e| e.into())
            .map(|x| x.into())
    }
}
