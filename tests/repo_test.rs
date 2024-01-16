mod common;
use infrastructure::team_repo::TeamRepo;
use entity::models::Team;

#[tokio::test]
async fn find_new_user() {
    // using common code.
    let db = common::setup().await;
    let repo = TeamRepo::new(db);

    let new_team = Team {
        name: "Team NL".to_owned(),
        founding_date: chrono::NaiveDate::from_ymd_opt(1999, 9, 20).unwrap(),
        address: "Address 1".to_owned(),
        zip_code: "Zip Code 1".to_owned(),
    };
    repo.insert(new_team).await.unwrap();

    let found_team = repo.find(1).await;
    assert!(found_team.is_ok());
    assert_eq!(found_team.unwrap().name, "Team NL".to_owned());
}
