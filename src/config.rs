//! Configuration management for FREE WOLF K8 Linux Controller

use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub language: String,
    pub auto_run: bool,
    pub mode_id: u8,
    pub brightness: u8,
    pub speed: u8,
    pub music_submode: u8,
    pub music_delay: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            auto_run: false,
            mode_id: 1,      // Mode 1 (Steady)
            brightness: 4,   // Max brightness
            speed: 4,        // Max speed
            music_submode: 2,
            music_delay: 66,
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    pub fn config_dir() -> PathBuf {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/freewolf-k8")
        } else {
            PathBuf::from(".config/freewolf-k8")
        }
    }

    pub fn settings_file() -> PathBuf {
        Self::config_dir().join("settings.json")
    }

    pub fn autostart_dir() -> PathBuf {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(".config/autostart")
        } else {
            PathBuf::from(".config/autostart")
        }
    }

    pub fn autostart_file() -> PathBuf {
        Self::autostart_dir().join("freewolf-k8.desktop")
    }

    pub fn load() -> Settings {
        let path = Self::settings_file();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(mut settings) = serde_json::from_str::<Settings>(&content) {
                    settings.brightness = settings.brightness.min(4);
                    settings.speed = settings.speed.min(4);
                    settings.auto_run = Self::autostart_file().exists();
                    return settings;
                }
            }
        }
        let mut def = Settings::default();
        def.auto_run = Self::autostart_file().exists();
        def
    }

    pub fn save(settings: &Settings) {
        let dir = Self::config_dir();
        let _ = fs::create_dir_all(&dir);
        let path = Self::settings_file();
        if let Ok(json) = serde_json::to_string_pretty(settings) {
            let _ = fs::write(path, json);
        }
    }

    pub fn set_autostart(enabled: bool, binary_path: Option<&str>) {
        let auto_file = Self::autostart_file();
        if enabled {
            let dir = Self::autostart_dir();
            let _ = fs::create_dir_all(&dir);

            let exec = binary_path
                .map(|s| s.to_string())
                .or_else(|| std::env::current_exe().ok().map(|p| p.to_string_lossy().to_string()))
                .unwrap_or_else(|| "freewolf-k8".to_string());

            let desktop_entry = format!(
                "[Desktop Entry]\n\
                 Type=Application\n\
                 Name=FREE WOLF K8\n\
                 Comment=FREE WOLF K8 Linux Controller\n\
                 Exec={}\n\
                 Terminal=false\n\
                 Categories=Utility;Settings;\n\
                 X-GNOME-Autostart-enabled=true\n",
                exec
            );
            let _ = fs::write(auto_file, desktop_entry);
        } else if auto_file.exists() {
            let _ = fs::remove_file(auto_file);
        }
    }
}
