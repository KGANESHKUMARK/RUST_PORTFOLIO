use sqlx::{postgres::PgPoolOptions, mysql::MySqlPoolOptions, sqlite::SqlitePoolOptions, Row};
use oracle::Connection as OracleConnection;
use odbc::*;
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // PostgreSQL
    if let Ok(database_url) = env::var("POSTGRES_DATABASE_URL") {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        let row = sqlx::query("SELECT version()")
            .fetch_one(&pool)
            .await?;
        let version: String = row.get(0);
        println!("PostgreSQL version: {}", version);
    }

    // MySQL
    if let Ok(database_url) = env::var("MYSQL_DATABASE_URL") {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        let row = sqlx::query("SELECT VERSION()")
            .fetch_one(&pool)
            .await?;
        let version: String = row.get(0);
        println!("MySQL version: {}", version);
    }

    // SQLite
    if let Ok(database_url) = env::var("SQLITE_DATABASE_URL") {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;
        let row = sqlx::query("SELECT sqlite_version()")
            .fetch_one(&pool)
            .await?;
        let version: String = row.get(0);
        println!("SQLite version: {}", version);
    }

    // Oracle
    if let (Ok(username), Ok(password), Ok(connect_string)) = (
        env::var("ORACLE_DB_USERNAME"),
        env::var("ORACLE_DB_PASSWORD"),
        env::var("ORACLE_DB_CONNECT_STRING"),
    ) {
        let conn = OracleConnection::connect(username, password, connect_string)?;
        let rows = conn.query("SELECT * FROM v$version", &[])?;
        for row in rows {
            let row = row?;
            let version: String = row.get(0)?;
            println!("Oracle version: {}", version);
        }
    }

    // Snowflake
    if let Ok(dsn) = env::var("SNOWFLAKE_DSN") {
        let env = Environment::new()?;
        let conn = env.connect_with_connection_string(&dsn)?;
        let stmt = Statement::with_parent(&conn)?;
        let cursor = stmt.exec_direct("SELECT CURRENT_VERSION()")?;
        if let Some(mut cursor) = cursor {
            while let Some(row) = cursor.fetch()? {
                let version: &str = row.get(1)?;
                println!("Snowflake version: {}", version);
            }
        }
    }

    Ok(())
}