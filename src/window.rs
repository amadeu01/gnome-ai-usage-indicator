use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::{Duration, Instant};
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, Button, Label, Notebook, Orientation,
    ScrolledWindow, PolicyType, glib,
};
use crate::providers::ProviderData;
use crate::ui::provider_group::build_provider_group;

pub struct AppWindow {
    pub window: ApplicationWindow,
    notebook: Notebook,
    refresh_btn: Button,
    footer_label: Label,
    last_updated: Rc<RefCell<Option<Instant>>>,
    suppress_autohide: Rc<Cell<bool>>,
}

impl AppWindow {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("AI Usage")
            .decorated(false)
            .default_width(340)
            .resizable(false)
            .build();

        window.add_css_class("popover-window");

        let root_vbox = Box::new(Orientation::Vertical, 0);

        let header = Box::new(Orientation::Horizontal, 8);
        header.add_css_class("popover-header");
        header.set_margin_start(12);
        header.set_margin_end(12);
        header.set_margin_top(8);
        header.set_margin_bottom(8);

        let title_lbl = Label::new(Some("AI Usage"));
        title_lbl.add_css_class("popover-title");
        title_lbl.set_hexpand(true);
        title_lbl.set_halign(gtk4::Align::Start);

        let refresh_btn = Button::from_icon_name("view-refresh-symbolic");
        refresh_btn.add_css_class("flat");

        header.append(&title_lbl);
        header.append(&refresh_btn);

        let sep = gtk4::Separator::new(Orientation::Horizontal);

        let notebook = Notebook::new();
        notebook.set_show_border(false);

        let footer_sep = gtk4::Separator::new(Orientation::Horizontal);
        let footer_label = Label::new(Some("Loading…"));
        footer_label.add_css_class("footer-label");
        footer_label.set_margin_start(12);
        footer_label.set_margin_end(12);
        footer_label.set_margin_top(6);
        footer_label.set_margin_bottom(6);

        root_vbox.append(&header);
        root_vbox.append(&sep);
        root_vbox.append(&notebook);
        root_vbox.append(&footer_sep);
        root_vbox.append(&footer_label);

        window.set_child(Some(&root_vbox));

        let last_updated: Rc<RefCell<Option<Instant>>> = Rc::new(RefCell::new(None));

        let footer_clone = footer_label.clone();
        let lu_clone = last_updated.clone();
        glib::timeout_add_seconds_local(30, move || {
            update_footer(&footer_clone, &lu_clone);
            glib::ControlFlow::Continue
        });

        let suppress_autohide = Rc::new(Cell::new(false));

        let win_clone = window.clone();
        let suppress = suppress_autohide.clone();
        window.connect_notify_local(Some("is-active"), move |w, _| {
            if !w.is_active() && !suppress.get() {
                win_clone.set_visible(false);
            }
        });

        AppWindow { window, notebook, refresh_btn, footer_label, last_updated, suppress_autohide }
    }

    pub fn update_data(&self, data: Vec<ProviderData>) {
        // Save current tab label for restoration after rebuild
        let current_tab_label: Option<String> = self.notebook.current_page()
            .and_then(|page| self.notebook.nth_page(Some(page)))
            .and_then(|child| self.notebook.tab_label(&child))
            .and_then(|w| w.downcast::<Label>().ok())
            .map(|l| l.text().to_string());

        // Remove all existing pages
        while self.notebook.n_pages() > 0 {
            self.notebook.remove_page(Some(0));
        }

        if data.is_empty() {
            let msg = Label::new(Some("No providers configured — edit ~/.config/ai-usage-indicator/config.toml"));
            msg.set_wrap(true);
            msg.set_margin_start(12);
            msg.set_margin_end(12);
            msg.set_margin_top(8);
            msg.set_margin_bottom(8);
            let tab_label = Label::new(Some("Info"));
            self.notebook.append_page(&msg, Some(&tab_label));
        } else {
            // BTreeMap gives alphabetical ordering by provider name
            let mut grouped: BTreeMap<String, Vec<ProviderData>> = BTreeMap::new();
            for entry in data {
                grouped.entry(entry.name.clone()).or_default().push(entry);
            }

            let mut restore_page: Option<u32> = None;

            for (provider_name, entries) in &grouped {
                let content = Box::new(Orientation::Vertical, 8);
                content.set_margin_start(12);
                content.set_margin_end(12);
                content.set_margin_top(8);
                content.set_margin_bottom(8);

                let widget = build_provider_group(entries);
                content.append(&widget);

                let scrolled = ScrolledWindow::builder()
                    .hscrollbar_policy(PolicyType::Never)
                    .vscrollbar_policy(PolicyType::Automatic)
                    .max_content_height(500)
                    .propagate_natural_height(true)
                    .child(&content)
                    .build();

                let tab_label = Label::new(Some(provider_name));
                let page_num = self.notebook.append_page(&scrolled, Some(&tab_label));

                if let Some(ref saved) = current_tab_label {
                    if saved == provider_name {
                        restore_page = Some(page_num);
                    }
                }
            }

            if let Some(page_num) = restore_page {
                self.notebook.set_current_page(Some(page_num));
            }
        }

        *self.last_updated.borrow_mut() = Some(Instant::now());
        update_footer(&self.footer_label, &self.last_updated);
    }

    pub fn refresh_btn(&self) -> &Button {
        &self.refresh_btn
    }

    /// Toggle-show window anchored below tray icon at (tray_x, tray_y).
    /// Hides if already visible. On X11/XWayland uses XMoveWindow for pixel-precise
    /// anchoring. On Wayland falls back to compositor placement.
    pub fn show_at(&self, tray_x: i32, tray_y: i32) {
        if self.window.is_visible() {
            self.window.set_visible(false);
            return;
        }

        let (win_width, _) = self.window.default_size();
        let mut win_x = tray_x - win_width / 2;
        let mut win_y = tray_y + 24;

        // Clamp to monitor geometry
        if let Some(display) = gtk4::gdk::Display::default() {
            let monitors = display.monitors();
            for i in 0..monitors.n_items() {
                if let Some(monitor) = monitors.item(i).and_downcast::<gtk4::gdk::Monitor>() {
                    let geom = monitor.geometry();
                    if tray_x >= geom.x() && tray_x <= geom.x() + geom.width() {
                        win_x = win_x.max(geom.x()).min(geom.x() + geom.width() - win_width);
                        win_y = win_y.max(geom.y()).min(geom.y() + geom.height() - 100);
                        break;
                    }
                }
            }
        }

        // Suppress auto-hide for 300 ms: prevents the is-active focus-out event
        // that fires transiently right after present() from closing the window.
        self.suppress_autohide.set(true);
        let flag = self.suppress_autohide.clone();
        glib::timeout_add_local_once(Duration::from_millis(300), move || {
            flag.set(false);
        });

        // Realize creates the underlying X11 window (unmapped); we can then move
        // it before present() maps it, so the WM places it at the right position.
        gtk4::prelude::WidgetExt::realize(&self.window);
        try_position_window(&self.window, win_x, win_y);
        self.window.present();
    }
}

/// Move window to (x, y) on X11/XWayland using XMoveWindow.
/// Silently no-ops on Wayland (downcast fails) or if not yet realized.
fn try_position_window(window: &ApplicationWindow, x: i32, y: i32) {
    use gdk4_x11::prelude::*;

    let Some(surface) = window.surface() else { return };
    let Ok(x11_surf) = surface.downcast::<gdk4_x11::X11Surface>() else { return };
    let Ok(x11_disp) = gtk4::prelude::WidgetExt::display(window).downcast::<gdk4_x11::X11Display>() else { return };

    let xid = x11_surf.xid();
    // SAFETY: xdisplay() returns the underlying Xlib Display pointer; valid as long as
    // the GDK display is open. XMoveWindow is re-entrant-safe at this call site.
    unsafe {
        let xdisplay: *mut x11::xlib::Display = x11_disp.xdisplay();
        x11::xlib::XMoveWindow(xdisplay, xid, x, y);
    }
}

fn update_footer(label: &Label, last_updated: &Rc<RefCell<Option<Instant>>>) {
    let text = match *last_updated.borrow() {
        None => "Never updated".to_string(),
        Some(t) => {
            let secs = t.elapsed().as_secs();
            if secs < 60 {
                label.remove_css_class("footer-stale");
                "Updated just now".to_string()
            } else if secs < 300 {
                label.remove_css_class("footer-stale");
                format!("Updated {}m ago", secs / 60)
            } else {
                label.add_css_class("footer-stale");
                format!("Updated {}m ago", secs / 60)
            }
        }
    };
    label.set_text(&text);
}
