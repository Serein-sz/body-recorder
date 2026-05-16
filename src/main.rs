mod cli;
mod commands;
mod config;
mod error;
mod models;
mod output;
mod repository;
mod schema;
mod stats;
mod supabase;
mod use_cases;
mod validation;

#[tokio::main]
async fn main() {
    if let Err(error) = commands::run().await {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
