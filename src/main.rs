use api::ServerSettings;
use migration::{Migrator, MigratorTrait};
use sea_orm::SqlxPostgresConnector;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    set_secrets(secrets);
    match env_logger::try_init() {
        Ok(_) => println!("Logger initialized"),
        Err(_) => println!("Logger already initialized"),
    }

    let db = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    Migrator::up(&db, None).await.unwrap();
    let server_settings = ServerSettings { db_connection: db };
    println!("Successfully connected to database");
    let app = api::server(&server_settings).await;

    Ok(app.into())
}

fn set_secrets(secrets: shuttle_secrets::SecretStore) {
    secrets.into_iter().for_each(|(k, v)| {
        std::env::set_var(k, v);
    });
}
