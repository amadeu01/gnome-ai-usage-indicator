use ksni::Tray;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct AppTray {
    pub max_utilization: Arc<Mutex<f32>>,
    pub toggle_tx: async_channel::Sender<(i32, i32)>,
}

impl Tray for AppTray {
    fn id(&self) -> String {
        "ai-usage-indicator".to_string()
    }

    fn title(&self) -> String {
        "AI Usage Indicator".to_string()
    }

    fn icon_name(&self) -> String {
        let util = *self.max_utilization.lock().unwrap();
        if util >= 95.0 {
            "dialog-warning-symbolic".to_string()
        } else if util >= 80.0 {
            "emblem-important-symbolic".to_string()
        } else {
            "utilities-system-monitor-symbolic".to_string()
        }
    }

    fn tool_tip(&self) -> ksni::ToolTip {
        let util = *self.max_utilization.lock().unwrap();
        let description = if util >= 95.0 {
            format!("AI Usage: {:.0}% — Critical", util)
        } else if util >= 80.0 {
            format!("AI Usage: {:.0}% — Warning", util)
        } else {
            format!("AI Usage: {:.0}% max", util)
        };
        ksni::ToolTip {
            icon_name: self.icon_name(),
            icon_pixmap: vec![],
            title: "AI Usage".to_string(),
            description,
        }
    }

    fn activate(&mut self, x: i32, y: i32) {
        self.toggle_tx.send_blocking((x, y)).ok();
    }
}
