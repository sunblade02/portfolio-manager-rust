use dotenvy::dotenv;
use portfolio_manager::log;

#[tokio::main]
async fn main() {
    dotenv().ok();

    log::start_log_flusher().await;
}