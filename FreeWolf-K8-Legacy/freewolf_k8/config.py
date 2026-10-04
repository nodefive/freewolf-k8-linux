"""
Configuration management for FREE WOLF K8 Linux Controller.
Handles persistent user preferences (settings.json) and XDG autostart.
"""
import os
import sys
import json
from dataclasses import dataclass, asdict
from typing import Optional

CONFIG_DIR = os.path.expanduser("~/.config/freewolf-k8")
SETTINGS_FILE = os.path.join(CONFIG_DIR, "settings.json")
AUTOSTART_DIR = os.path.expanduser("~/.config/autostart")
AUTOSTART_FILE = os.path.join(AUTOSTART_DIR, "freewolf-k8.desktop")

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GUI_SCRIPT = os.path.join(REPO_ROOT, "k8gui")
ICON_PATH = os.path.join(REPO_ROOT, "assets", "DeviceDriver.png")

@dataclass
class Settings:
    language: str = "en"
    auto_run: bool = False
    mode_id: int = 1      # Mode 1 (Steady) is default
    brightness: int = 4   # 0 to 4 (0=off, 4=max)
    speed: int = 4
    music_submode: int = 2
    music_delay: int = 66

class ConfigManager:
    def __init__(self):
        self.settings = Settings()
        self.load()

    def load(self) -> Settings:
        if os.path.exists(SETTINGS_FILE):
            try:
                with open(SETTINGS_FILE, "r", encoding="utf-8") as f:
                    data = json.load(f)
                    for k, v in data.items():
                        if hasattr(self.settings, k):
                            setattr(self.settings, k, v)
            except Exception:
                pass
        
        # Clamp brightness and speed to valid 0-4 range
        self.settings.brightness = max(0, min(4, self.settings.brightness))
        self.settings.speed = max(0, min(4, self.settings.speed))

        # Check actual filesystem state of autostart
        self.settings.auto_run = self.is_autostart_enabled()
        return self.settings

    def save(self):
        try:
            os.makedirs(CONFIG_DIR, exist_ok=True)
            with open(SETTINGS_FILE, "w", encoding="utf-8") as f:
                json.dump(asdict(self.settings), f, indent=2)
        except Exception:
            pass

    def is_autostart_enabled(self) -> bool:
        return os.path.exists(AUTOSTART_FILE)

    def set_autostart(self, enabled: bool):
        self.settings.auto_run = enabled
        if enabled:
            try:
                os.makedirs(AUTOSTART_DIR, exist_ok=True)
                desktop_content = (
                    "[Desktop Entry]\n"
                    "Type=Application\n"
                    "Name=FREE WOLF K8\n"
                    "Comment=FREE WOLF K8 Linux Controller\n"
                    f"Exec={GUI_SCRIPT}\n"
                    f"Icon={ICON_PATH}\n"
                    "Terminal=false\n"
                    "Categories=Utility;Settings;\n"
                    "X-GNOME-Autostart-enabled=true\n"
                )
                with open(AUTOSTART_FILE, "w", encoding="utf-8") as f:
                    f.write(desktop_content)
                os.chmod(AUTOSTART_FILE, 0o755)
            except Exception:
                pass
        else:
            if os.path.exists(AUTOSTART_FILE):
                try:
                    os.remove(AUTOSTART_FILE)
                except Exception:
                    pass
        self.save()

    def restore_factory_defaults(self):
        """Restores settings to factory defaults."""
        self.settings = Settings(
            language="en",
            auto_run=False,
            mode_id=1,      # Mode 1 (Steady) is default
            brightness=4,
            speed=4,
            music_submode=2,
            music_delay=66
        )
        self.set_autostart(False)
        self.save()

