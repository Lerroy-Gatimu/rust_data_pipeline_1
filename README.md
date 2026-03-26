# rust_data_pipeline_1

A simple, production-ready ETL data pipeline written in Rust using MySQL.

## Dependencies
1. tokio: async runtime. 
2. sqlx is fully async
   It gives type-safe queries, connection pooling, and compile-time checks 
3. serde + serde_json: to turn JSON into Rust structs.
4. anyhow: Turns every error into a single Result type so we can use ? everywhere.
5. dotenvy: Loads secrets from .env 
6. chrono: Handles timestamps 



## What it does
- **Extract** user data from `data/input.json`
- **Transform** names to uppercase + add timestamp
- **Load** into MySQL table `processed_users` (idempotent)


## Quick Start
1. `cargo new rust_data_pipeline_1 --bin && cd rust_data_pipeline_1`
2. Copy the `Cargo.toml`, `.env`, `data/input.json`, and `src/main.rs` from this repo.
3. Update `DATABASE_URL` in `.env`
4. `cargo run`


## How to Extend
- Replace JSON with a CSV (`csv` crate)
- Fetch from an API (`reqwest` + `tokio`)
- Add batch inserts with transactions for 100k+ rows
- Add `clap` for CLI arguments
- Use sqlx macros + `sqlx-cli` for full compile-time query checking

## License
MIT 

