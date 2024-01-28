use crate::graphql::types::CompetitionModel;
use async_graphql::{Context, Object, Result};
use entity::records::Competition;
use repository::competition_repo::CompetitionRepo;

#[derive(Default)]
pub struct CompetitionMutation;

#[Object]
impl CompetitionMutation {
    async fn create_competition(
        &self,
        ctx: &Context<'_>,
        input: Competition,
    ) -> Result<CompetitionModel> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.insert_one(input)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    async fn update_competition(
        &self,
        ctx: &Context<'_>,
        id: i32,
        input: Competition,
    ) -> Result<CompetitionModel> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.update_one(id, input)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    pub async fn delete_swimmer(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.delete_one_by_id(id)
            .await
            .map_err(Into::into)
            .map(|x| x.rows_affected == 1)
    }
}
