use crate::graphql::types::CompetitionJson;
use async_graphql::{Context, Object, Result};
use entity::records::Competition;
use infrastructure::competition_repo::CompetitionRepo;

#[derive(Default)]
pub struct CompetitionMutation;

#[Object]
impl CompetitionMutation {
    pub async fn create_team(
        &self,
        ctx: &Context<'_>,
        input: Competition,
    ) -> Result<CompetitionJson> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.insert_one(input)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into())
    }

    pub async fn update_team(
        &self,
        ctx: &Context<'_>,
        id: i32,
        input: Competition,
    ) -> Result<CompetitionJson> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.update_one(id, input)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into())
    }

    pub async fn delete_team(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.delete_one_by_id(id)
            .await
            .map_err(|e| e.into())
            .map(|x| x.rows_affected == 1)
    }
}
