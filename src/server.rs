use std::{future::Future, net::TcpListener};

use axum::{routing::get, Router, Server};

use crate::{configuration, route};

/// # Errors
///
/// Returns `Err` if the operator fails to start.
pub async fn run(
    tcp_listener: TcpListener,
    shutdown_signal: impl Future<Output = ()>,
    _configuration: configuration::Configuration,
) -> crate::Result<()> {
    let socket_addr = tcp_listener.local_addr()?;

    let app = Router::new()
        .route("/health/liveness", get(route::health_liveness_get))
        .route("/health/readiness", get(route::health_readiness_get));

    let server = Server::from_tcp(tcp_listener)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal);

    tracing::info!(%socket_addr, "Server started");

    server.await?;

    tracing::info!("Server stopped");

    Ok(())
}
