mod common;
use entity::records::{Team, Competition};
use infrastructure::team_repo::TeamRepo;
use infrastructure::competition_repo::CompetitionRepo;

#[tokio::test]
async fn find_new_team() {
    // using common code.
    let db = common::setup().await;
    let repo = TeamRepo::new(db);

    let new_team = Team {
        name: "Team NL".to_owned(),
        founding_date: chrono::NaiveDate::from_ymd_opt(1999, 9, 20).unwrap(),
        address: "Address 1".to_owned(),
        zip_code: "Zip Code 1".to_owned(),
    };
    repo.insert_one(new_team).await.unwrap();

    let found_team = repo.find_one_by_id(1).await;
    assert!(found_team.is_ok());
    assert_eq!(found_team.unwrap().name, "Team NL".to_owned());
}

#[tokio::test]
async fn find_multiple_teams() {
    // using common code.
    let db = common::setup().await;
    let repo = TeamRepo::new(db);

    let mut team = Team {
        name: "Team".to_owned(),
        founding_date: chrono::NaiveDate::from_ymd_opt(1999, 9, 20).unwrap(),
        address: "Address 1".to_owned(),
        zip_code: "Zip Code 1".to_owned(),
    };
    for i in 1..10 {
        // team.name.push_str(&i.to_string());
        team.name = String::from("Team") + &i.to_string();
        repo.insert_one(team.clone()).await.unwrap();
    }

    let found_teams = repo.find_many_by_name("Team").await;
    assert!(found_teams.is_ok());
    assert_eq!(found_teams.as_ref().unwrap().len(), 9);
    assert_eq!(found_teams.unwrap()[3].name.as_ref(), "Team4".to_owned());
}

#[tokio::test]
async fn update_team() {
    let db = common::setup().await;
    let repo = TeamRepo::new(db);

    let new_team = Team {
        name: "Team NL".to_owned(),
        founding_date: chrono::NaiveDate::from_ymd_opt(1999, 9, 20).unwrap(),
        address: "Address 1".to_owned(),
        zip_code: "Zip Code 1".to_owned(),
    };
    repo.insert_one(new_team).await.unwrap();

    let found_team = repo.find_one_by_id(1).await.unwrap();
    assert_eq!(found_team.name, "Team NL".to_owned());

    let updated_team = Team {
        name: "Team NL Updated".to_owned(),
        founding_date: chrono::NaiveDate::from_ymd_opt(1999, 9, 20).unwrap(),
        address: "Address 1".to_owned(),
        zip_code: "Zip Code 1".to_owned(),
    };
    repo.update_one(1, updated_team).await.unwrap();

    let found_team = repo.find_one_by_id(1).await.unwrap();
    assert_eq!(found_team.name, "Team NL Updated".to_owned());
}

#[tokio::test]
async fn delete_team() {
    let db = common::setup().await;
    let repo = TeamRepo::new(db);

    let new_team = Team {
        name: "Team NL".to_owned(),
        founding_date: chrono::NaiveDate::from_ymd_opt(1999, 9, 20).unwrap(),
        address: "Address 1".to_owned(),
        zip_code: "Zip Code 1".to_owned(),
    };
    repo.insert_one(new_team).await.unwrap();

    let found_team = repo.find_one_by_id(1).await.unwrap();
    assert_eq!(found_team.name, "Team NL".to_owned());

    let delete_result = repo.delete_one_by_id(1).await.unwrap();
    assert_eq!(delete_result.rows_affected, 1);

    let found_team = repo.find_one_by_id(1).await;
    assert!(found_team.is_err());
}

#[tokio::test]
async fn find_new_competition() {
    let db = common::setup().await;
    let team_repo = TeamRepo::new(db.clone());
    let competition_repo = CompetitionRepo::new(db);

    let new_team = Team {
        name: "Team NL".to_owned(),
        founding_date: chrono::NaiveDate::from_ymd_opt(1999, 9, 20).unwrap(),
        address: "Address 1".to_owned(),
        zip_code: "Zip Code 1".to_owned(),
    };
    let new_team = team_repo.insert_one(new_team).await.unwrap();

    let new_competition = Competition {
        name: "Competition 1".to_owned(),
        date: chrono::NaiveDate::from_ymd_opt(1999, 9, 20).unwrap(),
        location: "Location 1".to_owned(),
        r#type: "Type 1".to_owned(),
        host: 1,
    };
    competition_repo.insert_one(new_competition).await.unwrap();

    let found_competition = competition_repo.find_one_by_id(1).await;
    assert!(found_competition.is_ok());
    // assert_eq!(found_team.unwrap().name, "Team NL".to_owned());
    assert_eq!(found_competition.as_ref().unwrap().0.name, "Competition 1".to_owned());
    assert_eq!(new_team.name, found_competition.unwrap().1.name);

}