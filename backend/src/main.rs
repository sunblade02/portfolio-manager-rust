use axum_server::tls_rustls::RustlsConfig;
use dotenvy::dotenv;
use std::error::Error;
use std::net::SocketAddr;
use portfolio_manager::{db, log, router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    #[cfg(not(test))]
    log::setup_logging().await?;
    #[cfg(not(test))]
    log::start_log_flusher().await?;

    let pool = db::connect_db().await?;

    let app = router::create_app(router::AppState {
        db: pool,
    });

    let tls_config = RustlsConfig::from_pem_file("cert/cert.pem", "cert/key.pem")
        .await.unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}