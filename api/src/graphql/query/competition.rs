use crate::graphql::types::{TeamJson, CompetitionJson};
use async_graphql::{Context, Object, Result};
use infrastructure::competition_repo::CompetitionRepo;
use serde::Serialize;

#[derive(Default)]
pub struct CompetitionQuery;

#[Object]
impl CompetitionQuery {
    async fn get_teams_by_name(&self, ctx: &Context<'_>, name: String) -> Result<Vec<CompetitionJson>> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_many_by_name_populated(&name)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into_iter().map(|x| x.into()).collect())
    }

    async fn get_team_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<CompetitionJson> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_one_by_id(id)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into())
    }
}
