#[tokio::main]
async fn main() {
    if let Err(error) = body_recorder::presentation::cli::run().await {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
