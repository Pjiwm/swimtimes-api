use async_graphql::{Context, Object, Result};
use repository::team_repo::TeamRepo;
use entity::team::Model;

#[derive(Default)]
pub struct TeamQuery;

#[Object]
impl TeamQuery {
    async fn get_teams_by_name(&self, ctx: &Context<'_>, name: String) -> Result<Vec<Model>> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.find_many_by_name(&name)
            .await
            .map_err(Into::into)
            .map(|x| x.into_iter().map(Into::into).collect())
    }

    async fn get_team_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<Model> {
        let repo = ctx.data::<TeamRepo>()?;
        repo.find_one_by_id(id)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }
}
