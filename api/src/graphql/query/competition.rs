use crate::graphql::json_types::PopulatedCompetitionJson;
use async_graphql::{Context, Object, Result};
use entity::competition::Model as CompetitionModel;
use repository::competition_repo::CompetitionRepo;

#[derive(Default)]
pub struct CompetitionQuery;

#[Object]
impl CompetitionQuery {
    async fn get_competition_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<CompetitionModel> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_one_by_id(id)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    async fn get_competition_by_id_populated(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<PopulatedCompetitionJson> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_one_by_id_populated(id)
            .await
            .map_err(Into::into)
            .map(Into::into)
    }

    async fn get_competitions_by_name(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<Vec<CompetitionModel>> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_many_by_name(&name)
            .await
            .map_err(Into::into)
            .map(|x| x.into_iter().map(Into::into).collect())
    }

    async fn get_competitions_by_name_populated(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<Vec<PopulatedCompetitionJson>> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_many_by_name_populated(&name)
            .await
            .map_err(Into::into)
            .map(|x| x.into_iter().map(Into::into).collect())
    }
}
