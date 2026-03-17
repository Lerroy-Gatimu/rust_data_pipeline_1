use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{MySql, Pool};
use std::fs;

#[derive(Deserialize)]
struct RawUser {
    id: i32,
    name: String,
    email: String,
}

#[derive(Debug)]
struct ProcessedUser {
    id: i32,
    name: String,
    email: String,
    processed_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok(); 
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // Create connection pool 
    // A pool reuses connections instead of opening a new one every time.
    // Much faster and handles concurrent queries automatically.
    let pool: Pool<MySql> = sqlx::mysql::MySqlPool::connect(&database_url).await?;
    println!("✅ Connected to MySQL successfully!");

    // Ensure the target table exists (idempotent) 
    // use raw query + .execute because it's a DDL statement.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS processed_users (
            id INT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL,
            processed_at DATETIME NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;
    println!("✅ Table 'processed_users' is ready!");

    // Extract: Read JSON file
    let json_data = fs::read_to_string("data/input.json")?;
    let raw_users: Vec<RawUser> = serde_json::from_str(&json_data)?;
    println!("Extracted {} users from JSON.", raw_users.len());

    // Transform + Load (in one loop for simplicity)
    for raw in raw_users {
        // Transform step
        let processed_name = raw.name.to_uppercase();
        let processed_at = Utc::now(); 

        let user = ProcessedUser {
            id: raw.id,
            name: processed_name,
            email: raw.email,
            processed_at,
        };

        // Load step – using .bind prevents SQL injection
        // ON DUPLICATE KEY UPDATE makes the pipeline idempotent
        sqlx::query(
            r#"
            INSERT INTO processed_users (id, name, email, processed_at)
            VALUES (?, ?, ?, ?)
            ON DUPLICATE KEY UPDATE
                name = VALUES(name),
                email = VALUES(email),
                processed_at = VALUES(processed_at)
            "#,
        )
        .bind(user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(user.processed_at)   // sqlx + chrono feature handles this automatically
        .execute(&pool)
        .await?;

        println!("✅ Processed & inserted user ID {} → {}", user.id, user.name);
    }

    println!("Data pipeline completed successfully!");
    Ok(())
}