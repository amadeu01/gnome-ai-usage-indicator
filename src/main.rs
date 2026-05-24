mod config;
mod dbus;
mod providers;

use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use zbus::ConnectionBuilder;

use dbus::{AiUsageIndicatorInterface, BUS_NAME, OBJECT_PATH};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = Arc::new(Mutex::new(config::Config::load()));
    let data = Arc::new(Mutex::new(Vec::new()));
    let refresh_notify = Arc::new(Notify::new());

    let iface = AiUsageIndicatorInterface {
        data: data.clone(),
        refresh_notify: refresh_notify.clone(),
        config: cfg.clone(),
    };

    let conn = ConnectionBuilder::session()?
        .name(BUS_NAME)?
        .serve_at(OBJECT_PATH, iface)?
        .build()
        .await?;

    eprintln!("Daemon running — DBus name: {BUS_NAME}");

    let data_loop = data.clone();
    let cfg_loop = cfg.clone();
    let conn_loop = conn.clone();
    let refresh_loop = refresh_notify.clone();

    tokio::spawn(async move {
        loop {
            let (interval_secs, current_cfg) = {
                let c = cfg_loop.lock().await;
                (c.poll_interval_secs, c.clone())
            };

            let fetched = providers::fetch_all(&current_cfg).await;
            let json = serde_json::to_string(&fetched).unwrap_or_else(|_| "[]".to_string());

            *data_loop.lock().await = fetched;

            if let Ok(iface_ref) = conn_loop
                .object_server()
                .interface::<_, AiUsageIndicatorInterface>(OBJECT_PATH)
                .await
            {
                let signal_ctxt = iface_ref.signal_context();
                let _ = AiUsageIndicatorInterface::data_updated(signal_ctxt, &json).await;
            }

            tokio::select! {
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(interval_secs)) => {}
                _ = refresh_loop.notified() => {
                    eprintln!("Refresh triggered");
                }
            }
        }
    });

    tokio::spawn(async move {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sig = signal(SignalKind::hangup()).expect("SIGHUP handler failed");
        loop {
            sig.recv().await;
            eprintln!("SIGHUP — reloading config");
            let new_cfg = config::Config::load();
            *cfg.lock().await = new_cfg;
            refresh_notify.notify_one();
        }
    });

    // Block until process is killed
    tokio::signal::ctrl_c().await?;
    eprintln!("Shutting down");
    Ok(())
}
