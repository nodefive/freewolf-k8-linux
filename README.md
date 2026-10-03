# FREE WOLF K8 Linux Controller (Native Rust)

A high-performance, native Linux driver and graphical/command-line configuration tool for the **FREE WOLF K8** Tri-Mode (Wired USB / 2.4GHz Wireless / Bluetooth) Mechanical Keyboard.

Engineered in **Rust** using **GTK4** and **Libadwaita** with the custom dark Argonaut GNOME theme. Built into a single unified binary (`freewolf-k8`) that intelligently acts as both a desktop GUI and a fast terminal CLI.

---

## ⚡ Key Highlights

- **Single Unified Binary**: One binary handles both the full-featured Libadwaita GUI and the terminal CLI.
  - Run without arguments (or with `--gui`, `-g`, or via `k8gui` symlink) -> **Modern Libadwaita GUI**.
  - Run with subcommands, options, `-c`, `--cli`, or via `k8ctl` symlink -> **Instant CLI**.
- **No Python or Heavy Runtime Required**: Compiled to a native Linux ELF binary with minimal memory footprint and instant startup.
- **Direct Kernel Communication**: Communicates with the keyboard's USB HID Feature Reports using native Linux `ioctl(HIDIOCSFEATURE(8))` on `/dev/hidraw*` (Interface 1).
- **21 Lighting Modes**: Complete support for all factory backlighting modes, 5-level brightness (0–4), and 5-level speed/delay adjustments.
- **Audio Spectrum Visualizer (Music Mode)**: Dynamic real-time backlighting response with multi-pattern support.
- **Virtual Keyboard Macro Engine**: Keystroke playback via Linux `/dev/uinput` with exact millisecond timing.
- **Multilingual Support (i18n)**: Fully translated into 5 languages (**English**, **Português**, **Español**, **Français**, **Deutsch**) with dynamic live language switching.
- **Integrated User Manual**: 9 complete topics detailing specs, hotkeys, connection modes, battery maintenance, and troubleshooting, directly accessible in GUI and CLI.
- **Automated udev & Permission Management**: Integrated privilege elevation (`sudo`) to configure non-root access for `/dev/hidraw*` and `/dev/uinput`.

---

## 📥 Installation & Build

### Prerequisites

On Arch Linux / Manjaro:
```bash
sudo pacman -S base-devel gtk4 libadwaita
```

On Ubuntu 22.04+ / Debian 12+:
```bash
sudo apt install build-essential libgtk-4-dev libadwaita-1-dev
```

On Fedora 38+:
```bash
sudo dnf install @development-tools gtk4-devel libadwaita-devel
```

### Build from Source

```bash
cargo build --release
```

The optimized binary will be created at `target/release/freewolf-k8`.

### Convenience Symlinks

Create symlinks to launch either interface directly:
```bash
ln -sf target/release/freewolf-k8 k8gui
ln -sf target/release/freewolf-k8 k8ctl
```

---

## 🚀 Usage

### 1. Graphical User Interface (GUI)

Launch the modern GTK4/Libadwaita GUI:
```bash
./freewolf-k8
# or
./freewolf-k8 --gui
# or
./k8gui
```

The GUI offers:
- **Lighting & System**: Select from 21 lighting modes, tune brightness and animation speed, toggle auto-start at login, switch languages dynamically, and reset to defaults.
- **Macro Manager**: View saved macro sequences and trigger playback with virtual keystrokes.
- **Manual & Help**: Step-by-step documentation with high-resolution keyboard layout diagrams and specs.
- **Device Status & Permissions**: Auto-detects connection state and provides a one-click root authentication dialog to install udev rules if needed.

### 2. Command-Line Interface (CLI)

Run commands directly from the terminal without opening the window:
```bash
# Check keyboard connection status
./freewolf-k8 status

# List all 21 lighting modes
./freewolf-k8 list

# Set lighting mode, brightness (0-4), and speed (0-4)
./freewolf-k8 set -m steady -b 4
./freewolf-k8 set -m neon -b 3 -s 2
./freewolf-k8 set -m 18 -b 4 -s 3

# Launch the music visualizer (pattern 1 or 2, frequency in ms)
./freewolf-k8 music -m 2 -d 50

# Manage macros
./freewolf-k8 macro list
./freewolf-k8 macro show 1
./freewolf-k8 macro play 1

# Consult the built-in manual
./freewolf-k8 help
./freewolf-k8 help 1
./freewolf-k8 help shortcuts

# Set up udev rules and non-root access
./freewolf-k8 setup-udev

# Explicit CLI flag or via symlink
./freewolf-k8 -c status
./k8ctl status
```

---

## 🔒 Device Permissions & udev

The controller accesses `/dev/hidraw*` (Interface 1: 0x1A2C:0x7C80 / 0x1A2C:0x7FFF) and `/dev/uinput` for macro playback.

To configure permissions automatically:
```bash
./freewolf-k8 setup-udev
```
Or manually install the included rules:
```bash
sudo cp 99-freewolf-k8.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
sudo modprobe uinput
```

---

## 📁 Project Architecture

```
FreeWolf-K8-Rust/
├── assets/
│   ├── icon/                 # UI tab icons
│   └── keyboard/             # High-res layout diagrams (kb_102.png)
├── src/
│   ├── cli.rs                # Terminal CLI implementation & command dispatch
│   ├── config.rs             # Configuration persistence & autostart .desktop
│   ├── driver.rs             # Linux sysfs discovery & HID Feature Report ioctl
│   ├── gui.rs                # Libadwaita / GTK4 Argonaut-themed GUI
│   ├── i18n.rs               # 5-language translation dictionary
│   ├── macro_mgr.rs          # Macro data structures & Linux /dev/uinput playback
│   ├── main.rs               # Smart dual GUI/CLI entrypoint
│   ├── manual.rs             # Integrated 9-topic hardware user manual
│   ├── music.rs              # Real-time audio spectrum lighting visualizer
│   ├── protocol.rs           # 21 lighting modes, USB IDs, and packet generators
│   └── udev.rs               # Permission probing, udev rules & elevation helpers
├── 99-freewolf-k8.rules      # Linux udev rules file
├── setup_udev.sh             # Standalone bash setup helper
├── Cargo.toml                # Rust package manifest & dependencies
└── README.md                 # Complete documentation
```

---

## 📜 License

Distributed under the MIT License.
