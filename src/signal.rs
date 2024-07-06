use anyhow::Result;

#[cfg(unix)]
pub async fn wait_for_signal() -> Result<()> {
    use tokio::select;
    use tokio::signal::unix::{signal, SignalKind};

    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    select! {
        _ = sigterm.recv() => println!("Received SIGTERM"),
        _ = sigint.recv() => println!("Received SIGINT"),
    };

    Ok(())
}

#[cfg(windows)]
pub async fn wait_for_signal() {
    unimplemented!();
}
