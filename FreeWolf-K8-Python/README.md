# FREE WOLF K8 Linux Controller

Native Linux driver, CLI, and GUI configuration application for the **FREE WOLF K8 mechanical keyboard** (`VendorID: 0x1a2c`, `ProductID: 0x7c80` / `0x7fff`).

Replicates the official Windows software lighting engine, visual layout, and real-time audio visualizer.

---

## 1. Quick Setup (Permissions)

Linux requires read/write permissions on the vendor configuration interface (`/dev/hidrawX`, Interface 1) and virtual input (`/dev/uinput`).

Run the automated setup command:
```bash
k8ctl setup-udev
```

Or manually:
```bash
sudo cp 99-freewolf-k8.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules && sudo udevadm trigger
```

---

## 2. Launching the Applications

### Graphical User Interface (GUI)
Launches the dark-themed desktop app matching the official software layout:
```bash
k8gui
```

### Command-Line Interface (CLI)
Query keyboard hardware status:
```bash
k8ctl status
```

List all 21 lighting modes:
```bash
k8ctl list
```

Set lighting mode (with optional brightness 0-4 and speed 0-4):
```bash
k8ctl set neon_stream --brightness 4 --speed 4
k8ctl set breathing --brightness 3 --speed 2
k8ctl set steady --brightness 4
```

Start real-time Music Mode visualizer stream (33ms, 66ms, or 100ms):
```bash
k8ctl music --submode 2 --delay 66
```

### Macro Management (CLI & GUI)
Manage macros from the CLI or in the dedicated Macro tab in the GUI:
```bash
# List all macros
k8ctl macro list

# Inspect actions, scan codes, and delays
k8ctl macro show 1

# Export / Import macros
k8ctl macro export 1 my_macro.json
k8ctl macro import my_macro.json

# Delete a macro
k8ctl macro delete 2
```

---

## 3. Protocol Specification

All configuration commands are dispatched via standard USB HID Class Requests (`SET_REPORT`, Feature Report ID `0x07`):
- **Interface**: 1 (Vendor Control Interface)
- **Transfer**: Control Transfer (Host-to-Device)
- **Report Type**: Feature (3)
- **Length**: 8 bytes

### Standard Lighting Packet Layout
```text
Byte 0:  0x07          (Report ID)
Byte 1:  0xFF          (Preamble)
Byte 2:  0xFF          (Preamble)
Byte 3:  Mode Index    (0x00 to 0x13)
Byte 4:  Brightness    (0x00 - 0x04 for Levels 1 to 5)
Byte 5:  Speed         (0x00 - 0x04 for Levels 1 to 5)
Byte 6:  0x00          (Padding)
Byte 7:  0x00          (Padding)
```

### Music Mode Streaming Packet Layout
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
The frequency rate (33ms / 66ms / 100ms) controls the timer delay between streaming frames sent from host to device.
