use api::{run_server, ServerSettings};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let db: DatabaseConnection = Database::connect(database_url).await?;
    Migrator::up(&db, None).await?;
    let server_settings = ServerSettings {
        port: 3000,
        db_connection: db,
    };
    run_server(server_settings).await;

    Ok(())
}
