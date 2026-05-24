use gtk4::prelude::*;
use gtk4::ProgressBar;

pub fn build_usage_bar(utilization: f32) -> ProgressBar {
    let bar = ProgressBar::new();
    bar.set_fraction((utilization / 100.0).clamp(0.0, 1.0) as f64);
    bar.remove_css_class("usage-ok");
    bar.remove_css_class("usage-warning");
    bar.remove_css_class("usage-critical");
    if utilization >= 95.0 {
        bar.add_css_class("usage-critical");
    } else if utilization >= 80.0 {
        bar.add_css_class("usage-warning");
    } else {
        bar.add_css_class("usage-ok");
    }
    bar
}
