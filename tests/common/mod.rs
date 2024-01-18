use entity::team::Entity as TeamEntity;
use entity::competition::Entity as CompetitionEntity;
use migration::TableCreateStatement;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbConn, Schema, DatabaseConnection};
pub async fn setup() -> DatabaseConnection {
    // Connecting SQLite
    let db = Database::connect("sqlite::memory:").await.unwrap();

    // Setup database schema
    setup_schema(&db).await;

    db
}

async fn setup_schema(db: &DbConn) {
    // Setup Schema helper
    let schema = Schema::new(DbBackend::Sqlite);

    // Derive from Entity
    let stmt: TableCreateStatement = schema.create_table_from_entity(TeamEntity);

    // Execute create table statement
    let _ = db.execute(db.get_database_backend().build(&stmt)).await;

    let stmt: TableCreateStatement = schema.create_table_from_entity(CompetitionEntity);

    // Execute create table statement
    let _ = db.execute(db.get_database_backend().build(&stmt)).await;
}
