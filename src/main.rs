use crate::validation::ValidationApp;
use crate::proto::validationsvc::validation_server::ValidationServer;
use ed25519::pkcs8::DecodePrivateKey;
use ed25519_dalek::SigningKey;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::signal::unix::{signal, SignalKind};
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tower_http::trace;
use tracing::Level;

mod validation;
mod config;
mod proto;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let opt = envy::from_env::<config::Options>()?;
    let keys = envy::from_env::<config::Keys>()?;
    let signing_key = SigningKey::read_pkcs8_pem_file(&keys.signing_key)?;

    // bind server socket
    let addr = SocketAddr::new(opt.ip, opt.port);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("starting server on {}", addr);

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        // configure the server
        .timeout(std::time::Duration::from_secs(10))
        .layer(
            trace::TraceLayer::new_for_grpc()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        // enable grpc reflection
        .add_service(reflection)
        .add_service(ValidationServer::new(ValidationApp::new(signing_key)))
        // serve
        .serve_with_incoming_shutdown(TcpListenerStream::new(listener), async {
            let _ = signal(SignalKind::terminate()).unwrap().recv().await;
            tracing::info!("shutting down");
        })
        .await?;

    Ok(())
}
