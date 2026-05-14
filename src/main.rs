mod cli;
mod commands;
mod config;
mod error;
mod models;
mod schema;
mod stats;
mod supabase;
mod validation;

#[tokio::main]
async fn main() {
    if let Err(error) = commands::run().await {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
