use sqlx::{sqlite::SqlitePoolOptions, Row};
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Get the database URL from the environment variables
    let database_url = env::var("DATABASE_URL")?;

    // Create a connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create a table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // Insert data
    sqlx::query("INSERT INTO users (name, age) VALUES (?, ?)")
        .bind("Alice")
        .bind(30)
        .execute(&pool)
        .await?;

    sqlx::query("INSERT INTO users (name, age) VALUES (?, ?)")
        .bind("Bob")
        .bind(25)
        .execute(&pool)
        .await?;

    // Select data
    let rows = sqlx::query("SELECT id, name, age FROM users")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let age: i32 = row.get("age");
        println!("User: id={}, name={}, age={}", id, name, age);
    }

    // Update data
    sqlx::query("UPDATE users SET age = ? WHERE name = ?")
        .bind(31)
        .bind("Alice")
        .execute(&pool)
        .await?;

    // Delete data
    sqlx::query("DELETE FROM users WHERE name = ?")
        .bind("Bob")
        .execute(&pool)
        .await?;

    // Add an index
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_users_name ON users (name)")
        .execute(&pool)
        .await?;

    // Drop the table
    // sqlx::query("DROP TABLE IF EXISTS users")
    //     .execute(&pool)
    //     .await?;

    Ok(())
}