//! Macro management and Linux /dev/uinput virtual keyboard player
#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};

pub const DELAY_RECORD: u8 = 0;
pub const DELAY_NONE: u8 = 1;
pub const DELAY_DEFAULT: u8 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroAction {
    pub desc: String,
    pub action: String, // "Down" or "Up"
    pub delay_ms: u32,
    #[serde(default)]
    pub keycode: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub id: u32,
    pub name: String,
    pub repeat_time: u32,
    pub delay_type: u8,
    pub default_delay: u32,
    #[serde(default)]
    pub actions: Vec<MacroAction>,
}

pub struct MacroManager {
    pub macros: Vec<Macro>,
}

impl MacroManager {
    pub fn macros_file() -> PathBuf {
        crate::config::ConfigManager::config_dir().join("macros.json")
    }

    pub fn load() -> Self {
        let path = Self::macros_file();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(macros) = serde_json::from_str::<Vec<Macro>>(&content) {
                    if !macros.is_empty() {
                        return Self { macros };
                    }
                }
            }
        }
        let default_macros = vec![Macro {
            id: 1,
            name: "Macro 1".to_string(),
            repeat_time: 1,
            delay_type: DELAY_DEFAULT,
            default_delay: 10,
            actions: vec![
                MacroAction { desc: "Key Q".to_string(), action: "Down".to_string(), delay_ms: 10, keycode: 16 },
                MacroAction { desc: "Key Q".to_string(), action: "Up".to_string(), delay_ms: 10, keycode: 16 },
                MacroAction { desc: "Key W".to_string(), action: "Down".to_string(), delay_ms: 10, keycode: 17 },
                MacroAction { desc: "Key W".to_string(), action: "Up".to_string(), delay_ms: 10, keycode: 17 },
                MacroAction { desc: "Key E".to_string(), action: "Down".to_string(), delay_ms: 10, keycode: 18 },
                MacroAction { desc: "Key E".to_string(), action: "Up".to_string(), delay_ms: 10, keycode: 18 },
            ],
        }];
        let mgr = Self { macros: default_macros };
        mgr.save();
        mgr
    }

    pub fn save(&self) {
        let dir = crate::config::ConfigManager::config_dir();
        let _ = fs::create_dir_all(&dir);
        let path = Self::macros_file();
        if let Ok(json) = serde_json::to_string_pretty(&self.macros) {
            let _ = fs::write(path, json);
        }
    }

    pub fn get_macro(&self, id: u32) -> Option<&Macro> {
        self.macros.iter().find(|m| m.id == id)
    }

    pub fn get_macro_mut(&mut self, id: u32) -> Option<&mut Macro> {
        self.macros.iter_mut().find(|m| m.id == id)
    }

    pub fn add_macro(&mut self, name: &str) -> &Macro {
        let next_id = self.macros.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        let m = Macro {
            id: next_id,
            name: name.to_string(),
            repeat_time: 1,
            delay_type: DELAY_DEFAULT,
            default_delay: 10,
            actions: Vec::new(),
        };
        self.macros.push(m);
        self.save();
        self.macros.last().unwrap()
    }

    pub fn delete_macro(&mut self, id: u32) -> bool {
        let orig = self.macros.len();
        self.macros.retain(|m| m.id != id);
        let changed = self.macros.len() != orig;
        if changed {
            self.save();
        }
        changed
    }

    pub fn copy_macro(&mut self, id: u32) -> Option<u32> {
        let existing = self.macros.iter().find(|m| m.id == id)?.clone();
        let next_id = self.macros.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        let copied = Macro {
            id: next_id,
            name: format!("{}_copy", existing.name),
            repeat_time: existing.repeat_time,
            delay_type: existing.delay_type,
            default_delay: existing.default_delay,
            actions: existing.actions,
        };
        self.macros.push(copied);
        self.save();
        Some(next_id)
    }

    pub fn rename_macro(&mut self, id: u32, new_name: &str) -> bool {
        if let Some(m) = self.macros.iter_mut().find(|m| m.id == id) {
            m.name = new_name.to_string();
            self.save();
            true
        } else {
            false
        }
    }

    pub fn delete_action(&mut self, macro_id: u32, action_idx: usize) -> bool {
        if let Some(m) = self.get_macro_mut(macro_id) {
            if action_idx < m.actions.len() {
                m.actions.remove(action_idx);
                self.save();
                return true;
            }
        }
        false
    }

    pub fn reorder_action(&mut self, macro_id: u32, old_idx: usize, new_idx: usize) -> bool {
        if let Some(m) = self.get_macro_mut(macro_id) {
            if old_idx < m.actions.len() && new_idx < m.actions.len() {
                let item = m.actions.remove(old_idx);
                m.actions.insert(new_idx, item);
                self.save();
                return true;
            }
        }
        false
    }

    pub fn update_action_delay(&mut self, macro_id: u32, action_idx: usize, delay_ms: u32) -> bool {
        if let Some(m) = self.get_macro_mut(macro_id) {
            if let Some(act) = m.actions.get_mut(action_idx) {
                act.delay_ms = delay_ms;
                self.save();
                return true;
            }
        }
        false
    }

    pub fn add_action(&mut self, macro_id: u32, action: MacroAction) {
        if let Some(m) = self.get_macro_mut(macro_id) {
            m.actions.push(action);
            self.save();
        }
    }

    pub fn import_macro(&mut self, path: &std::path::Path) -> Result<u32, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let mut imported: Macro = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        let next_id = self.macros.iter().map(|m| m.id).max().unwrap_or(0) + 1;
        imported.id = next_id;
        self.macros.push(imported);
        self.save();
        Ok(next_id)
    }

    pub fn export_macro(&self, macro_id: u32, path: &std::path::Path) -> Result<(), String> {
        if let Some(m) = self.get_macro(macro_id) {
            let json = serde_json::to_string_pretty(m).map_err(|e| e.to_string())?;
            fs::write(path, json).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("Macro not found".to_string())
        }
    }
}

// -----------------------------------------------------------------------------
// Linux /dev/uinput Virtual Keyboard Player
// -----------------------------------------------------------------------------

#[repr(C)]
struct UinputSetup {
    id: InputId,
    name: [libc::c_char; 80],
    ff_effects_max: u32,
}

#[repr(C)]
struct InputId {
    bustype: u16,
    vendor: u16,
    product: u16,
    version: u16,
}

#[repr(C)]
struct InputEvent {
    time: libc::timeval,
    type_: u16,
    code: u16,
    value: i32,
}

const EV_SYN: u16 = 0x00;
const EV_KEY: u16 = 0x01;
const SYN_REPORT: u16 = 0x00;

const UI_SET_EVBIT: libc::c_ulong = 0x40045564;
const UI_SET_KEYBIT: libc::c_ulong = 0x40045565;
const UI_DEV_CREATE: libc::c_ulong = 0x5501;
const UI_DEV_DESTROY: libc::c_ulong = 0x5502;

pub struct UinputPlayer {
    fd: libc::c_int,
}

impl UinputPlayer {
    pub fn is_available() -> bool {
        unsafe {
            let c_path = std::ffi::CString::new("/dev/uinput").unwrap();
            libc::access(c_path.as_ptr(), libc::W_OK) == 0
        }
    }

    pub fn new() -> Result<Self, String> {
        let fd = unsafe {
            let c_path = std::ffi::CString::new("/dev/uinput").unwrap();
            libc::open(c_path.as_ptr(), libc::O_WRONLY | libc::O_NONBLOCK)
        };

        if fd < 0 {
            return Err("Cannot open /dev/uinput. Ensure permissions are set via 'k8ctl setup-udev'".to_string());
        }

        unsafe {
            // Enable key events
            libc::ioctl(fd, UI_SET_EVBIT, EV_KEY as libc::c_ulong);
            // Enable keys 1..255
            for k in 1..256 {
                libc::ioctl(fd, UI_SET_KEYBIT, k as libc::c_ulong);
            }

            // Create virtual keyboard device
            let mut name = [0 as libc::c_char; 80];
            let dev_name = b"FREE WOLF K8 Virtual Keyboard\0";
            for (i, &b) in dev_name.iter().enumerate() {
                if i < 80 {
                    name[i] = b as libc::c_char;
                }
            }

            let setup = UinputSetup {
                id: InputId {
                    bustype: 0x03, // USB
                    vendor: crate::protocol::VID,
                    product: crate::protocol::PID_WIRED,
                    version: 1,
                },
                name,
                ff_effects_max: 0,
            };

            // UI_DEV_SETUP = 0x405c5503
            const UI_DEV_SETUP: libc::c_ulong = 0x405c5503;
            let ret = libc::ioctl(fd, UI_DEV_SETUP, &setup);
            if ret < 0 {
                libc::close(fd);
                return Err("Failed to setup uinput device".to_string());
            }

            if libc::ioctl(fd, UI_DEV_CREATE) < 0 {
                libc::close(fd);
                return Err("Failed to create uinput device".to_string());
            }
        }

        // Allow kernel device node registration
        thread::sleep(Duration::from_millis(50));
        Ok(Self { fd })
    }

    fn emit(&self, type_: u16, code: u16, value: i32) {
        let ev = InputEvent {
            time: libc::timeval { tv_sec: 0, tv_usec: 0 },
            type_,
            code,
            value,
        };
        unsafe {
            libc::write(self.fd, &ev as *const _ as *const libc::c_void, std::mem::size_of::<InputEvent>());
        }
    }

    pub fn play(&self, m: &Macro) {
        for _ in 0..m.repeat_time.max(1) {
            for action in &m.actions {
                let code = if action.keycode != 0 {
                    action.keycode
                } else {
                    desc_to_evkey(&action.desc)
                };
                if code == 0 {
                    continue;
                }

                let value = if action.action == "Down" { 1 } else { 0 };
                self.emit(EV_KEY, code, value);
                self.emit(EV_SYN, SYN_REPORT, 0);

                let delay = match m.delay_type {
                    DELAY_NONE => 0,
                    DELAY_DEFAULT => m.default_delay,
                    _ => action.delay_ms,
                };

                if delay > 0 {
                    thread::sleep(Duration::from_millis(delay as u64));
                }
            }
        }
    }
}

pub fn desc_to_evkey(desc: &str) -> u16 {
    match desc {
        "Key Esc" => 1,
        "Key 1" => 2, "Key 2" => 3, "Key 3" => 4, "Key 4" => 5,
        "Key 5" => 6, "Key 6" => 7, "Key 7" => 8, "Key 8" => 9, "Key 9" => 10, "Key 0" => 11,
        "Key -" => 12, "Key =" => 13, "Key Backspace" => 14,
        "Key Tab" => 15,
        "Key Q" => 16, "Key W" => 17, "Key E" => 18, "Key R" => 19, "Key T" => 20,
        "Key Y" => 21, "Key U" => 22, "Key I" => 23, "Key O" => 24, "Key P" => 25,
        "Key [" => 26, "Key ]" => 27,
        "Key Enter" => 28,
        "Key Ctrl" | "Key Left Ctrl" => 29,
        "Key A" => 30, "Key S" => 31, "Key D" => 32, "Key F" => 33, "Key G" => 34,
        "Key H" => 35, "Key J" => 36, "Key K" => 37, "Key L" => 38,
        "Key ;" => 39, "Key '" => 40, "Key `" => 41,
        "Key Shift" | "Key Left Shift" => 42,
        "Key \\" => 43,
        "Key Z" => 44, "Key X" => 45, "Key C" => 46, "Key V" => 47, "Key B" => 48,
        "Key N" => 49, "Key M" => 50,
        "Key ," => 51, "Key ." => 52, "Key /" => 53,
        "Key Right Shift" => 54,
        "Key Alt" | "Key Left Alt" => 56,
        "Key Space" => 57,
        "Key Caps Lock" => 58,
        "Key F1" => 59, "Key F2" => 60, "Key F3" => 61, "Key F4" => 62,
        "Key F5" => 63, "Key F6" => 64, "Key F7" => 65, "Key F8" => 66,
        "Key F9" => 67, "Key F10" => 68,
        "Key F11" => 87, "Key F12" => 88,
        "Key Right Ctrl" => 97,
        "Key Right Alt" => 100,
        "Key Up" => 103, "Key Left" => 105, "Key Right" => 106, "Key Down" => 108,
        "Key Home" => 102, "Key Page Up" => 104, "Key End" => 107, "Key Page Down" => 109,
        "Key Insert" => 110, "Key Delete" => 111,
        "Key Win" | "Key Left Win" => 125, "Key Right Win" => 126,
        _ => {
            let clean = desc.strip_prefix("Key ").unwrap_or(desc).to_ascii_uppercase();
            match clean.as_str() {
                "A" => 30, "B" => 48, "C" => 46, "D" => 32, "E" => 18, "F" => 33,
                "G" => 34, "H" => 35, "I" => 23, "J" => 36, "K" => 37, "L" => 38,
                "M" => 50, "N" => 49, "O" => 24, "P" => 25, "Q" => 16, "R" => 19,
                "S" => 31, "T" => 20, "U" => 22, "V" => 47, "W" => 17, "X" => 45,
                "Y" => 21, "Z" => 44,
                _ => 0,
            }
        }
    }
}

impl Drop for UinputPlayer {
    fn drop(&mut self) {
        unsafe {
            libc::ioctl(self.fd, UI_DEV_DESTROY);
            libc::close(self.fd);
        }
    }
}
