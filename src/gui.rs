//! Native Linux Graphical User Interface for FREE WOLF K8 Keyboard
//! Replicating the exact pixel positioning, layout, alignment, and styling of the Python Tkinter app.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};
use libadwaita as adw;
use adw::prelude::*;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, CheckButton, DropDown, Entry, EventControllerKey, FileChooserAction,
    FileChooserNative, Fixed, Grid, Image, Label, ListBox, ListBoxRow, Orientation, PasswordEntry,
    Picture, Scale, ScrolledWindow, Separator, Stack, StringList, TextBuffer, TextTag, TextView,
    Window, WrapMode,
};
use crate::config::{ConfigManager, Settings};
use crate::driver::{FreeWolfK8Driver, DeviceState};
use crate::i18n::{LANGUAGES, get_mode_name, t};
use crate::macro_mgr::{
    Macro, MacroAction, MacroManager, UinputPlayer, DELAY_RECORD, DELAY_NONE, DELAY_DEFAULT,
};
use crate::manual::{get_topics, ManualBlock, TopicInfo};
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
    current_macro_id: Option<u32>,
    is_recording: bool,
    record_last_instant: Option<Instant>,
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

fn key_to_info(key: &gtk4::gdk::Key, hw_code: u32) -> (String, u16) {
    let name = key.name().unwrap_or_else(|| glib::GString::from("Unknown"));
    let name_str = name.as_str();

    let (desc, evcode) = match name_str {
        "Escape" => ("Key Esc".to_string(), 1),
        "Return" => ("Key Enter".to_string(), 28),
        "BackSpace" => ("Key Backspace".to_string(), 14),
        "Tab" | "ISO_Left_Tab" => ("Key Tab".to_string(), 15),
        "space" => ("Key Space".to_string(), 57),
        "Caps_Lock" => ("Key Caps Lock".to_string(), 58),
        "Shift_L" => ("Key Shift".to_string(), 42),
        "Shift_R" => ("Key Right Shift".to_string(), 54),
        "Control_L" => ("Key Ctrl".to_string(), 29),
        "Control_R" => ("Key Right Ctrl".to_string(), 97),
        "Alt_L" => ("Key Alt".to_string(), 56),
        "Alt_R" => ("Key Right Alt".to_string(), 100),
        "Super_L" => ("Key Win".to_string(), 125),
        "Super_R" => ("Key Right Win".to_string(), 126),
        "Up" => ("Key Up".to_string(), 103),
        "Down" => ("Key Down".to_string(), 108),
        "Left" => ("Key Left".to_string(), 105),
        "Right" => ("Key Right".to_string(), 106),
        "Insert" => ("Key Insert".to_string(), 110),
        "Delete" => ("Key Delete".to_string(), 111),
        "Home" => ("Key Home".to_string(), 102),
        "End" => ("Key End".to_string(), 107),
        "Page_Up" | "Prior" => ("Key Page Up".to_string(), 104),
        "Page_Down" | "Next" => ("Key Page Down".to_string(), 109),
        "minus" | "underscore" => ("Key -".to_string(), 12),
        "equal" | "plus" => ("Key =".to_string(), 13),
        "bracketleft" | "braceleft" => ("Key [".to_string(), 26),
        "bracketright" | "braceright" => ("Key ]".to_string(), 27),
        "backslash" | "bar" => ("Key \\".to_string(), 43),
        "semicolon" | "colon" => ("Key ;".to_string(), 39),
        "apostrophe" | "quotedbl" => ("Key '".to_string(), 40),
        "grave" | "asciitilde" => ("Key `".to_string(), 41),
        "comma" | "less" => ("Key ,".to_string(), 51),
        "period" | "greater" => ("Key .".to_string(), 52),
        "slash" | "question" => ("Key /".to_string(), 53),
        s if s.starts_with('F') && s.len() <= 3 && s[1..].chars().all(|c| c.is_ascii_digit()) => {
            let num: u16 = s[1..].parse().unwrap_or(1);
            let code = match num {
                1..=10 => 58 + num,
                11 => 87,
                12 => 88,
                _ => 0,
            };
            (format!("Key F{}", num), code)
        }
        s if s.len() == 1 => {
            let ch = s.chars().next().unwrap().to_ascii_uppercase();
            let d = format!("Key {}", ch);
            let c = crate::macro_mgr::desc_to_evkey(&d);
            (d, c)
        }
        other => {
            let d = format!("Key {}", other);
            let c = crate::macro_mgr::desc_to_evkey(&d);
            (d, c)
        }
    };

    let final_evcode = if evcode != 0 {
        evcode
    } else if hw_code >= 8 {
        (hw_code - 8) as u16
    } else {
        hw_code as u16
    };

    (desc, final_evcode)
}

fn show_input_dialog<F: Fn(String) + 'static>(
    parent: &adw::ApplicationWindow,
    title: &str,
    prompt: &str,
    initial_text: &str,
    on_ok: F,
) {
    let dialog = Window::builder()
        .title(title)
        .transient_for(parent)
        .modal(true)
        .destroy_with_parent(true)
        .resizable(false)
        .build();
    dialog.set_default_size(320, 140);

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);

    let lbl = Label::new(Some(prompt));
    lbl.set_halign(Align::Start);
    lbl.add_css_class("field-title");
    vbox.append(&lbl);

    let entry = Entry::new();
    entry.set_text(initial_text);
    entry.add_css_class("entry-dark");
    vbox.append(&entry);

    let btn_box = GtkBox::new(Orientation::Horizontal, 8);
    btn_box.set_halign(Align::End);

    let btn_cancel = Button::with_label("Cancel");
    btn_cancel.add_css_class("secondary-btn");
    let btn_ok = Button::with_label("OK");
    btn_ok.add_css_class("accent-btn");

    btn_box.append(&btn_cancel);
    btn_box.append(&btn_ok);
    vbox.append(&btn_box);

    dialog.set_child(Some(&vbox));

    let dlg_cancel = dialog.clone();
    btn_cancel.connect_clicked(move |_| {
        dlg_cancel.close();
    });

    let dlg_ok = dialog.clone();
    let entry_ok = entry.clone();
    btn_ok.connect_clicked(move |_| {
        let val = entry_ok.text().to_string();
        on_ok(val);
        dlg_ok.close();
    });

    dialog.present();
}

fn show_confirm_dialog<F: Fn() + 'static>(
    parent: &adw::ApplicationWindow,
    title: &str,
    prompt: &str,
    on_confirm: F,
) {
    let dialog = Window::builder()
        .title(title)
        .transient_for(parent)
        .modal(true)
        .destroy_with_parent(true)
        .resizable(false)
        .build();
    dialog.set_default_size(320, 130);

    let vbox = GtkBox::new(Orientation::Vertical, 12);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);

    let lbl = Label::new(Some(prompt));
    lbl.set_wrap(true);
    lbl.set_halign(Align::Start);
    vbox.append(&lbl);

    let btn_box = GtkBox::new(Orientation::Horizontal, 8);
    btn_box.set_halign(Align::End);

    let btn_cancel = Button::with_label("Cancel");
    btn_cancel.add_css_class("secondary-btn");
    let btn_ok = Button::with_label("Delete");
    btn_ok.add_css_class("destructive-btn");

    btn_box.append(&btn_cancel);
    btn_box.append(&btn_ok);
    vbox.append(&btn_box);

    dialog.set_child(Some(&vbox));

    let dlg_cancel = dialog.clone();
    btn_cancel.connect_clicked(move |_| {
        dlg_cancel.close();
    });

    let dlg_ok = dialog.clone();
    btn_ok.connect_clicked(move |_| {
        on_confirm();
        dlg_ok.close();
    });

    dialog.present();
}

fn create_action_row(action: &MacroAction, lang: &str) -> ListBoxRow {
    let row = ListBoxRow::new();
    let hbox = GtkBox::new(Orientation::Horizontal, 0);
    hbox.add_css_class("macro-table-row");

    let lbl_desc = Label::new(Some(&action.desc));
    lbl_desc.set_hexpand(true);
    lbl_desc.set_halign(Align::Start);
    lbl_desc.set_margin_start(8);

    let act_text = if action.action == "Down" {
        t(lang, "action_down")
    } else {
        t(lang, "action_up")
    };
    let lbl_action = Label::new(Some(act_text));
    lbl_action.set_width_request(100);
    lbl_action.set_halign(Align::Center);

    let lbl_delay = Label::new(Some(&action.delay_ms.to_string()));
    lbl_delay.set_width_request(100);
    lbl_delay.set_halign(Align::Center);

    hbox.append(&lbl_desc);
    hbox.append(&lbl_action);
    hbox.append(&lbl_delay);
    row.set_child(Some(&hbox));
    row
}

fn populate_action_table(listbox: &ListBox, actions: &[MacroAction], lang: &str) {
    while let Some(child) = listbox.first_child() {
        listbox.remove(&child);
    }
    for action in actions {
        let row = create_action_row(action, lang);
        listbox.append(&row);
    }
}

fn populate_macro_list(listbox: &ListBox, macros: &[Macro], selected_id: Option<u32>) {
    while let Some(child) = listbox.first_child() {
        listbox.remove(&child);
    }
    let mut sel_row = None;
    for (idx, m) in macros.iter().enumerate() {
        let row = ListBoxRow::new();
        let lbl = Label::new(Some(&format!("  {}", m.name)));
        lbl.set_halign(Align::Start);
        lbl.set_margin_start(8);
        lbl.set_margin_top(4);
        lbl.set_margin_bottom(4);
        lbl.add_css_class("field-title");
        row.set_child(Some(&lbl));
        listbox.append(&row);

        if Some(m.id) == selected_id || (selected_id.is_none() && idx == 0) {
            sel_row = Some(row.clone());
        }
    }
    if let Some(r) = sel_row {
        listbox.select_row(Some(&r));
    }
}

fn setup_help_tags(buf: &TextBuffer) {
    let tag_table = buf.tag_table();

    let tag_h2 = TextTag::new(Some("h2"));
    tag_h2.set_property("foreground", "#027ad7");
    tag_h2.set_property("weight", 700);
    tag_h2.set_property("pixels-above-lines", 14);
    tag_h2.set_property("pixels-below-lines", 6);
    tag_h2.set_property("left-margin", 16);
    tag_h2.set_property("right-margin", 16);
    tag_table.add(&tag_h2);

    let tag_body = TextTag::new(Some("body"));
    tag_body.set_property("foreground", "#d8dee9");
    tag_body.set_property("pixels-above-lines", 3);
    tag_body.set_property("pixels-below-lines", 4);
    tag_body.set_property("left-margin", 16);
    tag_body.set_property("right-margin", 16);
    tag_table.add(&tag_body);

    let tag_bullet_dot = TextTag::new(Some("bullet_dot"));
    tag_bullet_dot.set_property("foreground", "#027ad7");
    tag_bullet_dot.set_property("weight", 700);
    tag_bullet_dot.set_property("left-margin", 16);
    tag_table.add(&tag_bullet_dot);

    let tag_bullet = TextTag::new(Some("bullet"));
    tag_bullet.set_property("foreground", "#d8dee9");
    tag_bullet.set_property("pixels-above-lines", 2);
    tag_bullet.set_property("pixels-below-lines", 3);
    tag_bullet.set_property("left-margin", 28);
    tag_bullet.set_property("right-margin", 16);
    tag_table.add(&tag_bullet);

    let tag_keycap = TextTag::new(Some("keycap"));
    tag_keycap.set_property("foreground", "#8ce10b");
    tag_keycap.set_property("background", "#1a2133");
    tag_keycap.set_property("weight", 700);
    tag_table.add(&tag_keycap);

    let tag_keydesc = TextTag::new(Some("keydesc"));
    tag_keydesc.set_property("foreground", "#d8dee9");
    tag_keydesc.set_property("pixels-above-lines", 3);
    tag_keydesc.set_property("pixels-below-lines", 3);
    tag_keydesc.set_property("left-margin", 16);
    tag_keydesc.set_property("right-margin", 16);
    tag_table.add(&tag_keydesc);

    let tag_table_lbl = TextTag::new(Some("table_lbl"));
    tag_table_lbl.set_property("foreground", "#ffffff");
    tag_table_lbl.set_property("weight", 700);
    tag_table_lbl.set_property("left-margin", 16);
    tag_table.add(&tag_table_lbl);

    let tag_table_val = TextTag::new(Some("table_val"));
    tag_table_val.set_property("foreground", "#d8dee9");
    tag_table_val.set_property("left-margin", 16);
    tag_table_val.set_property("right-margin", 16);
    tag_table.add(&tag_table_val);

    let tag_code = TextTag::new(Some("code"));
    tag_code.set_property("font", "Monospace 9");
    tag_code.set_property("foreground", "#61afef");
    tag_code.set_property("background", "#161924");
    tag_code.set_property("pixels-above-lines", 4);
    tag_code.set_property("pixels-below-lines", 4);
    tag_code.set_property("left-margin", 24);
    tag_code.set_property("right-margin", 24);
    tag_table.add(&tag_code);

    let tag_tip = TextTag::new(Some("tip"));
    tag_tip.set_property("foreground", "#ffb900");
    tag_tip.set_property("background", "#222638");
    tag_tip.set_property("pixels-above-lines", 6);
    tag_tip.set_property("pixels-below-lines", 6);
    tag_tip.set_property("left-margin", 20);
    tag_tip.set_property("right-margin", 20);
    tag_table.add(&tag_tip);
}

fn render_topic_content(buf: &TextBuffer, topic: &TopicInfo, pic: Option<&Picture>) {
    buf.set_text("");
    if let Some(p) = pic {
        p.set_visible(topic.id == 0);
    }
    let mut it = buf.end_iter();
    for block in topic.blocks {
        match block {
            ManualBlock::Title(_) => {}
            ManualBlock::H2(h) => {
                buf.insert_with_tags_by_name(&mut it, &format!("{}\n", h), &["h2"]);
            }
            ManualBlock::Body(b) => {
                buf.insert_with_tags_by_name(&mut it, &format!("{}\n", b), &["body"]);
            }
            ManualBlock::Bullet(b) => {
                buf.insert_with_tags_by_name(&mut it, "• ", &["bullet_dot"]);
                buf.insert_with_tags_by_name(&mut it, &format!("{}\n", b), &["bullet"]);
            }
            ManualBlock::Key(k, desc) => {
                buf.insert_with_tags_by_name(&mut it, &format!(" [ {:^16} ] ", k), &["keycap"]);
                buf.insert_with_tags_by_name(&mut it, &format!("  {}\n", desc), &["keydesc"]);
            }
            ManualBlock::TableRow(lbl, val) => {
                buf.insert_with_tags_by_name(&mut it, &format!("{:<26} ", lbl), &["table_lbl"]);
                buf.insert_with_tags_by_name(&mut it, &format!(": {}\n", val), &["table_val"]);
            }
            ManualBlock::Code(c) => {
                buf.insert_with_tags_by_name(&mut it, &format!("   {}\n", c), &["code"]);
            }
            ManualBlock::Tip(t) => {
                buf.insert_with_tags_by_name(&mut it, &format!(" 💡 Tip: {}\n", t), &["tip"]);
            }
        }
    }
}

fn build_ui(app: &adw::Application) {
    // Force Dark Theme
    let style_manager = adw::StyleManager::default();
    style_manager.set_color_scheme(adw::ColorScheme::ForceDark);

    // Apply custom Argonaut GNOME theme styling matching the Python Tkinter app EXACTLY
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(
        r#"
         window {
             background-color: #0e1019;
         }
         headerbar {
             background-color: #0e1019;
             border-bottom: 1px solid #181c2e;
             color: #ffffff;
             min-height: 38px;
         }
         headerbar .title {
             font-weight: bold;
             font-size: 12px;
             color: #ffffff;
         }
         .sidebar-bg {
             background-color: #0e1019;
             border-right: 1px solid #181c2e;
         }
         .nav-btn {
             background-color: transparent;
             border: none;
             border-radius: 0;
             min-width: 48px;
             min-height: 44px;
             padding: 0;
         }
         .nav-btn:hover {
             background-color: #161b2c;
         }
         .nav-btn-active {
             background-color: #132238;
             border-left: 3px solid #027ad7;
         }
         .card-panel {
             background-color: #151829;
             border: 1px solid #232840;
             border-radius: 0;
             padding: 0;
         }
         .card-title {
             font-size: 11px;
             font-weight: bold;
             color: #ffffff;
         }
         .field-title {
             font-size: 10px;
             font-weight: bold;
             color: #ffffff;
         }
         .badge-connected {
             background-color: #101321;
             color: #8ce10b;
             border: 1px solid #8ce10b;
             font-weight: bold;
             font-size: 11px;
             padding: 5px 6px;
         }
         .badge-warning {
             background-color: #101321;
             color: #ffb900;
             border: 1px solid #ffb900;
             font-weight: bold;
             font-size: 11px;
             padding: 5px 6px;
         }
         .badge-disconnected {
             background-color: #101321;
             color: #7e88a0;
             border: 1px solid #232840;
             font-weight: bold;
             font-size: 11px;
             padding: 5px 6px;
         }
         .status-circle-ok {
             background-color: #8ce10b;
             border-radius: 5px;
             min-width: 10px;
             min-height: 10px;
         }
         .status-circle-warn {
             background-color: #ffb900;
             border-radius: 5px;
             min-width: 10px;
             min-height: 10px;
         }
         .status-circle-err {
             background-color: #ED5F5D;
             border-radius: 5px;
             min-width: 10px;
             min-height: 10px;
         }
         .accent-btn {
             background-color: #027ad7;
             color: #ffffff;
             font-weight: bold;
             border: none;
             padding: 4px 10px;
             border-radius: 2px;
             font-size: 9px;
         }
         .accent-btn:hover {
             background-color: #1a8fe5;
         }
         .secondary-btn {
             background-color: #1a1e32;
             color: #ffffff;
             border: 1px solid #262c45;
             border-radius: 2px;
             font-size: 9px;
             font-weight: bold;
             padding: 5px 8px;
         }
         .secondary-btn:hover {
             background-color: #222842;
         }
         .destructive-btn {
             background-color: #ED5F5D;
             color: #ffffff;
             font-weight: bold;
             border: none;
             padding: 4px 10px;
             border-radius: 2px;
             font-size: 9px;
         }
         .destructive-btn:hover {
             background-color: #f17876;
         }
         .small-icon-btn {
             background: transparent;
             border: none;
             padding: 2px 4px;
             border-radius: 2px;
         }
         .small-icon-btn:hover {
             background-color: #222842;
         }
         .macro-table-header {
             font-size: 10px;
             font-weight: bold;
             color: #7e88a0;
             padding: 4px 8px;
             border-bottom: 1px solid #232840;
         }
         .macro-table-row {
             padding: 4px 8px;
             font-size: 10px;
             border-bottom: 1px solid #1a1e30;
         }
         .macro-table-row:selected {
             background-color: #132238;
             color: #027ad7;
         }
         .entry-dark {
             background-color: #0e1019;
             color: #ffffff;
             border: 1px solid #262c45;
             border-radius: 2px;
             font-size: 10px;
             padding: 2px 4px;
         }
         .muted-text {
             font-size: 9px;
             color: #7e88a0;
         }
         .topic-item-btn {
             background: transparent;
             border: none;
             border-radius: 0;
             padding: 7px 10px;
             color: #d8dee9;
             font-size: 10px;
         }
         .topic-item-btn:hover {
             background-color: #1c2035;
             color: #ffffff;
         }
         .topic-item-active {
             background-color: #132238;
             color: #027ad7;
             border-left: 3px solid #027ad7;
             font-weight: bold;
         }
         scale highlight {
             background-color: #027ad7;
         }
         scale slider {
             background-color: #ffffff;
             min-width: 16px;
             min-height: 16px;
         }
         scale trough {
             background-color: #101321;
             border: 1px solid #232840;
             min-height: 6px;
         }
        "#,
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

    let initial_macro_mgr = MacroManager::load();
    let initial_macro_id = initial_macro_mgr.macros.first().map(|m| m.id);

    let state = Rc::new(RefCell::new(AppState {
        settings: initial_settings,
        current_mode: initial_mode,
        macro_mgr: initial_macro_mgr,
        current_macro_id: initial_macro_id,
        is_recording: false,
        record_last_instant: None,
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
    let macro_listbox = ListBox::new();
    macro_listbox.add_css_class("card-panel");
    macro_scrolled.set_child(Some(&macro_listbox));
    card_macro_left.append(&macro_scrolled);
    stack_left.add_named(&card_macro_left, Some("macro"));

    // --- Right Card: Macro Actions & Controls (w=530, h=494) ---
    let card_macro_right = GtkBox::new(Orientation::Vertical, 8);
    card_macro_right.set_size_request(530, 494);

    // Top Card (h=155)
    let macro_top_card = GtkBox::new(Orientation::Horizontal, 0);
    macro_top_card.set_size_request(530, 155);
    macro_top_card.add_css_class("card-panel");

    // Left half: 3x2 Grid of action buttons
    let grid_actions = Grid::new();
    grid_actions.set_row_spacing(6);
    grid_actions.set_column_spacing(6);
    grid_actions.set_margin_start(16);
    grid_actions.set_margin_end(12);
    grid_actions.set_margin_top(14);
    grid_actions.set_margin_bottom(14);

    let btn_macro_new = Button::with_label(t(&cur_lang, "btn_new"));
    btn_macro_new.add_css_class("secondary-btn");
    btn_macro_new.set_size_request(86, 28);
    let btn_macro_del = Button::with_label(t(&cur_lang, "btn_delete"));
    btn_macro_del.add_css_class("secondary-btn");
    btn_macro_del.set_size_request(86, 28);
    let btn_macro_copy = Button::with_label(t(&cur_lang, "btn_copy"));
    btn_macro_copy.add_css_class("secondary-btn");
    btn_macro_copy.set_size_request(86, 28);
    let btn_macro_rename = Button::with_label(t(&cur_lang, "btn_rename"));
    btn_macro_rename.add_css_class("secondary-btn");
    btn_macro_rename.set_size_request(86, 28);
    let btn_macro_import = Button::with_label(t(&cur_lang, "btn_import"));
    btn_macro_import.add_css_class("secondary-btn");
    btn_macro_import.set_size_request(86, 28);
    let btn_macro_export = Button::with_label(t(&cur_lang, "btn_export"));
    btn_macro_export.add_css_class("secondary-btn");
    btn_macro_export.set_size_request(86, 28);

    grid_actions.attach(&btn_macro_new, 0, 0, 1, 1);
    grid_actions.attach(&btn_macro_del, 1, 0, 1, 1);
    grid_actions.attach(&btn_macro_copy, 0, 1, 1, 1);
    grid_actions.attach(&btn_macro_rename, 1, 1, 1, 1);
    grid_actions.attach(&btn_macro_import, 0, 2, 1, 1);
    grid_actions.attach(&btn_macro_export, 1, 2, 1, 1);
    macro_top_card.append(&grid_actions);

    let sep_top_mid = Separator::new(Orientation::Vertical);
    sep_top_mid.set_margin_top(8);
    sep_top_mid.set_margin_bottom(8);
    macro_top_card.append(&sep_top_mid);

    // Right half: Repeat Time and Delay Mode
    let settings_box = GtkBox::new(Orientation::Vertical, 8);
    settings_box.set_hexpand(true);
    settings_box.set_margin_start(16);
    settings_box.set_margin_end(16);
    settings_box.set_margin_top(14);
    settings_box.set_margin_bottom(14);

    let row_rep = GtkBox::new(Orientation::Horizontal, 8);
    let lbl_rep = Label::new(Some(t(&cur_lang, "repeat_time")));
    lbl_rep.add_css_class("field-title");
    let entry_repeat = Entry::new();
    entry_repeat.set_text("1");
    entry_repeat.set_width_chars(5);
    entry_repeat.add_css_class("entry-dark");
    let lbl_rep_unit = Label::new(Some("(1 - 9999)"));
    lbl_rep_unit.add_css_class("muted-text");
    row_rep.append(&lbl_rep);
    row_rep.append(&entry_repeat);
    row_rep.append(&lbl_rep_unit);
    settings_box.append(&row_rep);

    let delay_box = GtkBox::new(Orientation::Vertical, 4);
    let lbl_delay_title = Label::new(Some(t(&cur_lang, "delay_mode")));
    lbl_delay_title.add_css_class("field-title");
    lbl_delay_title.set_halign(Align::Start);
    delay_box.append(&lbl_delay_title);

    let rb_delay_record = CheckButton::with_label(t(&cur_lang, "delay_record"));
    let rb_delay_none = CheckButton::with_label(t(&cur_lang, "delay_none"));
    rb_delay_none.set_group(Some(&rb_delay_record));

    let row_def = GtkBox::new(Orientation::Horizontal, 6);
    let rb_delay_default = CheckButton::with_label(t(&cur_lang, "delay_default"));
    rb_delay_default.set_group(Some(&rb_delay_record));
    rb_delay_default.set_active(true);
    let entry_default_delay = Entry::new();
    entry_default_delay.set_text("10");
    entry_default_delay.set_width_chars(4);
    entry_default_delay.add_css_class("entry-dark");
    let lbl_ms = Label::new(Some(t(&cur_lang, "delay_ms")));
    lbl_ms.add_css_class("field-title");
    row_def.append(&rb_delay_default);
    row_def.append(&entry_default_delay);
    row_def.append(&lbl_ms);

    delay_box.append(&rb_delay_record);
    delay_box.append(&rb_delay_none);
    delay_box.append(&row_def);
    settings_box.append(&delay_box);

    macro_top_card.append(&settings_box);
    card_macro_right.append(&macro_top_card);

    // Bottom Card (h=331)
    let macro_bottom_card = GtkBox::new(Orientation::Vertical, 6);
    macro_bottom_card.set_size_request(530, 331);
    macro_bottom_card.add_css_class("card-panel");

    let hdr_row = GtkBox::new(Orientation::Horizontal, 8);
    hdr_row.set_margin_start(16);
    hdr_row.set_margin_end(16);
    hdr_row.set_margin_top(8);
    let lbl_rec_title = Label::new(Some(t(&cur_lang, "macro_record")));
    lbl_rec_title.add_css_class("card-title");
    lbl_rec_title.set_hexpand(true);
    lbl_rec_title.set_halign(Align::Start);
    hdr_row.append(&lbl_rec_title);

    let btns_sub = GtkBox::new(Orientation::Horizontal, 4);
    let btn_action_del = Button::new();
    if let Some(p) = get_asset_path("assets/icon/record_del.png") {
        let pic = Picture::for_filename(p);
        pic.set_size_request(20, 20);
        btn_action_del.set_child(Some(&pic));
    } else {
        btn_action_del.set_label("✕");
    }
    btn_action_del.add_css_class("small-icon-btn");
    btn_action_del.set_tooltip_text(Some("Delete Action"));

    let btn_action_up = Button::new();
    if let Some(p) = get_asset_path("assets/icon/record_up.png") {
        let pic = Picture::for_filename(p);
        pic.set_size_request(20, 20);
        btn_action_up.set_child(Some(&pic));
    } else {
        btn_action_up.set_label("▲");
    }
    btn_action_up.add_css_class("small-icon-btn");
    btn_action_up.set_tooltip_text(Some("Move Up"));

    let btn_action_down = Button::new();
    if let Some(p) = get_asset_path("assets/icon/record_down.png") {
        let pic = Picture::for_filename(p);
        pic.set_size_request(20, 20);
        btn_action_down.set_child(Some(&pic));
    } else {
        btn_action_down.set_label("▼");
    }
    btn_action_down.add_css_class("small-icon-btn");
    btn_action_down.set_tooltip_text(Some("Move Down"));

    btns_sub.append(&btn_action_del);
    btns_sub.append(&btn_action_up);
    btns_sub.append(&btn_action_down);
    hdr_row.append(&btns_sub);
    macro_bottom_card.append(&hdr_row);

    // Table Column Headers
    let tbl_hdr = GtkBox::new(Orientation::Horizontal, 0);
    tbl_hdr.add_css_class("macro-table-header");
    tbl_hdr.set_margin_start(16);
    tbl_hdr.set_margin_end(16);
    let lbl_th_desc = Label::new(Some(t(&cur_lang, "col_desc")));
    lbl_th_desc.set_hexpand(true);
    lbl_th_desc.set_halign(Align::Start);
    lbl_th_desc.set_margin_start(8);
    let lbl_th_act = Label::new(Some(t(&cur_lang, "col_action")));
    lbl_th_act.set_width_request(100);
    lbl_th_act.set_halign(Align::Center);
    let lbl_th_del = Label::new(Some(t(&cur_lang, "col_delay")));
    lbl_th_del.set_width_request(100);
    lbl_th_del.set_halign(Align::Center);
    tbl_hdr.append(&lbl_th_desc);
    tbl_hdr.append(&lbl_th_act);
    tbl_hdr.append(&lbl_th_del);
    macro_bottom_card.append(&tbl_hdr);

    // Scrolled Table
    let act_scrolled = ScrolledWindow::new();
    act_scrolled.set_vexpand(true);
    act_scrolled.set_margin_start(16);
    act_scrolled.set_margin_end(16);
    let action_listbox = ListBox::new();
    action_listbox.add_css_class("card-panel");
    act_scrolled.set_child(Some(&action_listbox));
    macro_bottom_card.append(&act_scrolled);

    // Bottom Controls Row
    let bot_row = GtkBox::new(Orientation::Horizontal, 8);
    bot_row.set_margin_start(16);
    bot_row.set_margin_end(16);
    bot_row.set_margin_top(4);
    bot_row.set_margin_bottom(8);

    let lbl_record_hint = Label::new(Some("Ready"));
    lbl_record_hint.set_wrap(true);
    lbl_record_hint.set_hexpand(true);
    lbl_record_hint.set_halign(Align::Start);
    lbl_record_hint.add_css_class("muted-text");
    bot_row.append(&lbl_record_hint);

    let btn_macro_record = Button::with_label(t(&cur_lang, "btn_record"));
    btn_macro_record.add_css_class("secondary-btn");
    btn_macro_record.set_size_request(80, 28);

    let btn_macro_play = Button::with_label(t(&cur_lang, "btn_play"));
    btn_macro_play.add_css_class("secondary-btn");
    btn_macro_play.set_size_request(80, 28);

    let btn_macro_save = Button::with_label(t(&cur_lang, "btn_save"));
    btn_macro_save.add_css_class("accent-btn");
    btn_macro_save.set_size_request(80, 28);

    bot_row.append(&btn_macro_record);
    bot_row.append(&btn_macro_play);
    bot_row.append(&btn_macro_save);
    macro_bottom_card.append(&bot_row);

    card_macro_right.append(&macro_bottom_card);
    stack_right.add_named(&card_macro_right, Some("macro"));

    // Populate initial macro list and table
    let init_mid = state.borrow().current_macro_id;
    let init_macros = state.borrow().macro_mgr.macros.clone();
    populate_macro_list(&macro_listbox, &init_macros, init_mid);
    if let Some(mid) = init_mid {
        if let Some(m) = state.borrow().macro_mgr.get_macro(mid) {
            entry_repeat.set_text(&m.repeat_time.to_string());
            entry_default_delay.set_text(&m.default_delay.to_string());
            match m.delay_type {
                DELAY_RECORD => rb_delay_record.set_active(true),
                DELAY_NONE => rb_delay_none.set_active(true),
                _ => rb_delay_default.set_active(true),
            }
            populate_action_table(&action_listbox, &m.actions, &cur_lang);
        }
    }

    // Macro list selection handler
    let state_msel = state.clone();
    let entry_rep_msel = entry_repeat.clone();
    let entry_def_msel = entry_default_delay.clone();
    let rb_dr_msel = rb_delay_record.clone();
    let rb_dn_msel = rb_delay_none.clone();
    let rb_dd_msel = rb_delay_default.clone();
    let act_lb_msel = action_listbox.clone();
    let btn_del_msel = btn_macro_del.clone();
    let btn_cp_msel = btn_macro_copy.clone();
    let btn_ren_msel = btn_macro_rename.clone();
    let btn_exp_msel = btn_macro_export.clone();
    macro_listbox.connect_row_selected(move |_, row_opt| {
        if let Some(row) = row_opt {
            let idx = row.index() as usize;
            let mut st = state_msel.borrow_mut();
            let macro_opt = st.macro_mgr.macros.get(idx).cloned();
            if let Some(m) = macro_opt {
                let mid = m.id;
                st.current_macro_id = Some(mid);
                entry_rep_msel.set_text(&m.repeat_time.to_string());
                entry_def_msel.set_text(&m.default_delay.to_string());
                match m.delay_type {
                    DELAY_RECORD => rb_dr_msel.set_active(true),
                    DELAY_NONE => rb_dn_msel.set_active(true),
                    _ => rb_dd_msel.set_active(true),
                }
                populate_action_table(&act_lb_msel, &m.actions, &st.settings.language);
                btn_del_msel.set_sensitive(true);
                btn_cp_msel.set_sensitive(true);
                btn_ren_msel.set_sensitive(true);
                btn_exp_msel.set_sensitive(true);
            }
        } else {
            btn_del_msel.set_sensitive(false);
            btn_cp_msel.set_sensitive(false);
            btn_ren_msel.set_sensitive(false);
            btn_exp_msel.set_sensitive(false);
        }
    });

    // Action Table Double-Click Delay Editing
    let win_edit_act = window.clone();
    let state_edit_act = state.clone();
    let act_lb_edit_act = action_listbox.clone();
    action_listbox.connect_row_activated(move |_, row| {
        let idx = row.index() as usize;
        let st = state_edit_act.borrow();
        if let Some(cur_id) = st.current_macro_id {
            if let Some(m) = st.macro_mgr.get_macro(cur_id) {
                if let Some(act) = m.actions.get(idx) {
                    let desc = act.desc.clone();
                    let cur_delay = act.delay_ms.to_string();
                    let st_inner = state_edit_act.clone();
                    let alb_inner = act_lb_edit_act.clone();
                    show_input_dialog(
                        &win_edit_act,
                        "Edit Delay",
                        &format!("Enter delay (ms) for {}:", desc),
                        &cur_delay,
                        move |val| {
                            if let Ok(d) = val.trim().parse::<u32>() {
                                let mut stm = st_inner.borrow_mut();
                                stm.macro_mgr.update_action_delay(cur_id, idx, d);
                                if let Some(m_updated) = stm.macro_mgr.get_macro(cur_id) {
                                    let actions = m_updated.actions.clone();
                                    let lang = stm.settings.language.clone();
                                    drop(stm);
                                    populate_action_table(&alb_inner, &actions, &lang);
                                    if let Some(r) = alb_inner.row_at_index(idx as i32) {
                                        alb_inner.select_row(Some(&r));
                                    }
                                }
                            }
                        },
                    );
                }
            }
        }
    });

    // Action Buttons: Delete, Move Up, Move Down
    let state_adel = state.clone();
    let act_lb_adel = action_listbox.clone();
    btn_action_del.connect_clicked(move |_| {
        let mut st = state_adel.borrow_mut();
        if let Some(cur_id) = st.current_macro_id {
            if let Some(row) = act_lb_adel.selected_row() {
                let idx = row.index() as usize;
                st.macro_mgr.delete_action(cur_id, idx);
                if let Some(m) = st.macro_mgr.get_macro(cur_id) {
                    let actions = m.actions.clone();
                    let lang = st.settings.language.clone();
                    drop(st);
                    populate_action_table(&act_lb_adel, &actions, &lang);
                    let new_idx = idx.min(actions.len().saturating_sub(1));
                    if let Some(new_r) = act_lb_adel.row_at_index(new_idx as i32) {
                        act_lb_adel.select_row(Some(&new_r));
                    }
                }
            }
        }
    });

    let state_aup = state.clone();
    let act_lb_aup = action_listbox.clone();
    btn_action_up.connect_clicked(move |_| {
        let mut st = state_aup.borrow_mut();
        if let Some(cur_id) = st.current_macro_id {
            if let Some(row) = act_lb_aup.selected_row() {
                let idx = row.index() as usize;
                if idx > 0 {
                    st.macro_mgr.reorder_action(cur_id, idx, idx - 1);
                    if let Some(m) = st.macro_mgr.get_macro(cur_id) {
                        let actions = m.actions.clone();
                        let lang = st.settings.language.clone();
                        drop(st);
                        populate_action_table(&act_lb_aup, &actions, &lang);
                        if let Some(new_r) = act_lb_aup.row_at_index((idx - 1) as i32) {
                            act_lb_aup.select_row(Some(&new_r));
                        }
                    }
                }
            }
        }
    });

    let state_adown = state.clone();
    let act_lb_adown = action_listbox.clone();
    btn_action_down.connect_clicked(move |_| {
        let mut st = state_adown.borrow_mut();
        if let Some(cur_id) = st.current_macro_id {
            if let Some(row) = act_lb_adown.selected_row() {
                let idx = row.index() as usize;
                if let Some(m) = st.macro_mgr.get_macro(cur_id) {
                    if idx + 1 < m.actions.len() {
                        st.macro_mgr.reorder_action(cur_id, idx, idx + 1);
                        let actions = st.macro_mgr.get_macro(cur_id).unwrap().actions.clone();
                        let lang = st.settings.language.clone();
                        drop(st);
                        populate_action_table(&act_lb_adown, &actions, &lang);
                        if let Some(new_r) = act_lb_adown.row_at_index((idx + 1) as i32) {
                            act_lb_adown.select_row(Some(&new_r));
                        }
                    }
                }
            }
        }
    });

    // Top Macro Management Buttons: New, Delete, Copy, Rename, Import, Export
    let state_mnew = state.clone();
    let m_lb_new = macro_listbox.clone();
    let act_lb_new = action_listbox.clone();
    let cur_lang_new = cur_lang.clone();
    btn_macro_new.connect_clicked(move |_| {
        let mut st = state_mnew.borrow_mut();
        let next_id = st.macro_mgr.macros.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        let name = format!("Macro {}", next_id);
        let new_m = st.macro_mgr.add_macro(&name);
        let mid = new_m.id;
        st.current_macro_id = Some(mid);
        let macros = st.macro_mgr.macros.clone();
        drop(st);
        populate_macro_list(&m_lb_new, &macros, Some(mid));
        populate_action_table(&act_lb_new, &[], &cur_lang_new);
    });

    let win_del = window.clone();
    let state_mdel = state.clone();
    let m_lb_del = macro_listbox.clone();
    let act_lb_del_ref = action_listbox.clone();
    let cur_lang_mdel = cur_lang.clone();
    btn_macro_del.connect_clicked(move |_| {
        let st = state_mdel.borrow();
        if let Some(cur_id) = st.current_macro_id {
            if let Some(m) = st.macro_mgr.get_macro(cur_id) {
                let mname = m.name.clone();
                let st_inner = state_mdel.clone();
                let mlb_inner = m_lb_del.clone();
                let alb_inner = act_lb_del_ref.clone();
                let lang_inner = cur_lang_mdel.clone();
                show_confirm_dialog(
                    &win_del,
                    "Delete Macro",
                    &format!("Are you sure you want to delete '{}'?", mname),
                    move || {
                        let mut stm = st_inner.borrow_mut();
                        stm.macro_mgr.delete_macro(cur_id);
                        stm.current_macro_id = stm.macro_mgr.macros.first().map(|m| m.id);
                        let new_sel = stm.current_macro_id;
                        let macros = stm.macro_mgr.macros.clone();
                        let actions = stm.macro_mgr.macros.first().map(|m| m.actions.clone()).unwrap_or_default();
                        drop(stm);
                        populate_macro_list(&mlb_inner, &macros, new_sel);
                        populate_action_table(&alb_inner, &actions, &lang_inner);
                    },
                );
            }
        }
    });

    let state_mcopy = state.clone();
    let m_lb_cp = macro_listbox.clone();
    let act_lb_cp = action_listbox.clone();
    let cur_lang_cp = cur_lang.clone();
    btn_macro_copy.connect_clicked(move |_| {
        let mut st = state_mcopy.borrow_mut();
        if let Some(cur_id) = st.current_macro_id {
            if let Some(new_id) = st.macro_mgr.copy_macro(cur_id) {
                st.current_macro_id = Some(new_id);
                let macros = st.macro_mgr.macros.clone();
                let actions = st.macro_mgr.get_macro(new_id).map(|m| m.actions.clone()).unwrap_or_default();
                drop(st);
                populate_macro_list(&m_lb_cp, &macros, Some(new_id));
                populate_action_table(&act_lb_cp, &actions, &cur_lang_cp);
            }
        }
    });

    let win_ren = window.clone();
    let state_mren = state.clone();
    let m_lb_ren = macro_listbox.clone();
    btn_macro_rename.connect_clicked(move |_| {
        let st = state_mren.borrow();
        if let Some(cur_id) = st.current_macro_id {
            if let Some(m) = st.macro_mgr.get_macro(cur_id) {
                let mname = m.name.clone();
                let st_inner = state_mren.clone();
                let mlb_inner = m_lb_ren.clone();
                show_input_dialog(
                    &win_ren,
                    "Rename Macro",
                    "Enter new name:",
                    &mname,
                    move |new_name| {
                        if !new_name.trim().is_empty() {
                            let mut stm = st_inner.borrow_mut();
                            stm.macro_mgr.rename_macro(cur_id, new_name.trim());
                            let macros = stm.macro_mgr.macros.clone();
                            drop(stm);
                            populate_macro_list(&mlb_inner, &macros, Some(cur_id));
                        }
                    },
                );
            }
        }
    });

    let win_imp = window.clone();
    let state_mimp = state.clone();
    let m_lb_imp = macro_listbox.clone();
    let act_lb_imp = action_listbox.clone();
    let cur_lang_imp = cur_lang.clone();
    let lbl_hint_imp = lbl_record_hint.clone();
    btn_macro_import.connect_clicked(move |_| {
        let chooser = FileChooserNative::new(
            Some("Import Macro JSON"),
            Some(&win_imp),
            FileChooserAction::Open,
            Some("Import"),
            Some("Cancel"),
        );
        let filter = gtk4::FileFilter::new();
        filter.set_name(Some("JSON Files (*.json)"));
        filter.add_pattern("*.json");
        chooser.add_filter(&filter);

        let st_inner = state_mimp.clone();
        let mlb_inner = m_lb_imp.clone();
        let alb_inner = act_lb_imp.clone();
        let lang_inner = cur_lang_imp.clone();
        let hint_inner = lbl_hint_imp.clone();
        chooser.connect_response(move |d, resp| {
            if resp == gtk4::ResponseType::Accept {
                if let Some(file) = d.file() {
                    if let Some(path) = file.path() {
                        let mut stm = st_inner.borrow_mut();
                        match stm.macro_mgr.import_macro(&path) {
                            Ok(new_id) => {
                                stm.current_macro_id = Some(new_id);
                                let macros = stm.macro_mgr.macros.clone();
                                let actions = stm.macro_mgr.get_macro(new_id).map(|m| m.actions.clone()).unwrap_or_default();
                                let name = stm.macro_mgr.get_macro(new_id).map(|m| m.name.clone()).unwrap_or_default();
                                drop(stm);
                                populate_macro_list(&mlb_inner, &macros, Some(new_id));
                                populate_action_table(&alb_inner, &actions, &lang_inner);
                                hint_inner.set_text(&format!("Imported '{}'.", name));
                            }
                            Err(e) => {
                                hint_inner.set_text(&format!("Import failed: {}", e));
                            }
                        }
                    }
                }
            }
        });
        chooser.show();
    });

    let win_exp = window.clone();
    let state_mexp = state.clone();
    let lbl_hint_exp = lbl_record_hint.clone();
    btn_macro_export.connect_clicked(move |_| {
        let st = state_mexp.borrow();
        if let Some(cur_id) = st.current_macro_id {
            if let Some(m) = st.macro_mgr.get_macro(cur_id) {
                let mname = m.name.clone();
                drop(st);

                let chooser = FileChooserNative::new(
                    Some("Export Macro JSON"),
                    Some(&win_exp),
                    FileChooserAction::Save,
                    Some("Export"),
                    Some("Cancel"),
                );
                chooser.set_current_name(&format!("{}.json", mname));
                let filter = gtk4::FileFilter::new();
                filter.set_name(Some("JSON Files (*.json)"));
                filter.add_pattern("*.json");
                chooser.add_filter(&filter);

                let st_inner = state_mexp.clone();
                let hint_inner = lbl_hint_exp.clone();
                chooser.connect_response(move |d, resp| {
                    if resp == gtk4::ResponseType::Accept {
                        if let Some(file) = d.file() {
                            if let Some(path) = file.path() {
                                let stm = st_inner.borrow();
                                match stm.macro_mgr.export_macro(cur_id, &path) {
                                    Ok(()) => {
                                        hint_inner.set_text(&format!("Exported to '{}'.", path.display()));
                                    }
                                    Err(e) => {
                                        hint_inner.set_text(&format!("Export failed: {}", e));
                                    }
                                }
                            }
                        }
                    }
                });
                chooser.show();
            }
        }
    });

    // Repeat Time and Delay Mode editing callbacks
    let state_rep = state.clone();
    entry_repeat.connect_changed(move |entry| {
        if let Ok(rep) = entry.text().trim().parse::<u32>() {
            let mut st = state_rep.borrow_mut();
            if let Some(cur_id) = st.current_macro_id {
                if let Some(m) = st.macro_mgr.get_macro_mut(cur_id) {
                    m.repeat_time = rep.max(1);
                }
            }
        }
    });

    let state_defd = state.clone();
    let act_lb_defd = action_listbox.clone();
    entry_default_delay.connect_changed(move |entry| {
        if let Ok(def_d) = entry.text().trim().parse::<u32>() {
            let mut st = state_defd.borrow_mut();
            if let Some(cur_id) = st.current_macro_id {
                let is_default_mode = st.macro_mgr.get_macro(cur_id).map(|m| m.delay_type == DELAY_DEFAULT).unwrap_or(false);
                if let Some(m) = st.macro_mgr.get_macro_mut(cur_id) {
                    m.default_delay = def_d;
                    if is_default_mode {
                        for a in &mut m.actions {
                            a.delay_ms = def_d;
                        }
                    }
                }
                if is_default_mode {
                    let actions = st.macro_mgr.get_macro(cur_id).unwrap().actions.clone();
                    let lang = st.settings.language.clone();
                    drop(st);
                    populate_action_table(&act_lb_defd, &actions, &lang);
                }
            }
        }
    });

    let state_rb = state.clone();
    let act_lb_rb = action_listbox.clone();
    let entry_def_rb = entry_default_delay.clone();
    let rb_dr_cb = rb_delay_record.clone();
    let rb_dn_cb = rb_delay_none.clone();

    let on_delay_toggled = Rc::new(move || {
        let mut st = state_rb.borrow_mut();
        if let Some(cur_id) = st.current_macro_id {
            let mode = if rb_dr_cb.is_active() {
                DELAY_RECORD
            } else if rb_dn_cb.is_active() {
                DELAY_NONE
            } else {
                DELAY_DEFAULT
            };
            let def_d = entry_def_rb.text().trim().parse::<u32>().unwrap_or(10);
            if let Some(m) = st.macro_mgr.get_macro_mut(cur_id) {
                m.delay_type = mode;
                if mode == DELAY_NONE {
                    for a in &mut m.actions {
                        a.delay_ms = 0;
                    }
                } else if mode == DELAY_DEFAULT {
                    for a in &mut m.actions {
                        a.delay_ms = def_d;
                    }
                }
            }
            if mode != DELAY_RECORD {
                let actions = st.macro_mgr.get_macro(cur_id).unwrap().actions.clone();
                let lang = st.settings.language.clone();
                drop(st);
                populate_action_table(&act_lb_rb, &actions, &lang);
            }
        }
    });

    let odt1 = on_delay_toggled.clone();
    rb_delay_record.connect_toggled(move |_| odt1());
    let odt2 = on_delay_toggled.clone();
    rb_delay_none.connect_toggled(move |_| odt2());
    let odt3 = on_delay_toggled.clone();
    rb_delay_default.connect_toggled(move |_| odt3());

    // Record / Play / Save callbacks
    let state_rec = state.clone();
    let btn_rec_ref = btn_macro_record.clone();
    let lbl_hint_rec = lbl_record_hint.clone();
    let cur_lang_rec = cur_lang.clone();
    btn_macro_record.connect_clicked(move |_| {
        let mut st = state_rec.borrow_mut();
        st.is_recording = !st.is_recording;
        if st.is_recording {
            st.record_last_instant = None;
            btn_rec_ref.set_label(t(&cur_lang_rec, "btn_stop"));
            btn_rec_ref.remove_css_class("secondary-btn");
            btn_rec_ref.add_css_class("destructive-btn");
            lbl_hint_rec.set_text(t(&cur_lang_rec, "hint_recording"));
        } else {
            btn_rec_ref.set_label(t(&cur_lang_rec, "btn_record"));
            btn_rec_ref.remove_css_class("destructive-btn");
            btn_rec_ref.add_css_class("secondary-btn");
            let count = st.current_macro_id
                .and_then(|id| st.macro_mgr.get_macro(id))
                .map(|m| m.actions.len())
                .unwrap_or(0);
            lbl_hint_rec.set_text(&format!("Recorded {} actions.", count));
        }
    });

    let state_play = state.clone();
    let btn_play_ref = btn_macro_play.clone();
    let lbl_hint_play = lbl_record_hint.clone();
    btn_macro_play.connect_clicked(move |_| {
        let st = state_play.borrow();
        let cur_id = match st.current_macro_id {
            Some(id) => id,
            None => return,
        };
        let macro_opt = st.macro_mgr.get_macro(cur_id).cloned();
        drop(st);

        if let Some(m) = macro_opt {
            if m.actions.is_empty() {
                lbl_hint_play.set_text("Macro has no actions.");
                return;
            }
            if !UinputPlayer::is_available() {
                lbl_hint_play.set_text("Virtual keyboard unavailable (/dev/uinput). Run 'setup-udev'.");
                return;
            }

            let btn_done = btn_play_ref.clone();
            let hint_done = lbl_hint_play.clone();
            let macro_name = m.name.clone();

            btn_play_ref.set_sensitive(false);
            lbl_hint_play.set_text(&format!("Playing '{}'...", macro_name));

            glib::spawn_future_local(async move {
                let _ = gio::spawn_blocking(move || {
                    if let Ok(player) = UinputPlayer::new() {
                        player.play(&m);
                    }
                }).await;
                btn_done.set_sensitive(true);
                hint_done.set_text(&format!("Completed playback of '{}'.", macro_name));
            });
        }
    });

    let state_save = state.clone();
    let entry_rep_save = entry_repeat.clone();
    let entry_def_save = entry_default_delay.clone();
    let rb_dr_save = rb_delay_record.clone();
    let rb_dn_save = rb_delay_none.clone();
    let lbl_hint_save = lbl_record_hint.clone();
    btn_macro_save.connect_clicked(move |_| {
        let mut st = state_save.borrow_mut();
        if let Some(cur_id) = st.current_macro_id {
            let mut saved_name = None;
            if let Some(m) = st.macro_mgr.get_macro_mut(cur_id) {
                if let Ok(rep) = entry_rep_save.text().trim().parse::<u32>() {
                    m.repeat_time = rep.max(1);
                }
                if let Ok(def_d) = entry_def_save.text().trim().parse::<u32>() {
                    m.default_delay = def_d;
                }
                m.delay_type = if rb_dr_save.is_active() {
                    DELAY_RECORD
                } else if rb_dn_save.is_active() {
                    DELAY_NONE
                } else {
                    DELAY_DEFAULT
                };
                saved_name = Some(m.name.clone());
            }
            if let Some(name) = saved_name {
                st.macro_mgr.save();
                lbl_hint_save.set_text(&format!("Macro '{}' saved successfully.", name));
            }
        }
    });

    // Window Key Controller for Recording Actions
    let key_ctrl = EventControllerKey::new();
    let state_kpress = state.clone();
    let act_lb_kpress = action_listbox.clone();
    let cur_lang_kpress = cur_lang.clone();
    key_ctrl.connect_key_pressed(move |_ctrl, key, code, _modifiers| {
        let mut st = state_kpress.borrow_mut();
        if !st.is_recording {
            return glib::Propagation::Proceed;
        }
        let cur_id = match st.current_macro_id {
            Some(id) => id,
            None => return glib::Propagation::Proceed,
        };
        let mode = st.macro_mgr.get_macro(cur_id).map(|m| m.delay_type).unwrap_or(DELAY_DEFAULT);
        let def_d = st.macro_mgr.get_macro(cur_id).map(|m| m.default_delay).unwrap_or(10);
        let now = Instant::now();
        let delay = match mode {
            DELAY_NONE => 0,
            DELAY_DEFAULT => def_d,
            _ => {
                if let Some(last) = st.record_last_instant {
                    now.duration_since(last).as_millis() as u32
                } else {
                    10
                }
            }
        };
        st.record_last_instant = Some(now);

        let (desc, evcode) = key_to_info(&key, code);
        let action = MacroAction {
            desc: desc.clone(),
            action: "Down".to_string(),
            delay_ms: delay,
            keycode: evcode,
        };
        st.macro_mgr.add_action(cur_id, action.clone());
        drop(st);

        let row = create_action_row(&action, &cur_lang_kpress);
        act_lb_kpress.append(&row);
        glib::Propagation::Stop
    });

    let state_krel = state.clone();
    let act_lb_krel = action_listbox.clone();
    let cur_lang_krel = cur_lang.clone();
    key_ctrl.connect_key_released(move |_ctrl, key, code, _modifiers| {
        let mut st = state_krel.borrow_mut();
        if !st.is_recording {
            return;
        }
        let cur_id = match st.current_macro_id {
            Some(id) => id,
            None => return,
        };
        let mode = st.macro_mgr.get_macro(cur_id).map(|m| m.delay_type).unwrap_or(DELAY_DEFAULT);
        let def_d = st.macro_mgr.get_macro(cur_id).map(|m| m.default_delay).unwrap_or(10);
        let now = Instant::now();
        let delay = match mode {
            DELAY_NONE => 0,
            DELAY_DEFAULT => def_d,
            _ => {
                if let Some(last) = st.record_last_instant {
                    now.duration_since(last).as_millis() as u32
                } else {
                    10
                }
            }
        };
        st.record_last_instant = Some(now);

        let (desc, evcode) = key_to_info(&key, code);
        let action = MacroAction {
            desc: desc.clone(),
            action: "Up".to_string(),
            delay_ms: delay,
            keycode: evcode,
        };
        st.macro_mgr.add_action(cur_id, action.clone());
        drop(st);

        let row = create_action_row(&action, &cur_lang_krel);
        act_lb_krel.append(&row);
    });
    window.add_controller(key_ctrl);

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
    setup_help_tags(&help_text_view.buffer());

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

    if let Some(top0) = initial_topics.first() {
        right_title.set_text(&format!("{} {}", top0.icon, top0.title));
        render_topic_content(&help_text_view.buffer(), top0, pic_overview.as_ref());
    }

    for (idx, top) in initial_topics.iter().enumerate() {
        let btn = Button::with_label(&format!("{} {}", top.icon, top.title));
        btn.add_css_class("topic-item-btn");
        if idx == 0 {
            btn.add_css_class("topic-item-active");
        }
        topic_box.append(&btn);
        topic_buttons.push(btn);
    }

    for (idx, btn) in topic_buttons.iter().enumerate() {
        let state_top = state.clone();
        let rt_ref = right_title.clone();
        let htv_ref = help_text_view.clone();
        let pic_ref = pic_overview.clone();
        let tb_click = topic_buttons.clone();
        btn.connect_clicked(move |_| {
            let mut st = state_top.borrow_mut();
            st.current_help_topic = idx;
            let current_topics = get_topics(&st.settings.language);
            if let Some(t) = current_topics.get(idx) {
                rt_ref.set_text(&format!("{} {}", t.icon, t.title));
                render_topic_content(&htv_ref.buffer(), t, pic_ref.as_ref());
            }
            for (i, b) in tb_click.iter().enumerate() {
                if i == idx {
                    b.add_css_class("topic-item-active");
                } else {
                    b.remove_css_class("topic-item-active");
                }
            }
        });
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
                render_topic_content(&htv_prev.buffer(), t, pic_prev.as_ref());
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
                render_topic_content(&htv_next.buffer(), t, pic_next.as_ref());
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

    // Macro tab clones
    let lbl_mac_lang = lbl_macro_title.clone();
    let btn_mn_lang = btn_macro_new.clone();
    let btn_md_lang = btn_macro_del.clone();
    let btn_mc_lang = btn_macro_copy.clone();
    let btn_mren_lang = btn_macro_rename.clone();
    let btn_mimp_lang = btn_macro_import.clone();
    let btn_mexp_lang = btn_macro_export.clone();
    let lbl_mrep_lang = lbl_rep.clone();
    let lbl_mdel_lang = lbl_delay_title.clone();
    let rb_dr_lang = rb_delay_record.clone();
    let rb_dn_lang = rb_delay_none.clone();
    let rb_dd_lang = rb_delay_default.clone();
    let lbl_ms_lang = lbl_ms.clone();
    let lbl_mrec_title_lang = lbl_rec_title.clone();
    let lbl_th_desc_lang = lbl_th_desc.clone();
    let lbl_th_act_lang = lbl_th_act.clone();
    let lbl_th_del_lang = lbl_th_del.clone();
    let btn_mrec_lang = btn_macro_record.clone();
    let btn_mplay_lang = btn_macro_play.clone();
    let btn_msave_lang = btn_macro_save.clone();
    let act_lb_lang = action_listbox.clone();

    // Help tab clones
    let lbl_ht_lang = lbl_help_title.clone();
    let rt_lang = right_title.clone();
    let htv_lang = help_text_view.clone();
    let btn_pt_lang = btn_prev_topic.clone();
    let btn_nt_lang = btn_next_topic.clone();
    let rbs_lang = radio_buttons.clone();
    let tb_lang = topic_buttons.clone();
    let pic_lang = pic_overview.clone();

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

            // Macro View Labels
            lbl_mac_lang.set_text(t(code, "macro_list"));
            btn_mn_lang.set_label(t(code, "btn_new"));
            btn_md_lang.set_label(t(code, "btn_delete"));
            btn_mc_lang.set_label(t(code, "btn_copy"));
            btn_mren_lang.set_label(t(code, "btn_rename"));
            btn_mimp_lang.set_label(t(code, "btn_import"));
            btn_mexp_lang.set_label(t(code, "btn_export"));
            lbl_mrep_lang.set_text(t(code, "repeat_time"));
            lbl_mdel_lang.set_text(t(code, "delay_mode"));
            rb_dr_lang.set_label(Some(t(code, "delay_record")));
            rb_dn_lang.set_label(Some(t(code, "delay_none")));
            rb_dd_lang.set_label(Some(t(code, "delay_default")));
            lbl_ms_lang.set_text(t(code, "delay_ms"));
            lbl_mrec_title_lang.set_text(t(code, "macro_record"));
            lbl_th_desc_lang.set_text(t(code, "col_desc"));
            lbl_th_act_lang.set_text(t(code, "col_action"));
            lbl_th_del_lang.set_text(t(code, "col_delay"));
            btn_mrec_lang.set_label(if st.is_recording { t(code, "btn_stop") } else { t(code, "btn_record") });
            btn_mplay_lang.set_label(t(code, "btn_play"));
            btn_msave_lang.set_label(t(code, "btn_save"));

            if let Some(cur_id) = st.current_macro_id {
                if let Some(m) = st.macro_mgr.get_macro(cur_id) {
                    populate_action_table(&act_lb_lang, &m.actions, code);
                }
            }

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
                render_topic_content(&htv_lang.buffer(), t, pic_lang.as_ref());
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

