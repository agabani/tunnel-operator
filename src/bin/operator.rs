use tokio::net::TcpListener;
use tunnel_operator::{configuration, server, shutdown};

#[tokio::main]
async fn main() -> tunnel_operator::Result<()> {
    tracing_subscriber::fmt::try_init()?;

    let configuration = configuration::load(&[])?;

    let tcp_listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.http_server.host, configuration.http_server.port
    ))
    .await?;

    server::run(tcp_listener.into_std()?, shutdown::recv(), configuration).await?;

    Ok(())
}
