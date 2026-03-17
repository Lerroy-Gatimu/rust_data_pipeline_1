Why each dependency?
tokio: Rust's most popular async runtime. sqlx is fully async, so we need it.
sqlx: The best choice for Rust + MySQL in 2026. It gives type-safe queries, connection pooling, and compile-time checks (when you use the macros feature). We use the bind version here for maximum beginner-friendliness.
serde + serde_json: Standard way to turn JSON into Rust structs.
anyhow: Turns every error into a single Result type so we can use ? everywhere.
dotenvy: Loads secrets from .env (never hard-code passwords).
chrono: Handles timestamps that MySQL understands perfectly.

# rust_data_pipeline_1

A simple, production-ready ETL data pipeline written in Rust using MySQL.

## What it does
- **Extract** user data from `data/input.json`
- **Transform** names to uppercase + add timestamp
- **Load** into MySQL table `processed_users` (idempotent)

## Prerequisites
- Rust 1.80+
- MySQL 8.x running
- Database `rust_data_pipeline_1` created

## Quick Start
1. `cargo new rust_data_pipeline_1 --bin && cd rust_data_pipeline_1`
2. Copy the `Cargo.toml`, `.env`, `data/input.json`, and `src/main.rs` from this guide.
3. Update `DATABASE_URL` in `.env`
4. `cargo run`

## Project Structure


## How to Extend
- Replace JSON with a CSV (`csv` crate)
- Fetch from an API (`reqwest` + `tokio`)
- Add batch inserts with transactions for 100k+ rows
- Add `clap` for CLI arguments
- Use sqlx macros + `sqlx-cli` for full compile-time query checking

## License
MIT 

