use axum_server::tls_rustls::RustlsConfig;
use axum::{Router, routing::get};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Home" }));

    let tls_config = RustlsConfig::from_pem_file("cert/cert.pem", "cert/key.pem")
        .await.unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service())
        .await.unwrap();
}