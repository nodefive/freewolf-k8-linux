//! Modern Libadwaita / GTK4 Graphical User Interface for FREE WOLF K8

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;
use libadwaita as adw;
use adw::prelude::*;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, CheckButton, DropDown, Label, Orientation,
    PasswordEntry, Picture, Scale, ScrolledWindow, Separator, Stack, StringList, Switch, TextView, WrapMode,
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
    #[allow(dead_code)]
    music_engine: MusicVisualizerEngine,
    current_help_topic: usize,
}

fn build_ui(app: &adw::Application) {
    // Force Dark Theme
    let style_manager = adw::StyleManager::default();
    style_manager.set_color_scheme(adw::ColorScheme::ForceDark);

    // Apply custom Argonaut GNOME theme styling
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        "window { background-color: #0e1019; }\n\
         .sidebar-card { background-color: #151829; border: 1px solid #232840; border-radius: 10px; padding: 12px; }\n\
         .content-card { background-color: #151829; border: 1px solid #232840; border-radius: 10px; padding: 14px; }\n\
         .badge-connected { background-color: #101321; color: #8ce10b; border: 1px solid #8ce10b; border-radius: 6px; font-weight: bold; padding: 4px; }\n\
         .badge-disconnected { background-color: #101321; color: #7e88a0; border: 1px solid #232840; border-radius: 6px; font-weight: bold; padding: 4px; }\n\
         .badge-warning { background-color: #101321; color: #ffb900; border: 1px solid #ffb900; border-radius: 6px; font-weight: bold; padding: 4px; }\n\
         .accent-btn { background-color: #027ad7; color: #ffffff; font-weight: bold; border-radius: 6px; }\n\
         .accent-btn:hover { background-color: #1a8fe5; }\n\
         .nav-active { background-color: #132238; border-left: 3px solid #027ad7; font-weight: bold; }\n\
         scale highlight { background-color: #027ad7; }\n\
         scale slider { background-color: #ffffff; }\n"
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
    }));

    let cur_lang = state.borrow().settings.language.clone();

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title(t(&cur_lang, "app_title"))
        .default_width(860)
        .default_height(550)
        .resizable(false)
        .build();

    let root_box = GtkBox::new(Orientation::Horizontal, 12);
    root_box.set_margin_start(12);
    root_box.set_margin_end(12);
    root_box.set_margin_top(12);
    root_box.set_margin_bottom(12);
    window.set_content(Some(&root_box));

    let main_stack = Stack::new();
    main_stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    main_stack.set_hexpand(true);
    main_stack.set_vexpand(true);

    // -------------------------------------------------------------------------
    // LEFT SIDEBAR (Navigation + System Info + Device Badge)
    // -------------------------------------------------------------------------
    let sidebar = GtkBox::new(Orientation::Vertical, 10);
    sidebar.set_size_request(230, -1);
    sidebar.add_css_class("sidebar-card");

    // Nav Switcher Buttons
    let nav_box = GtkBox::new(Orientation::Vertical, 4);
    let btn_nav_light = Button::with_label(&format!("⚙  {}", t(&cur_lang, "tab_light")));
    let btn_nav_macro = Button::with_label(&format!("⌨  {}", t(&cur_lang, "tab_macro")));
    let btn_nav_help = Button::with_label(&format!("📖  {}", t(&cur_lang, "tab_help")));

    btn_nav_light.add_css_class("nav-active");
    nav_box.append(&btn_nav_light);
    nav_box.append(&btn_nav_macro);
    nav_box.append(&btn_nav_help);
    sidebar.append(&nav_box);

    sidebar.append(&Separator::new(Orientation::Horizontal));

    // Device Badge & Connection Info
    let lbl_conn_title = Label::new(Some("FREE WOLF K8 USB"));
    lbl_conn_title.add_css_class("badge-connected");
    lbl_conn_title.set_justify(gtk4::Justification::Center);
    sidebar.append(&lbl_conn_title);

    let lbl_node = Label::new(Some("Checking device..."));
    lbl_node.set_wrap(true);
    lbl_node.set_wrap_mode(gtk4::pango::WrapMode::Word);
    lbl_node.set_justify(gtk4::Justification::Center);
    sidebar.append(&lbl_node);

    let btn_fix_udev = Button::with_label("Fix Permissions");
    btn_fix_udev.add_css_class("accent-btn");
    btn_fix_udev.set_visible(false);
    sidebar.append(&btn_fix_udev);

    sidebar.append(&Separator::new(Orientation::Horizontal));

    // Language Selector
    let lbl_lang = Label::new(Some(t(&cur_lang, "language")));
    lbl_lang.set_halign(Align::Start);
    sidebar.append(&lbl_lang);

    let lang_names: Vec<&str> = LANGUAGES.iter().map(|(_, name)| *name).collect();
    let lang_model = StringList::new(&lang_names);
    let combo_lang = DropDown::new(Some(lang_model), None::<gtk4::Expression>);
    
    if let Some(pos) = LANGUAGES.iter().position(|(code, _)| *code == cur_lang) {
        combo_lang.set_selected(pos as u32);
    }
    sidebar.append(&combo_lang);

    // Auto Run Switch
    let auto_box = GtkBox::new(Orientation::Horizontal, 8);
    let lbl_autorun = Label::new(Some(t(&cur_lang, "auto_run")));
    lbl_autorun.set_hexpand(true);
    lbl_autorun.set_halign(Align::Start);
    let switch_autorun = Switch::new();
    switch_autorun.set_active(state.borrow().settings.auto_run);
    auto_box.append(&lbl_autorun);
    auto_box.append(&switch_autorun);
    sidebar.append(&auto_box);

    // Reset Settings Button
    let btn_reset = Button::with_label(t(&cur_lang, "reset"));
    sidebar.append(&btn_reset);

    // Version at bottom
    let spacer = GtkBox::new(Orientation::Vertical, 0);
    spacer.set_vexpand(true);
    sidebar.append(&spacer);

    let lbl_ver = Label::new(Some("Ver: 1.0.3.1 (Native Rust)"));
    lbl_ver.set_opacity(0.6);
    lbl_ver.set_halign(Align::Start);
    sidebar.append(&lbl_ver);

    root_box.append(&sidebar);
    root_box.append(&main_stack);

    // -------------------------------------------------------------------------
    // PAGE 1: LIGHTING CONTROLS
    // -------------------------------------------------------------------------
    let page_light = GtkBox::new(Orientation::Vertical, 10);
    page_light.add_css_class("content-card");

    let lbl_light_hdr = Label::new(Some(t(&cur_lang, "lighting_modes")));
    lbl_light_hdr.set_halign(Align::Start);
    lbl_light_hdr.add_css_class("title-2");
    page_light.append(&lbl_light_hdr);
    page_light.append(&Separator::new(Orientation::Horizontal));

    // Modes 2-Column Grid
    let grid_modes = gtk4::Grid::new();
    grid_modes.set_column_spacing(16);
    grid_modes.set_row_spacing(4);
    grid_modes.set_column_homogeneous(true);

    let first_mode_name = get_mode_name(&cur_lang, LIGHT_MODES[0].id, LIGHT_MODES[0].name);
    let first_rb = CheckButton::with_label(first_mode_name);
    let mut radio_buttons = Vec::new();
    radio_buttons.push(first_rb.clone());

    for (i, m) in LIGHT_MODES.iter().enumerate() {
        let rb = if i == 0 {
            first_rb.clone()
        } else {
            let mode_name = get_mode_name(&cur_lang, m.id, m.name);
            let b = CheckButton::with_label(mode_name);
            b.set_group(Some(&first_rb));
            b
        };

        if m.id == state.borrow().settings.mode_id {
            rb.set_active(true);
        }

        let state_rc = state.clone();
        let m_static: &'static LightMode = m;
        rb.connect_toggled(move |btn| {
            if btn.is_active() {
                let mut st = state_rc.borrow_mut();
                st.current_mode = m_static;
                st.settings.mode_id = m_static.id;
                ConfigManager::save(&st.settings);

                let probe = FreeWolfK8Driver::probe();
                if let Some(node) = probe.node {
                    let _ = FreeWolfK8Driver::set_lighting(&node, m_static, st.settings.brightness, st.settings.speed);
                }
            }
        });

        let col = (i % 2) as i32;
        let row = (i / 2) as i32;
        grid_modes.attach(&rb, col, row, 1, 1);
        if i > 0 {
            radio_buttons.push(rb);
        }
    }
    page_light.append(&grid_modes);
    page_light.append(&Separator::new(Orientation::Horizontal));

    // Sliders Box
    let sliders_box = GtkBox::new(Orientation::Vertical, 8);
    
    // Brightness row
    let row_b = GtkBox::new(Orientation::Horizontal, 12);
    let lbl_b = Label::new(Some(&format!("{}:", t(&cur_lang, "light_brightness"))));
    lbl_b.set_size_request(80, -1);
    lbl_b.set_halign(Align::End);
    let scale_b = Scale::with_range(Orientation::Horizontal, 0.0, 4.0, 1.0);
    scale_b.set_hexpand(true);
    scale_b.set_value(state.borrow().settings.brightness as f64);
    scale_b.set_draw_value(false);
    row_b.append(&lbl_b);
    row_b.append(&scale_b);
    sliders_box.append(&row_b);

    // Speed / Delay row
    let row_s = GtkBox::new(Orientation::Horizontal, 12);
    let lbl_s = Label::new(Some(&format!("{}:", t(&cur_lang, "light_speed"))));
    lbl_s.set_size_request(80, -1);
    lbl_s.set_halign(Align::End);
    let scale_s = Scale::with_range(Orientation::Horizontal, 0.0, 4.0, 1.0);
    scale_s.set_hexpand(true);
    scale_s.set_value(state.borrow().settings.speed as f64);
    scale_s.set_draw_value(false);
    row_s.append(&lbl_s);
    row_s.append(&scale_s);
    sliders_box.append(&row_s);

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

    page_light.append(&sliders_box);
    main_stack.add_named(&page_light, Some("light"));

    // -------------------------------------------------------------------------
    // PAGE 2: MACRO MANAGER
    // -------------------------------------------------------------------------
    let page_macro = GtkBox::new(Orientation::Vertical, 10);
    page_macro.add_css_class("content-card");

    let lbl_macro_hdr = Label::new(Some(t(&cur_lang, "macro_list")));
    lbl_macro_hdr.set_halign(Align::Start);
    lbl_macro_hdr.add_css_class("title-2");
    page_macro.append(&lbl_macro_hdr);
    page_macro.append(&Separator::new(Orientation::Horizontal));

    let macro_top = GtkBox::new(Orientation::Horizontal, 10);
    let btn_macro_new = Button::with_label(t(&cur_lang, "btn_new"));
    let btn_macro_del = Button::with_label(t(&cur_lang, "btn_delete"));
    let btn_macro_copy = Button::with_label(t(&cur_lang, "btn_copy"));
    let btn_macro_play = Button::with_label("▶ Play");
    btn_macro_play.add_css_class("accent-btn");

    macro_top.append(&btn_macro_new);
    macro_top.append(&btn_macro_del);
    macro_top.append(&btn_macro_copy);
    macro_top.append(&btn_macro_play);
    page_macro.append(&macro_top);

    let macro_scrolled = ScrolledWindow::new();
    macro_scrolled.set_vexpand(true);
    let macro_text = TextView::new();
    macro_text.set_editable(false);
    macro_text.set_cursor_visible(false);

    let mut macro_summary = String::new();
    for m in &state.borrow().macro_mgr.macros {
        macro_summary.push_str(&format!("[ID: {}] {} ({} actions, repeat: {})\n", m.id, m.name, m.actions.len(), m.repeat_time));
    }
    if macro_summary.is_empty() {
        macro_summary.push_str("No macros saved yet. Use the K8 macro manager to define custom keystrokes.");
    }
    macro_text.buffer().set_text(&macro_summary);
    macro_scrolled.set_child(Some(&macro_text));
    page_macro.append(&macro_scrolled);

    let state_play = state.clone();
    btn_macro_play.connect_clicked(move |_| {
        if let Some(m) = state_play.borrow().macro_mgr.macros.first() {
            if let Ok(player) = UinputPlayer::new() {
                player.play(m);
            }
        }
    });

    main_stack.add_named(&page_macro, Some("macro"));

    // -------------------------------------------------------------------------
    // PAGE 3: USER MANUAL & HELP
    // -------------------------------------------------------------------------
    let page_help = GtkBox::new(Orientation::Vertical, 10);
    page_help.add_css_class("content-card");

    let help_hdr = GtkBox::new(Orientation::Horizontal, 8);
    let lbl_help_topic_title = Label::new(Some("📋 Overview & Specs"));
    lbl_help_topic_title.set_hexpand(true);
    lbl_help_topic_title.set_halign(Align::Start);
    lbl_help_topic_title.add_css_class("title-2");

    let btn_prev_topic = Button::with_label(t(&cur_lang, "btn_prev"));
    let btn_next_topic = Button::with_label(t(&cur_lang, "btn_next"));

    help_hdr.append(&lbl_help_topic_title);
    help_hdr.append(&btn_prev_topic);
    help_hdr.append(&btn_next_topic);
    page_help.append(&help_hdr);
    page_help.append(&Separator::new(Orientation::Horizontal));

    // Keyboard Image for Overview
    let img_path = ["assets/keyboard/kb_102.png", "/home/unl0cker/Desktop/FreeWolf-K8-Rust/assets/keyboard/kb_102.png"]
        .into_iter()
        .find(|p| Path::new(p).exists());

    let pic_overview = if let Some(p) = img_path {
        let pic = Picture::for_filename(p);
        pic.set_can_shrink(true);
        pic.set_height_request(160);
        pic.set_margin_bottom(8);
        Some(pic)
    } else {
        None
    };

    if let Some(ref pic) = pic_overview {
        page_help.append(pic);
    }

    let help_scrolled = ScrolledWindow::new();
    help_scrolled.set_vexpand(true);
    let help_text = TextView::new();
    help_text.set_editable(false);
    help_text.set_wrap_mode(WrapMode::Word);

    let topics = get_topics(&state.borrow().settings.language);
    if let Some(first) = topics.first() {
        lbl_help_topic_title.set_text(&format!("{} {}", first.icon, first.title));
        help_text.buffer().set_text(first.content);
    }
    help_scrolled.set_child(Some(&help_text));
    page_help.append(&help_scrolled);

    let state_prev = state.clone();
    let text_prev = help_text.clone();
    let title_prev = lbl_help_topic_title.clone();
    let pic_prev = pic_overview.clone();
    btn_prev_topic.connect_clicked(move |_| {
        let mut st = state_prev.borrow_mut();
        if st.current_help_topic > 0 {
            st.current_help_topic -= 1;
            let topics = get_topics(&st.settings.language);
            if let Some(t) = topics.get(st.current_help_topic) {
                title_prev.set_text(&format!("{} {}", t.icon, t.title));
                text_prev.buffer().set_text(t.content);
            }
            if let Some(ref p) = pic_prev {
                p.set_visible(st.current_help_topic == 0);
            }
        }
    });

    let state_next = state.clone();
    let text_next = help_text.clone();
    let title_next = lbl_help_topic_title.clone();
    let pic_next = pic_overview.clone();
    btn_next_topic.connect_clicked(move |_| {
        let mut st = state_next.borrow_mut();
        let topics = get_topics(&st.settings.language);
        if st.current_help_topic + 1 < topics.len() {
            st.current_help_topic += 1;
            if let Some(t) = topics.get(st.current_help_topic) {
                title_next.set_text(&format!("{} {}", t.icon, t.title));
                text_next.buffer().set_text(t.content);
            }
            if let Some(ref p) = pic_next {
                p.set_visible(st.current_help_topic == 0);
            }
        }
    });

    main_stack.add_named(&page_help, Some("help"));

    // -------------------------------------------------------------------------
    // SIDEBAR NAVIGATION CALLBACKS
    // -------------------------------------------------------------------------
    let stack_l = main_stack.clone();
    let bnl = btn_nav_light.clone();
    let bnm = btn_nav_macro.clone();
    let bnh = btn_nav_help.clone();
    btn_nav_light.connect_clicked(move |_| {
        stack_l.set_visible_child_name("light");
        bnl.add_css_class("nav-active");
        bnm.remove_css_class("nav-active");
        bnh.remove_css_class("nav-active");
    });

    let stack_m = main_stack.clone();
    let bnl2 = btn_nav_light.clone();
    let bnm2 = btn_nav_macro.clone();
    let bnh2 = btn_nav_help.clone();
    btn_nav_macro.connect_clicked(move |_| {
        stack_m.set_visible_child_name("macro");
        bnm2.add_css_class("nav-active");
        bnl2.remove_css_class("nav-active");
        bnh2.remove_css_class("nav-active");
    });

    let stack_h = main_stack.clone();
    let bnl3 = btn_nav_light.clone();
    let bnm3 = btn_nav_macro.clone();
    let bnh3 = btn_nav_help.clone();
    btn_nav_help.connect_clicked(move |_| {
        stack_h.set_visible_child_name("help");
        bnh3.add_css_class("nav-active");
        bnl3.remove_css_class("nav-active");
        bnm3.remove_css_class("nav-active");
    });

    // Language DropDown change
    let state_lang = state.clone();
    let win_lang = window.clone();
    let help_text_lang = help_text.clone();
    let help_title_lang = lbl_help_topic_title.clone();
    let btn_nl_lang = btn_nav_light.clone();
    let btn_nm_lang = btn_nav_macro.clone();
    let btn_nh_lang = btn_nav_help.clone();
    let lbl_l_lang = lbl_lang.clone();
    let lbl_ar_lang = lbl_autorun.clone();
    let btn_res_lang = btn_reset.clone();
    let lbl_lh_lang = lbl_light_hdr.clone();
    let lbl_b_l = lbl_b.clone();
    let lbl_s_l = lbl_s.clone();
    let lbl_mh_lang = lbl_macro_hdr.clone();
    let btn_mn_lang = btn_macro_new.clone();
    let btn_md_lang = btn_macro_del.clone();
    let btn_mc_lang = btn_macro_copy.clone();
    let btn_pt_lang = btn_prev_topic.clone();
    let btn_nt_lang = btn_next_topic.clone();
    let rbs_lang = radio_buttons.clone();

    combo_lang.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        if let Some((code, _)) = LANGUAGES.get(idx) {
            let mut st = state_lang.borrow_mut();
            st.settings.language = code.to_string();
            ConfigManager::save(&st.settings);

            // Update Window & Nav Titles
            win_lang.set_title(Some(t(code, "app_title")));
            btn_nl_lang.set_label(&format!("⚙  {}", t(code, "tab_light")));
            btn_nm_lang.set_label(&format!("⌨  {}", t(code, "tab_macro")));
            btn_nh_lang.set_label(&format!("📖  {}", t(code, "tab_help")));

            // Update Sidebar Labels
            lbl_l_lang.set_text(t(code, "language"));
            lbl_ar_lang.set_text(t(code, "auto_run"));
            btn_res_lang.set_label(t(code, "reset"));

            // Update Light Page
            lbl_lh_lang.set_text(t(code, "lighting_modes"));
            lbl_b_l.set_text(&format!("{}:", t(code, "light_brightness")));
            lbl_s_l.set_text(&format!("{}:", t(code, "light_speed")));
            for (rb, m) in rbs_lang.iter().zip(LIGHT_MODES.iter()) {
                rb.set_label(Some(get_mode_name(code, m.id, m.name)));
            }

            // Update Macro Page
            lbl_mh_lang.set_text(t(code, "macro_list"));
            btn_mn_lang.set_label(t(code, "btn_new"));
            btn_md_lang.set_label(t(code, "btn_delete"));
            btn_mc_lang.set_label(t(code, "btn_copy"));

            // Update Help Page
            btn_pt_lang.set_label(t(code, "btn_prev"));
            btn_nt_lang.set_label(t(code, "btn_next"));
            let topics = get_topics(code);
            if let Some(t) = topics.get(st.current_help_topic) {
                help_title_lang.set_text(&format!("{} {}", t.icon, t.title));
                help_text_lang.buffer().set_text(t.content);
            }
        }
    });

    // Auto Run Toggle
    switch_autorun.connect_active_notify(move |sw| {
        let val = sw.is_active();
        ConfigManager::set_autostart(val, None);
    });

    // Reset Settings Button Callback
    let state_res = state.clone();
    let sc_b = scale_b.clone();
    let sc_s = scale_s.clone();
    let rb_first = radio_buttons.get(1).unwrap_or(&first_rb).clone(); // Mode 1: Steady
    btn_reset.connect_clicked(move |_| {
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

    // Fix udev Permissions Button Callback (Modal Dialog)
    let win_fix = window.clone();
    btn_fix_udev.connect_clicked(move |_| {
        let dialog = gtk4::Window::builder()
            .transient_for(&win_fix)
            .modal(true)
            .title("Root Authentication")
            .default_width(380)
            .default_height(180)
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

        let lbl_status = Label::new(None);
        lbl_status.set_css_classes(&["badge-warning"]);
        lbl_status.set_visible(false);
        vbox.append(&lbl_status);

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
        let lbl_s = lbl_status.clone();
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

    // Hardware Poller Timer (Every 2 seconds)
    let lbl_node_timer = lbl_node.clone();
    let lbl_badge_timer = lbl_conn_title.clone();
    let btn_fix_timer = btn_fix_udev.clone();
    glib::timeout_add_local(Duration::from_millis(2000), move || {
        let probe = FreeWolfK8Driver::probe();
        match probe.state {
            DeviceState::Connected => {
                lbl_badge_timer.set_text("FREE WOLF K8 USB");
                lbl_badge_timer.set_css_classes(&["badge-connected"]);
                lbl_node_timer.set_text(&format!("Connected ({})", probe.node.unwrap_or_default()));
                btn_fix_timer.set_visible(false);
            }
            DeviceState::PermissionDenied => {
                lbl_badge_timer.set_text("FREE WOLF K8 (Access Denied)");
                lbl_badge_timer.set_css_classes(&["badge-warning"]);
                lbl_node_timer.set_text("Write permission required");
                btn_fix_timer.set_visible(true);
            }
            DeviceState::ClaimedByVm => {
                lbl_badge_timer.set_text("FREE WOLF K8 (QEMU VM)");
                lbl_badge_timer.set_css_classes(&["badge-warning"]);
                lbl_node_timer.set_text("USB claimed by guest OS");
                btn_fix_timer.set_visible(false);
            }
            DeviceState::NotFound => {
                lbl_badge_timer.set_text("NO DEVICE DETECTED");
                lbl_badge_timer.set_css_classes(&["badge-disconnected"]);
                lbl_node_timer.set_text("Please connect keyboard");
                btn_fix_timer.set_visible(false);
            }
        }
        glib::ControlFlow::Continue
    });

    window.present();
}
