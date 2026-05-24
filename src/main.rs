mod config;
mod providers;
mod tray;
mod ui;
mod window;

use std::sync::{Arc, Mutex};
use gtk4::prelude::*;
use gtk4::{glib, Application};
use providers::ProviderData;

const APP_ID: &str = "io.github.amadeu01.ai-usage-indicator";

fn main() -> anyhow::Result<()> {
    let rt = Arc::new(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?,
    );

    let cfg = config::Config::load();
    let cfg = Arc::new(Mutex::new(cfg));

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    let rt_activate = rt.clone();
    let cfg_activate = cfg.clone();

    app.connect_activate(move |app| {
        let rt = rt_activate.clone();
        let cfg = cfg_activate.clone();

        let css_provider = gtk4::CssProvider::new();
        css_provider.load_from_string(include_str!("style.css"));
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &css_provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let win = std::rc::Rc::new(window::AppWindow::new(app));

        let (data_tx, data_rx) = async_channel::unbounded::<Vec<ProviderData>>();
        let (toggle_tx, toggle_rx) = async_channel::unbounded::<(i32, i32)>();

        let max_util = Arc::new(Mutex::new(0.0f32));

        let tray = tray::AppTray {
            max_utilization: max_util.clone(),
            toggle_tx,
        };

        let tray_service = ksni::TrayService::new(tray);
        tray_service.spawn();

        let data_tx_poll = data_tx.clone();
        let cfg_poll = cfg.clone();
        rt.spawn(async move {
            loop {
                let (interval_secs, current_cfg) = {
                    let c = cfg_poll.lock().unwrap();
                    (c.poll_interval_secs, c.clone())
                };
                let data = providers::fetch_all(&current_cfg).await;
                data_tx_poll.send(data).await.ok();
                tokio::time::sleep(tokio::time::Duration::from_secs(interval_secs)).await;
            }
        });

        let data_tx_sighup = data_tx.clone();
        let cfg_sighup = cfg.clone();
        rt.spawn(async move {
            use tokio::signal::unix::{signal, SignalKind};
            let mut sig = signal(SignalKind::hangup()).expect("SIGHUP handler failed");
            loop {
                sig.recv().await;
                eprintln!("SIGHUP received — reloading config");
                let new_cfg = config::Config::load();
                *cfg_sighup.lock().unwrap() = new_cfg.clone();
                let data = providers::fetch_all(&new_cfg).await;
                data_tx_sighup.send(data).await.ok();
            }
        });

        let win_toggle = win.clone();
        glib::spawn_future_local(async move {
            while let Ok((x, y)) = toggle_rx.recv().await {
                win_toggle.show_at(x, y);
            }
        });

        let win_data = win.clone();
        let max_util_recv = max_util.clone();
        glib::spawn_future_local(async move {
            while let Ok(data) = data_rx.recv().await {
                let max = data.iter()
                    .filter(|d| d.error.is_none())
                    .map(|d| d.utilization)
                    .fold(0.0f32, f32::max);
                *max_util_recv.lock().unwrap() = max;
                win_data.update_data(data);
            }
        });

        let (refresh_done_tx, refresh_done_rx) = async_channel::bounded::<()>(1);
        let btn_renable = win.refresh_btn().clone();
        glib::spawn_future_local(async move {
            while refresh_done_rx.recv().await.is_ok() {
                btn_renable.set_sensitive(true);
            }
        });

        let data_tx_refresh = data_tx.clone();
        let cfg_refresh = cfg.clone();
        let rt_refresh = rt.clone();
        win.refresh_btn().connect_clicked(move |btn| {
            btn.set_sensitive(false);
            let cfg_r = cfg_refresh.lock().unwrap().clone();
            let tx = data_tx_refresh.clone();
            let done_tx = refresh_done_tx.clone();
            rt_refresh.spawn(async move {
                let data = providers::fetch_all(&cfg_r).await;
                tx.send(data).await.ok();
                done_tx.send(()).await.ok();
            });
        });

    });

    app.run();
    Ok(())
}
