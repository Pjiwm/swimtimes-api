use crate::graphql::types::TeamJson;
use async_graphql::{Context, Object, Result};
use repository::team_repo::TeamRepo;

#[derive(Default)]
pub struct TeamQuery;

#[Object]
impl TeamQuery {
    async fn get_teams_by_name(&self, ctx: &Context<'_>, name: String) -> Result<Vec<TeamJson>> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.find_many_by_name(&name)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into_iter().map(|x| x.into()).collect())
    }

    async fn get_team_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<TeamJson> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.find_one_by_id(id)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into())
    }
}
