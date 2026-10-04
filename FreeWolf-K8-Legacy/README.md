# FREE WOLF K8 — Legacy Python Edition

[![Python 3.10+](https://img.shields.io/badge/Python-3.10%2B-blue.svg)](https://www.python.org/)
[![GUI](https://img.shields.io/badge/GUI-Tkinter-green.svg)](https://docs.python.org/3/library/tkinter.html)
[![License](https://img.shields.io/badge/License-MIT-purple.svg)](../LICENSE)

This directory contains the original Python 3 / Tkinter implementation and driver for the **FREE WOLF K8** Tri-Mode Mechanical Gaming Keyboard (`VID: 0x1A2C`, `PID: 0x7C80` / `0x7FFF`).

> [!NOTE]
> For daily use, high performance, and modern desktop integration, the **native Rust + GTK4/Libadwaita** application in the repository root is strongly recommended. This Python version is preserved as a lightweight, zero-compilation reference and alternative.

---

## Requirements & Prerequisites

The Python implementation relies strictly on Python standard libraries and does not require third-party `pip` packages.

### System Packages Needed

- **Python 3.10+**
- **Python Tkinter GUI bindings**

Install Tkinter for your Linux distribution:

#### Ubuntu / Debian / Linux Mint / Pop!_OS
```bash
sudo apt update
sudo apt install -y python3 python3-tk
```

#### Fedora / RHEL / Rocky Linux / AlmaLinux
```bash
sudo dnf install -y python3 python3-tkinter
```

#### Arch Linux / Manjaro / EndeavourOS
```bash
sudo pacman -S --needed python tk
```

#### openSUSE (Tumbleweed / Leap)
```bash
sudo zypper install -y python3 python3-tk
```

---

## 1. Device Permissions (udev)

Linux restricts write access to raw USB HID nodes (`/dev/hidraw*`, Interface 1) and virtual input devices (`/dev/uinput`).

Run the automated setup tool:
```bash
./setup_udev.sh
# or via CLI:
./k8ctl setup-udev
```

Alternatively, copy the rules manually:
```bash
sudo cp 99-freewolf-k8.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
sudo modprobe uinput
sudo usermod -aG input $USER
```
*(Remember to log out and log back in for group changes to apply).*

---

## 2. Launching the Applications

Both `k8gui` and `k8ctl` are executable Python launcher scripts in this directory.

### Graphical User Interface (GUI)

Launches the dark-themed Tkinter desktop application:
```bash
./k8gui
# or
python3 k8gui
```

Features included in the GUI:
- **Lighting Controls**: Switch between 21 factory lighting animations, adjust brightness (0–4) and animation speed (0–4) in real time.
- **Audio Spectrum Visualizer**: Equalizer rhythm stream (Music Mode) with selectable submode patterns and frequency intervals (33ms, 66ms, 100ms).
- **Macro Manager**: Record keystroke sequences with real-time delays, customize repeats, import/export macros to JSON, and replay via `/dev/uinput`.
- **Integrated Manual**: Built-in 9-topic hardware documentation with keyboard diagrams and shortcut tables.

---

### Command-Line Interface (CLI)

Control your keyboard directly from the terminal without launching the GUI:

```bash
# Check device hardware status and hidraw node
./k8ctl status

# List all 21 available lighting modes
./k8ctl list

# Set lighting mode by name or index
./k8ctl set steady --brightness 4
./k8ctl set neon_stream --brightness 3 --speed 2
./k8ctl set 18 --brightness 4 --speed 3
./k8ctl set 0                          # Turn off all backlights (power saving)

# Start real-time Music Mode visualizer
./k8ctl music --submode 2 --delay 66

# Manage macros via CLI
./k8ctl macro list                     # List all saved macros
./k8ctl macro show 1                   # View actions and delays in macro #1
./k8ctl macro export 1 my_macro.json   # Export macro to JSON
./k8ctl macro import my_macro.json     # Import macro from JSON
./k8ctl macro delete 2                 # Delete macro #2

# View built-in hardware documentation
./k8ctl help
./k8ctl help shortcuts
./k8ctl help battery
```

---

## 3. Protocol Specification

All lighting commands communicate over USB HID Feature Reports on **Interface 1** (Vendor Control Interface) using standard Linux `ioctl(HIDIOCSFEATURE(8))` on `/dev/hidraw*`:

### Lighting Command Packet Layout (8 Bytes)
```text
Byte 0:  0x07          (Report ID)
Byte 1:  0xFF          (Preamble Byte 1)
Byte 2:  0xFF          (Preamble Byte 2)
Byte 3:  Mode Index    (0x00 to 0x13)
Byte 4:  Brightness    (0x00 - 0x04 for Levels 1 to 5)
Byte 5:  Speed         (0x00 - 0x04 for Levels 1 to 5)
Byte 6:  0x00          (Padding)
Byte 7:  0x00          (Padding)
```

### Music Streaming Packet Layout (8 Bytes)
```text
Byte 0:  0x07          (Report ID)
Byte 1:  Band 0 (Bass)
Byte 2:  Band 1 (Low-Mid)
Byte 3:  Band 2 (Mid-High)
Byte 4:  Band 3 (High)
Byte 5:  Submode       (0x01 = Mode 1, 0x02 = Mode 2)
Byte 6:  0x00          (Padding)
Byte 7:  0x00          (Padding)
```

---

## File Structure

```
FreeWolf-K8-Legacy/
├── assets/
│   ├── DeviceDriver.png      # 500x500 high-res application icon
│   ├── icon/                 # UI navigation tab icons
│   └── keyboard/             # High-resolution keyboard layout diagram (kb_102.png)
├── freewolf_k8/
│   ├── cli.py                # Command-line interface implementation
│   ├── config.py             # Settings persistence (~/.config/freewolf-k8)
│   ├── driver.py             # Kernel sysfs discovery & HID Feature Report ioctl
│   ├── gui.py                # Tkinter dark Argonaut GUI application
│   ├── i18n.py               # 5-language translation dictionary
│   ├── macro.py              # Macro management & /dev/uinput playback engine
│   ├── manual.py             # Integrated 9-topic hardware documentation
│   ├── music.py              # Real-time audio spectrum lighting visualizer
│   ├── protocol.py           # Protocol definitions & packet generators
│   └── udev.py               # Permission probing, udev rules & elevation helpers
├── 99-freewolf-k8.rules      # Linux udev rules file
├── setup_udev.sh             # Bash permission setup script
├── k8ctl                     # CLI launcher script
├── k8gui                     # GUI launcher script
└── README.md                 # Legacy documentation
```
