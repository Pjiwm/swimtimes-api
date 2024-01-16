use entity::models::Team;
use infrastructure::team_repo::TeamRepo;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL")?;
    let db: DatabaseConnection = Database::connect(database_url).await?;
    let team_repo = TeamRepo::new(db.clone());

    let bd = chrono::NaiveDate::from_ymd_opt(1999, 9, 20).unwrap();
    let model = Team {
        name: "Team 1".to_owned(),
        founding_date: bd,
        address: "Address 1".to_owned(),
        zip_code: "Zip Code 1".to_owned(),
    };
    match team_repo.insert(model).await {
        Ok(_) => println!("Successfully inserted team"),
        Err(e) => println!("Error inserting team: {}", e),
    };
    println!("Successfully connected to database");
    Migrator::up(&db, None).await?;
    Ok(())
}
