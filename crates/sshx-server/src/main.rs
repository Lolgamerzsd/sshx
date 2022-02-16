use sshx_server::make_server;
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "[::1]:8051".parse()?;

    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    tracing::info!("server listening at {addr}");
    make_server(&addr, async {
        tokio::select! {
            _ = sigterm.recv() => (),
            _ = sigint.recv() => (),
        }
        tracing::info!("gracefully shutting down...");
    })
    .await?;
    Ok(())
}
