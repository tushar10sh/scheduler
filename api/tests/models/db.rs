use crate::models::db::{sqlx_init_db, seaorm_init_db};

#[tokio::test]
async fn models_db_sqlx_init_db() {
    // Using sqlx 
    let db_conn_str = "postgres://postgres:postgres@localhost:15432/postgres";
    let pool = sqlx_init_db(db_conn_str).await.map_err(|e| {
        assert!(false, "TEST FAILED: SQLX Database pool creation failed.");
    });
    if let Ok(pool) = pool {
        println!("SQLX Database pool was created.");
    }
}

#[tokio::test]
async fn models_db_seaorm_init_db() {
    let db_conn_str = "postgres://postgres:postgres@localhost:15432/postgres";
    let db = seaorm_init_db(db_conn_str).await.map_err(|e| {
        assert!(false, "TEST FAILED: SEAORM Database connection failed.");
    });
    if let Ok(db) = db {
        println!("SEAORM Database connection was created.");
    }
}