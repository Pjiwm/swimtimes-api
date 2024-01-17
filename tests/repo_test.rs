mod common;
use entity::records::Team;
use infrastructure::team_repo::TeamRepo;

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

    let found_team = repo.find_one(1).await;
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
