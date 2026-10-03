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
                    return Self { macros };
                }
            }
        }
        Self { macros: Vec::new() }
    }

    pub fn save(&self) {
        let dir = crate::config::ConfigManager::config_dir();
        let _ = fs::create_dir_all(&dir);
        let path = Self::macros_file();
        if let Ok(json) = serde_json::to_string_pretty(&self.macros) {
            let _ = fs::write(path, json);
        }
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
                let code = action.keycode;
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

impl Drop for UinputPlayer {
    fn drop(&mut self) {
        unsafe {
            libc::ioctl(self.fd, UI_DEV_DESTROY);
            libc::close(self.fd);
        }
    }
}
