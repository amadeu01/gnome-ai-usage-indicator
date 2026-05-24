use chrono::Utc;
use gtk4::prelude::*;
use gtk4::{Box, Label, Orientation};
use crate::providers::ProviderData;
use super::usage_bar::build_usage_bar;

pub fn build_provider_group(entries: &[ProviderData]) -> Box {
    let outer = Box::new(Orientation::Vertical, 4);
    outer.add_css_class("provider-group");

    if entries.is_empty() {
        return outer;
    }

    let header = Label::new(Some(&entries[0].name));
    header.add_css_class("provider-header");
    header.set_halign(gtk4::Align::Start);
    outer.append(&header);

    for entry in entries {
        let section = build_entry_section(entry);
        outer.append(&section);
    }

    outer
}

fn build_entry_section(entry: &ProviderData) -> Box {
    let vbox = Box::new(Orientation::Vertical, 2);
    vbox.add_css_class("entry-section");

    if let Some(ref label_text) = entry.window_label {
        let lbl = Label::new(Some(label_text));
        lbl.add_css_class("window-label");
        lbl.set_halign(gtk4::Align::Start);
        vbox.append(&lbl);
    }

    if let Some(ref err) = entry.error {
        let err_lbl = Label::new(Some(err));
        err_lbl.add_css_class("error-label");
        err_lbl.set_halign(gtk4::Align::Start);
        err_lbl.set_wrap(true);
        vbox.append(&err_lbl);
        return vbox;
    }

    let has_limit = entry.utilization > 0.0 || entry.limit_credits.is_some();
    let is_ollama = entry.id == "ollama";

    if !is_ollama && has_limit {
        let bar = build_usage_bar(entry.utilization);
        vbox.append(&bar);

        let usage_text = if let (Some(used), Some(limit)) = (entry.used_credits, entry.limit_credits) {
            format!("${:.2} / ${:.2}", used, limit)
        } else {
            format!("{:.0}%", entry.utilization)
        };
        let usage_lbl = Label::new(Some(&usage_text));
        usage_lbl.add_css_class("usage-text");
        usage_lbl.set_halign(gtk4::Align::Start);
        vbox.append(&usage_lbl);
    }

    if let Some(ref reset_at) = entry.reset_at {
        let now = Utc::now();
        let diff = *reset_at - now;
        let reset_text = if diff.num_seconds() <= 0 {
            "Resets soon".to_string()
        } else if diff.num_hours() > 0 {
            format!("Resets in {}h {}m", diff.num_hours(), diff.num_minutes() % 60)
        } else {
            format!("Resets in {}m", diff.num_minutes())
        };
        let reset_lbl = Label::new(Some(&reset_text));
        reset_lbl.add_css_class("reset-label");
        reset_lbl.set_halign(gtk4::Align::Start);
        vbox.append(&reset_lbl);
    }

    if let Some(ref pace) = entry.pace_info {
        let pace_lbl = Label::new(Some(&pace.label));
        if pace.ahead {
            pace_lbl.add_css_class("pace-warning");
        } else {
            pace_lbl.add_css_class("pace-behind");
        }
        pace_lbl.set_halign(gtk4::Align::Start);
        vbox.append(&pace_lbl);
    }

    if let Some(ref meta_text) = entry.meta {
        let meta_lbl = Label::new(Some(meta_text));
        meta_lbl.add_css_class("meta-label");
        meta_lbl.set_halign(gtk4::Align::Start);
        meta_lbl.set_wrap(true);
        vbox.append(&meta_lbl);
    }

    let cost_text = match (entry.tokens_used, entry.cost_usd) {
        (Some(t), Some(c)) => Some(format!("{} · ${:.2}", format_tokens(t), c)),
        (Some(t), None) => Some(format_tokens(t)),
        (None, Some(c)) => Some(format!("${:.2}", c)),
        (None, None) => None,
    };
    if let Some(text) = cost_text {
        let cost_lbl = Label::new(Some(&text));
        cost_lbl.add_css_class("cost-label");
        cost_lbl.set_halign(gtk4::Align::Start);
        vbox.append(&cost_lbl);
    }

    vbox
}

fn format_tokens(tokens: u64) -> String {
    if tokens >= 1_000_000 {
        format!("{}M tokens", tokens / 1_000_000)
    } else if tokens >= 1_000 {
        format!("{}K tokens", tokens / 1_000)
    } else {
        format!("{} tokens", tokens)
    }
}
