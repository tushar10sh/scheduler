use std::process::exit;
use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::Pool;

pub type DbPool = Pool<Postgres>;

pub async fn sqlx_init_db(db_conn_str: &str) -> Result<DbPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_conn_str)
        .await;;
    match pool{
        Ok(pool) => Ok(pool),
        Err(e) => {
            println!("Unable to create db pool.");
            Err(e)
        }
    }
}

pub async fn seaorm_init_db(db_conn_str: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let mut connection_options = ConnectOptions::new(db_conn_str.to_string().to_owned());
    connection_options
        .max_connections(5)
        .connect_timeout(Duration::from_millis(500))
        .acquire_timeout(Duration::from_millis(500))
        .idle_timeout(Duration::from_millis(500))
        .sqlx_logging(true);

    let db: Result<DatabaseConnection, sea_orm::DbErr> = Database::connect(connection_options).await;
    match db{
        Ok(db) => Ok(db),
        Err(e) => {
            println!("Unable to create db connection.");
            Err(e)
        }
    }
}

#[cfg(test)]
#[path ="../../tests/models/db.rs"]
mod tests;