//! Native Linux Graphical User Interface for FREE WOLF K8 Keyboard
//! Replicating the exact pixel positioning, layout, alignment, and styling of the Python Tkinter app.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;
use libadwaita as adw;
use adw::prelude::*;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, CheckButton, DropDown, Fixed, Image, Label,
    Orientation, PasswordEntry, Picture, Scale, ScrolledWindow, Separator,
    Stack, StringList, TextView, WrapMode,
};
use crate::config::{ConfigManager, Settings};
use crate::driver::{FreeWolfK8Driver, DeviceState};
use crate::i18n::{LANGUAGES, get_mode_name, t};
use crate::macro_mgr::{Macro, MacroManager, UinputPlayer};
use crate::manual::get_topics;
use crate::music::MusicVisualizerEngine;
use crate::protocol::{LIGHT_MODES, LightMode};

pub fn run_gui() {
    let app = adw::Application::builder()
        .application_id("com.freewolf.k8")
        .build();

    app.connect_activate(build_ui);
    app.run_with_args(&Vec::<String>::new());
}

struct AppState {
    settings: Settings,
    current_mode: &'static LightMode,
    macro_mgr: MacroManager,
    #[allow(dead_code)]
    current_macro: Option<Macro>,
    music_engine: MusicVisualizerEngine,
    current_help_topic: usize,
    is_music_active: bool,
}

fn get_asset_path(rel: &str) -> Option<PathBuf> {
    let p = Path::new(rel);
    if p.exists() {
        return Some(p.to_path_buf());
    }
    let abs = Path::new("/home/unl0cker/Desktop/FreeWolf-K8-Rust").join(rel);
    if abs.exists() {
        return Some(abs);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let exe_cand = parent.join(rel);
            if exe_cand.exists() {
                return Some(exe_cand);
            }
        }
    }
    None
}

fn build_ui(app: &adw::Application) {
    // Force Dark Theme
    let style_manager = adw::StyleManager::default();
    style_manager.set_color_scheme(adw::ColorScheme::ForceDark);

    // Apply custom Argonaut GNOME theme styling matching the Python Tkinter app EXACTLY
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        "window {\n\
             background-color: #0e1019;\n\
         }\n\
         headerbar {\n\
             background-color: #0e1019;\n\
             border-bottom: 1px solid #181c2e;\n\
             color: #ffffff;\n\
             min-height: 38px;\n\
         }\n\
         headerbar .title {\n\
             font-weight: bold;\n\
             font-size: 12px;\n\
             color: #ffffff;\n\
         }\n\
         .sidebar-bg {\n\
             background-color: #0e1019;\n\
             border-right: 1px solid #181c2e;\n\
         }\n\
         .nav-btn {\n\
             background-color: transparent;\n\
             border: none;\n\
             border-radius: 0;\n\
             min-width: 48px;\n\
             min-height: 44px;\n\
             padding: 0;\n\
         }\n\
         .nav-btn:hover {\n\
             background-color: #161b2c;\n\
         }\n\
         .nav-btn-active {\n\
             background-color: #132238;\n\
             border-left: 3px solid #027ad7;\n\
         }\n\
         .card-panel {\n\
             background-color: #151829;\n\
             border: 1px solid #232840;\n\
             border-radius: 0;\n\
             padding: 0;\n\
         }\n\
         .card-title {\n\
             font-size: 11px;\n\
             font-weight: bold;\n\
             color: #ffffff;\n\
         }\n\
         .field-title {\n\
             font-size: 10px;\n\
             font-weight: bold;\n\
             color: #ffffff;\n\
         }\n\
         .badge-connected {\n\
             background-color: #101321;\n\
             color: #8ce10b;\n\
             border: 1px solid #8ce10b;\n\
             font-weight: bold;\n\
             font-size: 11px;\n\
             padding: 5px 6px;\n\
         }\n\
         .badge-warning {\n\
             background-color: #101321;\n\
             color: #ffb900;\n\
             border: 1px solid #ffb900;\n\
             font-weight: bold;\n\
             font-size: 11px;\n\
             padding: 5px 6px;\n\
         }\n\
         .badge-disconnected {\n\
             background-color: #101321;\n\
             color: #7e88a0;\n\
             border: 1px solid #232840;\n\
             font-weight: bold;\n\
             font-size: 11px;\n\
             padding: 5px 6px;\n\
         }\n\
         .status-circle-ok {\n\
             background-color: #8ce10b;\n\
             border-radius: 5px;\n\
             min-width: 10px;\n\
             min-height: 10px;\n\
         }\n\
         .status-circle-warn {\n\
             background-color: #ffb900;\n\
             border-radius: 5px;\n\
             min-width: 10px;\n\
             min-height: 10px;\n\
         }\n\
         .status-circle-err {\n\
             background-color: #ED5F5D;\n\
             border-radius: 5px;\n\
             min-width: 10px;\n\
             min-height: 10px;\n\
         }\n\
         .accent-btn {\n\
             background-color: #027ad7;\n\
             color: #ffffff;\n\
             font-weight: bold;\n\
             border: none;\n\
             padding: 4px 10px;\n\
             border-radius: 2px;\n\
             font-size: 9px;\n\
         }\n\
         .accent-btn:hover {\n\
             background-color: #1a8fe5;\n\
         }\n\
         .secondary-btn {\n\
             background-color: #1a1e32;\n\
             color: #ffffff;\n\
             border: 1px solid #262c45;\n\
             border-radius: 2px;\n\
             font-size: 9px;\n\
             font-weight: bold;\n\
             padding: 5px 8px;\n\
         }\n\
         .secondary-btn:hover {\n\
             background-color: #222842;\n\
         }\n\
         .topic-item-btn {\n\
             background: transparent;\n\
             border: none;\n\
             border-radius: 0;\n\
             padding: 7px 10px;\n\
             color: #d8dee9;\n\
             font-size: 10px;\n\
         }\n\
         .topic-item-btn:hover {\n\
             background-color: #1c2035;\n\
             color: #ffffff;\n\
         }\n\
         .topic-item-active {\n\
             background-color: #132238;\n\
             color: #027ad7;\n\
             border-left: 3px solid #027ad7;\n\
             font-weight: bold;\n\
         }\n\
         scale highlight {\n\
             background-color: #027ad7;\n\
         }\n\
         scale slider {\n\
             background-color: #ffffff;\n\
             min-width: 16px;\n\
             min-height: 16px;\n\
         }\n\
         scale trough {\n\
             background-color: #101321;\n\
             border: 1px solid #232840;\n\
             min-height: 6px;\n\
         }\n"
    );
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let initial_settings = ConfigManager::load();
    let initial_mode = LIGHT_MODES
        .iter()
        .find(|m| m.id == initial_settings.mode_id)
        .unwrap_or(&LIGHT_MODES[1]);

    let state = Rc::new(RefCell::new(AppState {
        settings: initial_settings,
        current_mode: initial_mode,
        macro_mgr: MacroManager::load(),
        current_macro: None,
        music_engine: MusicVisualizerEngine::new(),
        current_help_topic: 0,
        is_music_active: false,
    }));

    let cur_lang = state.borrow().settings.language.clone();

    // -------------------------------------------------------------------------
    // WINDOW & TITLEBAR (Exact dimensions: 835 width, 524 content height)
    // -------------------------------------------------------------------------
    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(t(&cur_lang, "app_title"))
        .default_width(835)
        .default_height(564)
        .resizable(false)
        .build();

    let root_vbox = GtkBox::new(Orientation::Vertical, 0);

    let header_bar = adw::HeaderBar::new();
    let win_title = adw::WindowTitle::new(t(&cur_lang, "app_title"), "");
    header_bar.set_title_widget(Some(&win_title));
    header_bar.set_show_end_title_buttons(true);
    header_bar.set_show_start_title_buttons(true);
    root_vbox.append(&header_bar);

    // Canvas Fixed Container for EXACT pixel-level alignment (835 x 524)
    let fixed_canvas = Fixed::new();
    fixed_canvas.set_size_request(835, 524);
    root_vbox.append(&fixed_canvas);
    window.set_content(Some(&root_vbox));

    // -------------------------------------------------------------------------
    // 1. LEFT NAVBAR (48px wide, vertical position starting at y=73, center 95)
    // -------------------------------------------------------------------------
    let navbar = Fixed::new();
    navbar.set_size_request(48, 524);
    navbar.add_css_class("sidebar-bg");

    let path_light_act = get_asset_path("assets/icon/light_active.png");
    let path_light_inact = get_asset_path("assets/icon/light_inactive.png");
    let path_macro_act = get_asset_path("assets/icon/macro_active.png");
    let path_macro_inact = get_asset_path("assets/icon/macro_inactive.png");
    let path_help_act = get_asset_path("assets/icon/help_active.png");
    let path_help_inact = get_asset_path("assets/icon/help_inactive.png");

    let img_nav_light = if let Some(ref p) = path_light_act {
        Image::from_file(p)
    } else {
        Image::from_icon_name("preferences-system-symbolic")
    };
    img_nav_light.set_pixel_size(36);

    let img_nav_macro = if let Some(ref p) = path_macro_inact {
        Image::from_file(p)
    } else {
        Image::from_icon_name("input-keyboard-symbolic")
    };
    img_nav_macro.set_pixel_size(36);

    let img_nav_help = if let Some(ref p) = path_help_inact {
        Image::from_file(p)
    } else {
        Image::from_icon_name("help-browser-symbolic")
    };
    img_nav_help.set_pixel_size(36);

    let btn_tab_light = Button::new();
    btn_tab_light.set_size_request(48, 44);
    btn_tab_light.set_child(Some(&img_nav_light));
    btn_tab_light.add_css_class("nav-btn");
    btn_tab_light.add_css_class("nav-btn-active");
    btn_tab_light.set_tooltip_text(Some(t(&cur_lang, "tab_light")));
    navbar.put(&btn_tab_light, 0.0, 73.0); // center y=95

    let btn_tab_macro = Button::new();
    btn_tab_macro.set_size_request(48, 44);
    btn_tab_macro.set_child(Some(&img_nav_macro));
    btn_tab_macro.add_css_class("nav-btn");
    btn_tab_macro.set_tooltip_text(Some(t(&cur_lang, "tab_macro")));
    navbar.put(&btn_tab_macro, 0.0, 143.0); // center y=165

    let btn_tab_help = Button::new();
    btn_tab_help.set_size_request(48, 44);
    btn_tab_help.set_child(Some(&img_nav_help));
    btn_tab_help.add_css_class("nav-btn");
    btn_tab_help.set_tooltip_text(Some(t(&cur_lang, "tab_help")));
    navbar.put(&btn_tab_help, 0.0, 213.0); // center y=235

    fixed_canvas.put(&navbar, 0.0, 0.0);

    // -------------------------------------------------------------------------
    // LEFT AND RIGHT CARD STACKS (Placed at x=56, y=15 and x=280, y=15)
    // -------------------------------------------------------------------------
    let stack_left = Stack::new();
    stack_left.set_size_request(210, 494);
    stack_left.set_transition_type(gtk4::StackTransitionType::None);

    let stack_right = Stack::new();
    stack_right.set_size_request(530, 494);
    stack_right.set_transition_type(gtk4::StackTransitionType::None);

    fixed_canvas.put(&stack_left, 56.0, 15.0);
    fixed_canvas.put(&stack_right, 280.0, 15.0);

    // =========================================================================
    // VIEW 1: LIGHT VIEW
    // =========================================================================
    // --- 1. Left Panel (w=210, h=494) ---
    let card_light_info = GtkBox::new(Orientation::Vertical, 0);
    card_light_info.set_size_request(210, 494);
    card_light_info.add_css_class("card-panel");

    // Top: Device Connected Header
    let lbl_conn_title = Label::new(Some(t(&cur_lang, "dev_connected")));
    lbl_conn_title.add_css_class("card-title");
    lbl_conn_title.set_justify(gtk4::Justification::Center);
    lbl_conn_title.set_margin_top(10);
    lbl_conn_title.set_margin_bottom(8);
    card_light_info.append(&lbl_conn_title);

    // Device Badge Box
    let dev_box = GtkBox::new(Orientation::Vertical, 6);
    dev_box.set_margin_start(12);
    dev_box.set_margin_end(12);
    dev_box.set_margin_bottom(8);

    let lbl_dev_badge = Label::new(Some(t(&cur_lang, "dev_badge_connected")));
    lbl_dev_badge.add_css_class("badge-connected");
    lbl_dev_badge.set_justify(gtk4::Justification::Center);
    dev_box.append(&lbl_dev_badge);

    let lbl_dev_detail = Label::new(Some("VID: 0x1A2C  PID: 0x7C80\nInterface 1 (HID)"));
    lbl_dev_detail.set_opacity(0.6);
    lbl_dev_detail.set_justify(gtk4::Justification::Center);
    dev_box.append(&lbl_dev_detail);
    card_light_info.append(&dev_box);

    let sep1 = Separator::new(Orientation::Horizontal);
    sep1.set_margin_start(14);
    sep1.set_margin_end(14);
    sep1.set_margin_top(10);
    sep1.set_margin_bottom(10);
    card_light_info.append(&sep1);

    // Language Section
    let lang_box = GtkBox::new(Orientation::Vertical, 5);
    lang_box.set_margin_start(14);
    lang_box.set_margin_end(14);
    lang_box.set_margin_bottom(8);

    let lbl_lang = Label::new(Some(t(&cur_lang, "language")));
    lbl_lang.add_css_class("field-title");
    lbl_lang.set_halign(Align::Start);
    lang_box.append(&lbl_lang);

    let lang_names: Vec<&str> = LANGUAGES.iter().map(|(_, name)| *name).collect();
    let lang_model = StringList::new(&lang_names);
    let combo_lang = DropDown::new(Some(lang_model), None::<gtk4::Expression>);
    if let Some(pos) = LANGUAGES.iter().position(|(code, _)| *code == cur_lang) {
        combo_lang.set_selected(pos as u32);
    }
    lang_box.append(&combo_lang);
    card_light_info.append(&lang_box);

    let sep2 = Separator::new(Orientation::Horizontal);
    sep2.set_margin_start(14);
    sep2.set_margin_end(14);
    sep2.set_margin_top(10);
    sep2.set_margin_bottom(10);
    card_light_info.append(&sep2);

    // Auto Run CheckButton
    let opt_box = GtkBox::new(Orientation::Vertical, 0);
    opt_box.set_margin_start(14);
    opt_box.set_margin_end(14);
    opt_box.set_margin_bottom(8);

    let cb_autorun = CheckButton::with_label(t(&cur_lang, "auto_run"));
    cb_autorun.set_active(state.borrow().settings.auto_run);
    opt_box.append(&cb_autorun);
    card_light_info.append(&opt_box);

    let sep3 = Separator::new(Orientation::Horizontal);
    sep3.set_margin_start(14);
    sep3.set_margin_end(14);
    sep3.set_margin_top(10);
    sep3.set_margin_bottom(10);
    card_light_info.append(&sep3);

    // Reset Factory Settings Button
    let btn_box = GtkBox::new(Orientation::Vertical, 0);
    btn_box.set_margin_start(14);
    btn_box.set_margin_end(14);
    btn_box.set_margin_top(4);
    btn_box.set_margin_bottom(8);

    let btn_restore = Button::with_label(t(&cur_lang, "restore_factory"));
    btn_restore.add_css_class("secondary-btn");
    btn_box.append(&btn_restore);
    card_light_info.append(&btn_box);

    // Spacer
    let spacer_info = GtkBox::new(Orientation::Vertical, 0);
    spacer_info.set_vexpand(true);
    card_light_info.append(&spacer_info);

    let sep4 = Separator::new(Orientation::Horizontal);
    sep4.set_margin_start(14);
    sep4.set_margin_end(14);
    sep4.set_margin_bottom(8);
    card_light_info.append(&sep4);

    // Connection Status Section (Anchored at bottom)
    let status_box = GtkBox::new(Orientation::Vertical, 2);
    status_box.set_margin_start(14);
    status_box.set_margin_end(14);
    status_box.set_margin_bottom(6);

    let status_hdr = GtkBox::new(Orientation::Horizontal, 6);
    let status_circle = GtkBox::new(Orientation::Horizontal, 0);
    status_circle.add_css_class("status-circle-ok");
    status_circle.set_valign(Align::Center);
    status_hdr.append(&status_circle);

    let lbl_status = Label::new(Some("Connected"));
    lbl_status.add_css_class("field-title");
    lbl_status.set_halign(Align::Start);
    status_hdr.append(&lbl_status);
    status_box.append(&status_hdr);

    let lbl_status_node = Label::new(Some("Ready on /dev/hidraw1"));
    lbl_status_node.set_opacity(0.6);
    lbl_status_node.set_halign(Align::Start);
    lbl_status_node.set_margin_start(16);
    status_box.append(&lbl_status_node);

    let btn_fix_udev = Button::with_label(t(&cur_lang, "btn_fix_udev"));
    btn_fix_udev.add_css_class("accent-btn");
    btn_fix_udev.set_visible(false);
    btn_fix_udev.set_margin_top(4);
    status_box.append(&btn_fix_udev);
    card_light_info.append(&status_box);

    // Versioning
    let ver_box = GtkBox::new(Orientation::Horizontal, 0);
    ver_box.set_margin_start(14);
    ver_box.set_margin_end(14);
    ver_box.set_margin_bottom(10);
    let lbl_ver = Label::new(Some("Ver: 1.0.3.1 (Native Rust)"));
    lbl_ver.set_opacity(0.6);
    lbl_ver.set_halign(Align::Start);
    ver_box.append(&lbl_ver);
    card_light_info.append(&ver_box);

    stack_left.add_named(&card_light_info, Some("light"));

    // --- 2. Right Panel: Lighting Modes & Controls (w=530, h=494) ---
    let card_light_modes = GtkBox::new(Orientation::Vertical, 0);
    card_light_modes.set_size_request(530, 494);
    card_light_modes.add_css_class("card-panel");

    let lbl_light_modes_title = Label::new(Some(t(&cur_lang, "lighting_modes")));
    lbl_light_modes_title.add_css_class("card-title");
    lbl_light_modes_title.set_justify(gtk4::Justification::Center);
    lbl_light_modes_title.set_margin_top(10);
    lbl_light_modes_title.set_margin_bottom(8);
    card_light_modes.append(&lbl_light_modes_title);

    let sep_modes = Separator::new(Orientation::Horizontal);
    card_light_modes.append(&sep_modes);

    // Dual-Column Mode Radio Button Grid (Matching Python Tkinter layout)
    let grid_box = GtkBox::new(Orientation::Vertical, 0);
    grid_box.set_margin_start(18);
    grid_box.set_margin_end(18);
    grid_box.set_margin_top(8);
    grid_box.set_margin_bottom(6);

    let grid_modes = gtk4::Grid::new();
    grid_modes.set_column_spacing(16);
    grid_modes.set_row_spacing(2);
    grid_modes.set_column_homogeneous(true);

    let first_rb = CheckButton::with_label(get_mode_name(&cur_lang, LIGHT_MODES[0].id, LIGHT_MODES[0].name));
    let mut radio_buttons = Vec::new();
    radio_buttons.push(first_rb.clone());

    let stack_controls = Stack::new();
    stack_controls.set_transition_type(gtk4::StackTransitionType::Crossfade);

    for (i, m) in LIGHT_MODES.iter().enumerate() {
        let rb = if i == 0 {
            first_rb.clone()
        } else {
            let b = CheckButton::with_label(get_mode_name(&cur_lang, m.id, m.name));
            b.set_group(Some(&first_rb));
            b
        };

        if m.id == state.borrow().settings.mode_id {
            rb.set_active(true);
        }

        let state_rc = state.clone();
        let m_static: &'static LightMode = m;
        let stack_ctrl_ref = stack_controls.clone();
        rb.connect_toggled(move |btn| {
            if btn.is_active() {
                let mut st = state_rc.borrow_mut();
                st.current_mode = m_static;
                st.settings.mode_id = m_static.id;
                ConfigManager::save(&st.settings);

                if m_static.is_music() {
                    stack_ctrl_ref.set_visible_child_name("music");
                } else {
                    stack_ctrl_ref.set_visible_child_name("std");
                    if st.is_music_active {
                        st.music_engine.stop();
                        st.is_music_active = false;
                    }
                    let probe = FreeWolfK8Driver::probe();
                    if let Some(node) = probe.node {
                        let _ = FreeWolfK8Driver::set_lighting(&node, m_static, st.settings.brightness, st.settings.speed);
                    }
                }
            }
        });

        // row = i // 2, col = i % 2
        let row = (i / 2) as i32;
        let col = (i % 2) as i32;
        grid_modes.attach(&rb, col, row, 1, 1);
        if i > 0 {
            radio_buttons.push(rb);
        }
    }
    grid_box.append(&grid_modes);
    card_light_modes.append(&grid_box);

    let sep_ctrl = Separator::new(Orientation::Horizontal);
    sep_ctrl.set_margin_top(6);
    sep_ctrl.set_margin_bottom(8);
    card_light_modes.append(&sep_ctrl);

    // Controls Section
    // 1. Standard Sliders Page
    let std_controls_box = GtkBox::new(Orientation::Vertical, 6);
    std_controls_box.set_valign(Align::Center);
    std_controls_box.set_margin_top(4);

    let row_b = GtkBox::new(Orientation::Horizontal, 14);
    row_b.set_halign(Align::Center);
    let lbl_b = Label::new(Some(t(&cur_lang, "light_brightness")));
    lbl_b.set_size_request(85, -1);
    lbl_b.set_halign(Align::End);
    lbl_b.add_css_class("field-title");
    let scale_b = Scale::with_range(Orientation::Horizontal, 0.0, 4.0, 1.0);
    scale_b.set_size_request(280, -1);
    scale_b.set_value(state.borrow().settings.brightness as f64);
    scale_b.set_draw_value(false);
    row_b.append(&lbl_b);
    row_b.append(&scale_b);
    std_controls_box.append(&row_b);

    let row_s = GtkBox::new(Orientation::Horizontal, 14);
    row_s.set_halign(Align::Center);
    let lbl_s = Label::new(Some(t(&cur_lang, "light_speed")));
    lbl_s.set_size_request(85, -1);
    lbl_s.set_halign(Align::End);
    lbl_s.add_css_class("field-title");
    let scale_s = Scale::with_range(Orientation::Horizontal, 0.0, 4.0, 1.0);
    scale_s.set_size_request(280, -1);
    scale_s.set_value(state.borrow().settings.speed as f64);
    scale_s.set_draw_value(false);
    row_s.append(&lbl_s);
    row_s.append(&scale_s);
    std_controls_box.append(&row_s);

    let state_b = state.clone();
    scale_b.connect_value_changed(move |sc| {
        let val = sc.value() as u8;
        let mut st = state_b.borrow_mut();
        st.settings.brightness = val;
        ConfigManager::save(&st.settings);
        let probe = FreeWolfK8Driver::probe();
        if let Some(node) = probe.node {
            let _ = FreeWolfK8Driver::set_lighting(&node, st.current_mode, val, st.settings.speed);
        }
    });

    let state_s = state.clone();
    scale_s.connect_value_changed(move |sc| {
        let val = sc.value() as u8;
        let mut st = state_s.borrow_mut();
        st.settings.speed = val;
        ConfigManager::save(&st.settings);
        let probe = FreeWolfK8Driver::probe();
        if let Some(node) = probe.node {
            let _ = FreeWolfK8Driver::set_lighting(&node, st.current_mode, st.settings.brightness, val);
        }
    });

    stack_controls.add_named(&std_controls_box, Some("std"));

    // 2. Music Visualizer Page
    let music_box = GtkBox::new(Orientation::Vertical, 6);
    music_box.set_valign(Align::Center);

    let lbl_music_title = Label::new(Some(t(&cur_lang, "music_title")));
    lbl_music_title.add_css_class("field-title");
    lbl_music_title.set_justify(gtk4::Justification::Center);
    music_box.append(&lbl_music_title);

    let row_m1 = GtkBox::new(Orientation::Horizontal, 10);
    row_m1.set_halign(Align::Center);
    let lbl_pattern = Label::new(Some(t(&cur_lang, "music_pattern")));
    lbl_pattern.add_css_class("field-title");
    let combo_pattern = DropDown::from_strings(&["Mode 1 (Single/Linear)", "Mode 2 (Dynamic Stereo)"]);
    combo_pattern.set_selected((state.borrow().settings.music_submode.saturating_sub(1)) as u32);
    row_m1.append(&lbl_pattern);
    row_m1.append(&combo_pattern);
    music_box.append(&row_m1);

    let row_m2 = GtkBox::new(Orientation::Horizontal, 10);
    row_m2.set_halign(Align::Center);
    let lbl_freq = Label::new(Some(t(&cur_lang, "music_freq")));
    lbl_freq.add_css_class("field-title");
    let combo_freq = DropDown::from_strings(&["33ms (30Hz)", "66ms (15Hz)", "100ms (10Hz)"]);
    combo_freq.set_selected(1);
    row_m2.append(&lbl_freq);
    row_m2.append(&combo_freq);
    music_box.append(&row_m2);

    let row_m3 = GtkBox::new(Orientation::Horizontal, 12);
    row_m3.set_halign(Align::Center);
    let btn_music_toggle = Button::with_label(t(&cur_lang, "music_start"));
    btn_music_toggle.add_css_class("accent-btn");
    let lbl_music_status = Label::new(Some("Idle"));
    lbl_music_status.set_opacity(0.6);
    row_m3.append(&btn_music_toggle);
    row_m3.append(&lbl_music_status);
    music_box.append(&row_m3);

    let state_mus = state.clone();
    let btn_mus_toggle = btn_music_toggle.clone();
    let lbl_mus_st = lbl_music_status.clone();
    let cp_ref = combo_pattern.clone();
    let cf_ref = combo_freq.clone();
    btn_music_toggle.connect_clicked(move |_| {
        let mut st = state_mus.borrow_mut();
        if st.is_music_active {
            st.music_engine.stop();
            st.is_music_active = false;
            btn_mus_toggle.set_label(t(&st.settings.language, "music_start"));
            lbl_mus_st.set_text("Idle");
        } else {
            let submode = (cp_ref.selected() + 1) as u8;
            let delay = match cf_ref.selected() {
                0 => 33,
                1 => 66,
                _ => 100,
            };
            st.music_engine.start(submode, delay);
            st.is_music_active = true;
            btn_mus_toggle.set_label(t(&st.settings.language, "music_stop"));
            lbl_mus_st.set_text("Streaming audio spectrum to LEDs...");
        }
    });

    stack_controls.add_named(&music_box, Some("music"));

    if initial_mode.is_music() {
        stack_controls.set_visible_child_name("music");
    } else {
        stack_controls.set_visible_child_name("std");
    }

    card_light_modes.append(&stack_controls);
    stack_right.add_named(&card_light_modes, Some("light"));

    // =========================================================================
    // VIEW 2: MACRO VIEW
    // =========================================================================
    // --- Left Card: Macro List (w=210, h=494) ---
    let card_macro_left = GtkBox::new(Orientation::Vertical, 0);
    card_macro_left.set_size_request(210, 494);
    card_macro_left.add_css_class("card-panel");

    let lbl_macro_title = Label::new(Some(t(&cur_lang, "macro_list")));
    lbl_macro_title.add_css_class("card-title");
    lbl_macro_title.set_justify(gtk4::Justification::Center);
    lbl_macro_title.set_margin_top(10);
    lbl_macro_title.set_margin_bottom(8);
    card_macro_left.append(&lbl_macro_title);
    card_macro_left.append(&Separator::new(Orientation::Horizontal));

    let macro_scrolled = ScrolledWindow::new();
    macro_scrolled.set_vexpand(true);
    let macro_text = TextView::new();
    macro_text.set_editable(false);
    macro_text.set_cursor_visible(false);

    let mut macro_summary = String::new();
    for m in &state.borrow().macro_mgr.macros {
        macro_summary.push_str(&format!("[ID: {}] {}\n", m.id, m.name));
    }
    if macro_summary.is_empty() {
        macro_summary.push_str("No macros saved yet.");
    }
    macro_text.buffer().set_text(&macro_summary);
    macro_scrolled.set_child(Some(&macro_text));
    card_macro_left.append(&macro_scrolled);
    stack_left.add_named(&card_macro_left, Some("macro"));

    // --- Right Card: Macro Actions & Controls (w=530, h=494) ---
    let card_macro_right = GtkBox::new(Orientation::Vertical, 8);
    card_macro_right.set_size_request(530, 494);

    // Top Card (h=155)
    let macro_top_card = GtkBox::new(Orientation::Vertical, 8);
    macro_top_card.set_size_request(530, 155);
    macro_top_card.add_css_class("card-panel");

    let macro_toolbar = GtkBox::new(Orientation::Horizontal, 8);
    macro_toolbar.set_margin_start(16);
    macro_toolbar.set_margin_end(16);
    macro_toolbar.set_margin_top(10);

    let btn_macro_new = Button::with_label(t(&cur_lang, "btn_new"));
    btn_macro_new.add_css_class("secondary-btn");
    let btn_macro_del = Button::with_label(t(&cur_lang, "btn_delete"));
    btn_macro_del.add_css_class("secondary-btn");
    let btn_macro_copy = Button::with_label(t(&cur_lang, "btn_copy"));
    btn_macro_copy.add_css_class("secondary-btn");
    let btn_macro_play = Button::with_label("▶ Play");
    btn_macro_play.add_css_class("accent-btn");

    macro_toolbar.append(&btn_macro_new);
    macro_toolbar.append(&btn_macro_del);
    macro_toolbar.append(&btn_macro_copy);
    macro_toolbar.append(&btn_macro_play);
    macro_top_card.append(&macro_toolbar);
    card_macro_right.append(&macro_top_card);

    // Bottom Card (h=331)
    let macro_bottom_card = GtkBox::new(Orientation::Vertical, 0);
    macro_bottom_card.set_size_request(530, 331);
    macro_bottom_card.add_css_class("card-panel");

    let macro_detail_scrolled = ScrolledWindow::new();
    macro_detail_scrolled.set_vexpand(true);
    let macro_detail_text = TextView::new();
    macro_detail_text.set_editable(false);
    macro_detail_text.buffer().set_text("Select a macro from the left list to view keystrokes and timing.\nPress 'Play' to execute virtual keystrokes via Linux /dev/uinput.");
    macro_detail_scrolled.set_child(Some(&macro_detail_text));
    macro_bottom_card.append(&macro_detail_scrolled);
    card_macro_right.append(&macro_bottom_card);

    let state_play = state.clone();
    btn_macro_play.connect_clicked(move |_| {
        if let Some(m) = state_play.borrow().macro_mgr.macros.first() {
            if let Ok(player) = UinputPlayer::new() {
                player.play(m);
            }
        }
    });

    stack_right.add_named(&card_macro_right, Some("macro"));

    // =========================================================================
    // VIEW 3: HELP VIEW
    // =========================================================================
    // --- Left Card: 9 Topics List (w=210, h=494) ---
    let card_help_left = GtkBox::new(Orientation::Vertical, 0);
    card_help_left.set_size_request(210, 494);
    card_help_left.add_css_class("card-panel");

    let lbl_help_title = Label::new(Some(t(&cur_lang, "help_title")));
    lbl_help_title.add_css_class("card-title");
    lbl_help_title.set_justify(gtk4::Justification::Center);
    lbl_help_title.set_margin_top(12);
    lbl_help_title.set_margin_bottom(8);
    card_help_left.append(&lbl_help_title);

    let sep_help_left = Separator::new(Orientation::Horizontal);
    sep_help_left.set_margin_start(8);
    sep_help_left.set_margin_end(8);
    sep_help_left.set_margin_bottom(6);
    card_help_left.append(&sep_help_left);

    let topic_scrolled = ScrolledWindow::new();
    topic_scrolled.set_vexpand(true);
    let topic_box = GtkBox::new(Orientation::Vertical, 2);
    topic_box.set_margin_start(4);
    topic_box.set_margin_end(4);

    let initial_topics = get_topics(&cur_lang);
    let mut topic_buttons: Vec<Button> = Vec::new();

    let right_title = Label::new(None);
    right_title.add_css_class("card-title");
    right_title.set_halign(Align::Start);

    let help_text_view = TextView::new();
    help_text_view.set_editable(false);
    help_text_view.set_wrap_mode(WrapMode::Word);

    let img_kb_path = get_asset_path("assets/keyboard/kb_102.png");
    let pic_overview = if let Some(ref p) = img_kb_path {
        let pic = Picture::for_filename(p);
        pic.set_can_shrink(true);
        pic.set_size_request(420, 204);
        pic.set_margin_bottom(8);
        pic.set_halign(Align::Center);
        Some(pic)
    } else {
        None
    };

    for (idx, top) in initial_topics.iter().enumerate() {
        let btn = Button::with_label(&format!("{} {}", top.icon, top.title));
        btn.add_css_class("topic-item-btn");
        if idx == 0 {
            btn.add_css_class("topic-item-active");
            right_title.set_text(&format!("{} {}", top.icon, top.title));
            help_text_view.buffer().set_text(top.content);
        }

        let state_top = state.clone();
        let rt_ref = right_title.clone();
        let htv_ref = help_text_view.clone();
        let pic_ref = pic_overview.clone();
        btn.connect_clicked(move |_| {
            let mut st = state_top.borrow_mut();
            st.current_help_topic = idx;
            let current_topics = get_topics(&st.settings.language);
            if let Some(t) = current_topics.get(idx) {
                rt_ref.set_text(&format!("{} {}", t.icon, t.title));
                htv_ref.buffer().set_text(t.content);
            }
            if let Some(ref p) = pic_ref {
                p.set_visible(idx == 0);
            }
        });

        topic_box.append(&btn);
        topic_buttons.push(btn);
    }
    topic_scrolled.set_child(Some(&topic_box));
    card_help_left.append(&topic_scrolled);
    stack_left.add_named(&card_help_left, Some("help"));

    // --- Right Card: Topic Content (w=530, h=494) ---
    let card_help_right = GtkBox::new(Orientation::Vertical, 0);
    card_help_right.set_size_request(530, 494);
    card_help_right.add_css_class("card-panel");

    let help_header_row = GtkBox::new(Orientation::Horizontal, 8);
    help_header_row.set_margin_start(16);
    help_header_row.set_margin_end(16);
    help_header_row.set_margin_top(10);
    help_header_row.set_margin_bottom(10);

    right_title.set_hexpand(true);
    help_header_row.append(&right_title);

    let btn_prev_topic = Button::with_label(t(&cur_lang, "btn_prev"));
    btn_prev_topic.add_css_class("secondary-btn");
    let btn_next_topic = Button::with_label(t(&cur_lang, "btn_next"));
    btn_next_topic.add_css_class("secondary-btn");
    help_header_row.append(&btn_prev_topic);
    help_header_row.append(&btn_next_topic);
    card_help_right.append(&help_header_row);

    card_help_right.append(&Separator::new(Orientation::Horizontal));

    let help_content_scroll = ScrolledWindow::new();
    help_content_scroll.set_vexpand(true);
    help_content_scroll.set_margin_start(10);
    help_content_scroll.set_margin_end(10);
    help_content_scroll.set_margin_top(10);
    help_content_scroll.set_margin_bottom(10);

    let help_inner_vbox = GtkBox::new(Orientation::Vertical, 6);
    if let Some(ref pic) = pic_overview {
        help_inner_vbox.append(pic);
    }
    help_inner_vbox.append(&help_text_view);
    help_content_scroll.set_child(Some(&help_inner_vbox));
    card_help_right.append(&help_content_scroll);

    // Prev / Next button handlers
    let state_prev = state.clone();
    let rt_prev = right_title.clone();
    let htv_prev = help_text_view.clone();
    let pic_prev = pic_overview.clone();
    let tb_prev = topic_buttons.clone();
    btn_prev_topic.connect_clicked(move |_| {
        let mut st = state_prev.borrow_mut();
        if st.current_help_topic > 0 {
            st.current_help_topic -= 1;
            let cur_idx = st.current_help_topic;
            let topics = get_topics(&st.settings.language);
            if let Some(t) = topics.get(cur_idx) {
                rt_prev.set_text(&format!("{} {}", t.icon, t.title));
                htv_prev.buffer().set_text(t.content);
            }
            if let Some(ref p) = pic_prev {
                p.set_visible(cur_idx == 0);
            }
            for (i, b) in tb_prev.iter().enumerate() {
                if i == cur_idx {
                    b.add_css_class("topic-item-active");
                } else {
                    b.remove_css_class("topic-item-active");
                }
            }
        }
    });

    let state_next = state.clone();
    let rt_next = right_title.clone();
    let htv_next = help_text_view.clone();
    let pic_next = pic_overview.clone();
    let tb_next = topic_buttons.clone();
    btn_next_topic.connect_clicked(move |_| {
        let mut st = state_next.borrow_mut();
        let topics = get_topics(&st.settings.language);
        if st.current_help_topic + 1 < topics.len() {
            st.current_help_topic += 1;
            let cur_idx = st.current_help_topic;
            if let Some(t) = topics.get(cur_idx) {
                rt_next.set_text(&format!("{} {}", t.icon, t.title));
                htv_next.buffer().set_text(t.content);
            }
            if let Some(ref p) = pic_next {
                p.set_visible(cur_idx == 0);
            }
            for (i, b) in tb_next.iter().enumerate() {
                if i == cur_idx {
                    b.add_css_class("topic-item-active");
                } else {
                    b.remove_css_class("topic-item-active");
                }
            }
        }
    });

    stack_right.add_named(&card_help_right, Some("help"));

    // -------------------------------------------------------------------------
    // NAVBAR SWITCHING CALLBACKS
    // -------------------------------------------------------------------------
    let sl_ref = stack_left.clone();
    let sr_ref = stack_right.clone();
    let b_light = btn_tab_light.clone();
    let b_macro = btn_tab_macro.clone();
    let b_help = btn_tab_help.clone();
    let img_l = img_nav_light.clone();
    let img_m = img_nav_macro.clone();
    let img_h = img_nav_help.clone();
    let p_la = path_light_act.clone();
    let p_li = path_light_inact.clone();
    let p_ma = path_macro_act.clone();
    let p_mi = path_macro_inact.clone();
    let p_ha = path_help_act.clone();
    let p_hi = path_help_inact.clone();

    let update_nav_visuals = move |active_tab: &str| {
        b_light.remove_css_class("nav-btn-active");
        b_macro.remove_css_class("nav-btn-active");
        b_help.remove_css_class("nav-btn-active");

        match active_tab {
            "light" => {
                b_light.add_css_class("nav-btn-active");
                if let Some(ref p) = p_la { img_l.set_from_file(Some(p)); }
                if let Some(ref p) = p_mi { img_m.set_from_file(Some(p)); }
                if let Some(ref p) = p_hi { img_h.set_from_file(Some(p)); }
            }
            "macro" => {
                b_macro.add_css_class("nav-btn-active");
                if let Some(ref p) = p_li { img_l.set_from_file(Some(p)); }
                if let Some(ref p) = p_ma { img_m.set_from_file(Some(p)); }
                if let Some(ref p) = p_hi { img_h.set_from_file(Some(p)); }
            }
            "help" => {
                b_help.add_css_class("nav-btn-active");
                if let Some(ref p) = p_li { img_l.set_from_file(Some(p)); }
                if let Some(ref p) = p_mi { img_m.set_from_file(Some(p)); }
                if let Some(ref p) = p_ha { img_h.set_from_file(Some(p)); }
            }
            _ => {}
        }
    };

    let sl1 = sl_ref.clone();
    let sr1 = sr_ref.clone();
    let unv1 = update_nav_visuals.clone();
    btn_tab_light.connect_clicked(move |_| {
        sl1.set_visible_child_name("light");
        sr1.set_visible_child_name("light");
        unv1("light");
    });

    let sl2 = sl_ref.clone();
    let sr2 = sr_ref.clone();
    let unv2 = update_nav_visuals.clone();
    btn_tab_macro.connect_clicked(move |_| {
        sl2.set_visible_child_name("macro");
        sr2.set_visible_child_name("macro");
        unv2("macro");
    });

    let sl3 = sl_ref.clone();
    let sr3 = sr_ref.clone();
    let unv3 = update_nav_visuals.clone();
    btn_tab_help.connect_clicked(move |_| {
        sl3.set_visible_child_name("help");
        sr3.set_visible_child_name("help");
        unv3("help");
    });

    // -------------------------------------------------------------------------
    // LIVE DYNAMIC LANGUAGE SWITCHING
    // -------------------------------------------------------------------------
    let state_lang = state.clone();
    let win_title_lang = win_title.clone();
    let win_ref_lang = window.clone();
    let lbl_ct_lang = lbl_conn_title.clone();
    let lbl_l_lang = lbl_lang.clone();
    let cb_ar_lang = cb_autorun.clone();
    let btn_res_lang = btn_restore.clone();
    let btn_fix_lang = btn_fix_udev.clone();
    let lbl_lm_lang = lbl_light_modes_title.clone();
    let lbl_b_l = lbl_b.clone();
    let lbl_s_l = lbl_s.clone();
    let lbl_mt_lang = lbl_music_title.clone();
    let lbl_pat_lang = lbl_pattern.clone();
    let lbl_fr_lang = lbl_freq.clone();
    let btn_mt_lang = btn_music_toggle.clone();
    let lbl_mac_lang = lbl_macro_title.clone();
    let btn_mn_lang = btn_macro_new.clone();
    let btn_md_lang = btn_macro_del.clone();
    let btn_mc_lang = btn_macro_copy.clone();
    let lbl_ht_lang = lbl_help_title.clone();
    let rt_lang = right_title.clone();
    let htv_lang = help_text_view.clone();
    let btn_pt_lang = btn_prev_topic.clone();
    let btn_nt_lang = btn_next_topic.clone();
    let rbs_lang = radio_buttons.clone();
    let tb_lang = topic_buttons.clone();

    combo_lang.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        if let Some((code, _)) = LANGUAGES.get(idx) {
            let mut st = state_lang.borrow_mut();
            st.settings.language = code.to_string();
            ConfigManager::save(&st.settings);

            // Titlebar
            win_title_lang.set_title(t(code, "app_title"));
            win_ref_lang.set_title(Some(t(code, "app_title")));

            // Light View Left Card
            lbl_ct_lang.set_text(t(code, "dev_connected"));
            lbl_l_lang.set_text(t(code, "language"));
            cb_ar_lang.set_label(Some(t(code, "auto_run")));
            btn_res_lang.set_label(t(code, "restore_factory"));
            btn_fix_lang.set_label(t(code, "btn_fix_udev"));

            // Light View Right Card
            lbl_lm_lang.set_text(t(code, "lighting_modes"));
            lbl_b_l.set_text(t(code, "light_brightness"));
            lbl_s_l.set_text(t(code, "light_speed"));
            for (rb, m) in rbs_lang.iter().zip(LIGHT_MODES.iter()) {
                rb.set_label(Some(get_mode_name(code, m.id, m.name)));
            }

            // Music Controls
            lbl_mt_lang.set_text(t(code, "music_title"));
            lbl_pat_lang.set_text(t(code, "music_pattern"));
            lbl_fr_lang.set_text(t(code, "music_freq"));
            if !st.is_music_active {
                btn_mt_lang.set_label(t(code, "music_start"));
            }

            // Macro View
            lbl_mac_lang.set_text(t(code, "macro_list"));
            btn_mn_lang.set_label(t(code, "btn_new"));
            btn_md_lang.set_label(t(code, "btn_delete"));
            btn_mc_lang.set_label(t(code, "btn_copy"));

            // Help View
            lbl_ht_lang.set_text(t(code, "help_title"));
            btn_pt_lang.set_label(t(code, "btn_prev"));
            btn_nt_lang.set_label(t(code, "btn_next"));

            let topics = get_topics(code);
            for (i, b) in tb_lang.iter().enumerate() {
                if let Some(top) = topics.get(i) {
                    b.set_label(&format!("{} {}", top.icon, top.title));
                }
            }
            if let Some(t) = topics.get(st.current_help_topic) {
                rt_lang.set_text(&format!("{} {}", t.icon, t.title));
                htv_lang.buffer().set_text(t.content);
            }
        }
    });

    // Auto Run Toggle
    cb_autorun.connect_toggled(move |btn| {
        let val = btn.is_active();
        ConfigManager::set_autostart(val, None);
    });

    // Reset Factory Settings Callback
    let state_res = state.clone();
    let sc_b = scale_b.clone();
    let sc_s = scale_s.clone();
    let rb_first = radio_buttons.get(1).unwrap_or(&first_rb).clone(); // Mode 1: Steady
    btn_restore.connect_clicked(move |_| {
        let mut st = state_res.borrow_mut();
        st.settings.brightness = 4;
        st.settings.speed = 2;
        st.settings.mode_id = 1;
        ConfigManager::save(&st.settings);

        sc_b.set_value(4.0);
        sc_s.set_value(2.0);
        rb_first.set_active(true);

        let probe = FreeWolfK8Driver::probe();
        if let Some(node) = probe.node {
            let _ = FreeWolfK8Driver::set_lighting(&node, &LIGHT_MODES[1], 4, 2);
        }
    });

    // Fix udev Permissions Callback
    let win_fix = window.clone();
    btn_fix_udev.connect_clicked(move |_| {
        let dialog = gtk4::Window::builder()
            .transient_for(&win_fix)
            .modal(true)
            .title("Root Authentication")
            .default_width(380)
            .default_height(170)
            .resizable(false)
            .build();

        let vbox = GtkBox::new(Orientation::Vertical, 12);
        vbox.set_margin_start(16);
        vbox.set_margin_end(16);
        vbox.set_margin_top(16);
        vbox.set_margin_bottom(16);

        let lbl_msg = Label::new(Some("Authentication is required to install udev rules for FREE WOLF K8:"));
        lbl_msg.set_wrap(true);
        lbl_msg.set_wrap_mode(gtk4::pango::WrapMode::Word);
        vbox.append(&lbl_msg);

        let entry_pw = PasswordEntry::new();
        entry_pw.set_show_peek_icon(true);
        vbox.append(&entry_pw);

        let lbl_status_dlg = Label::new(None);
        lbl_status_dlg.set_css_classes(&["badge-warning"]);
        lbl_status_dlg.set_visible(false);
        vbox.append(&lbl_status_dlg);

        let hbox = GtkBox::new(Orientation::Horizontal, 8);
        hbox.set_halign(Align::End);
        let btn_cancel = Button::with_label("Cancel");
        let btn_auth = Button::with_label("Authenticate");
        btn_auth.add_css_class("accent-btn");
        hbox.append(&btn_cancel);
        hbox.append(&btn_auth);
        vbox.append(&hbox);

        dialog.set_child(Some(&vbox));

        let dlg_c = dialog.clone();
        btn_cancel.connect_clicked(move |_| dlg_c.close());

        let dlg_a = dialog.clone();
        let ent = entry_pw.clone();
        let lbl_s = lbl_status_dlg.clone();
        btn_auth.connect_clicked(move |_| {
            let pw = ent.text();
            let (ok, msg) = crate::udev::run_setup_with_sudo(&pw);
            if ok {
                dlg_a.close();
            } else {
                lbl_s.set_text(&msg);
                lbl_s.set_visible(true);
                ent.set_text("");
            }
        });

        dialog.present();
    });

    // -------------------------------------------------------------------------
    // HARDWARE POLLER (Updates badge & status dot every 2 seconds)
    // -------------------------------------------------------------------------
    let lbl_node_timer = lbl_status_node.clone();
    let lbl_badge_timer = lbl_dev_badge.clone();
    let dot_timer = status_circle.clone();
    let btn_fix_timer = btn_fix_udev.clone();
    let lbl_st_timer = lbl_status.clone();

    glib::timeout_add_local(Duration::from_millis(2000), move || {
        let probe = FreeWolfK8Driver::probe();
        match probe.state {
            DeviceState::Connected => {
                lbl_st_timer.set_text("Connected");
                lbl_badge_timer.set_text("FREE WOLF K8 USB");
                lbl_badge_timer.set_css_classes(&["badge-connected"]);
                lbl_node_timer.set_text(&format!("Ready on {}", probe.node.unwrap_or_default()));
                dot_timer.set_css_classes(&["status-circle-ok"]);
                btn_fix_timer.set_visible(false);
            }
            DeviceState::PermissionDenied => {
                lbl_st_timer.set_text("Access Denied");
                lbl_badge_timer.set_text("FREE WOLF K8 (Access Denied)");
                lbl_badge_timer.set_css_classes(&["badge-warning"]);
                lbl_node_timer.set_text("Write permission required");
                dot_timer.set_css_classes(&["status-circle-warn"]);
                btn_fix_timer.set_visible(true);
            }
            DeviceState::ClaimedByVm => {
                lbl_st_timer.set_text("Claimed by VM");
                lbl_badge_timer.set_text("FREE WOLF K8 (QEMU VM)");
                lbl_badge_timer.set_css_classes(&["badge-warning"]);
                lbl_node_timer.set_text("USB claimed by guest OS");
                dot_timer.set_css_classes(&["status-circle-warn"]);
                btn_fix_timer.set_visible(false);
            }
            DeviceState::NotFound => {
                lbl_st_timer.set_text("Device disconnected");
                lbl_badge_timer.set_text("NO DEVICE DETECTED");
                lbl_badge_timer.set_css_classes(&["badge-disconnected"]);
                lbl_node_timer.set_text("Please connect keyboard");
                dot_timer.set_css_classes(&["status-circle-err"]);
                btn_fix_timer.set_visible(false);
            }
        }
        glib::ControlFlow::Continue
    });

    window.present();
}
