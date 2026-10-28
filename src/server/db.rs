use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr, TransactionTrait,
};
use sea_orm_migration::MigratorTrait;

use super::migrations::Migrator;

const MAX_CONNECTIONS: u32 = 10;
const MIGRATION_LOCK_KEY: i64 = 7_401_220_001;

pub async fn connect(url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(url);
    options.max_connections(MAX_CONNECTIONS).sqlx_logging(false);
    let db = Database::connect(options).await?;
    migrate(&db).await?;
    Ok(db)
}

async fn migrate(db: &DatabaseConnection) -> Result<(), DbErr> {
    let txn = db.begin().await?;
    txn.execute_unprepared(&format!(
        "SELECT pg_advisory_xact_lock({MIGRATION_LOCK_KEY})"
    ))
    .await?;
    Migrator::up(&txn, None).await?;
    txn.commit().await
}

#[cfg(test)]
#[sqlx::test(migrations = false)]
async fn concurrent_migrations_both_succeed(pool: sqlx::PgPool) {
    let db = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    let (first, second) = tokio::join!(migrate(&db), migrate(&db));
    first.unwrap();
    second.unwrap();
}

#[cfg(test)]
pub async fn test_db(pool: sqlx::PgPool) -> DatabaseConnection {
    let db = sea_orm::SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    migrate(&db).await.unwrap();
    db
}
