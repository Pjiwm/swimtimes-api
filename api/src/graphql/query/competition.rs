use crate::graphql::types::PopulatedCompetitionJson;
use async_graphql::{Context, Object, Result};
use repository::competition_repo::CompetitionRepo;

#[derive(Default)]
pub struct CompetitionQuery;

#[Object]
impl CompetitionQuery {
    async fn get_competition_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<PopulatedCompetitionJson> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_one_by_id_populated(id)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into())
    }

    async fn get_competitions_by_name_populated(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<Vec<PopulatedCompetitionJson>> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_many_by_name_populated(&name)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into_iter().map(|x| x.into()).collect())
    }

    async fn get_competitions_by_name(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<Vec<PopulatedCompetitionJson>> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_many_by_name_populated(&name)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into_iter().map(|x| x.into()).collect())
    }

    async fn get_competition_by_id_populated(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<PopulatedCompetitionJson> {
        let repo = ctx.data::<CompetitionRepo>()?;
        repo.find_one_by_id_populated(id)
            .await
            .map_err(|e| e.into())
            .map(|x| x.into())
    }
}
