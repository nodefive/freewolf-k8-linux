"""
Linux hidraw driver for FREE WOLF K8 Keyboard
"""
import os
import glob
import fcntl
from typing import Optional, Tuple
from .protocol import (
    VID, PID_WIRED, PID_WIRELESS,
    build_lighting_packet, build_music_packet, LightMode, LIGHT_MODES
)

# Standard Linux HID ioctl for sending Feature Reports:
# HIDIOCSFEATURE(8) = _IOC(_IOC_WRITE|_IOC_READ, 'H', 0x06, 8) = 0xc0084806
HIDIOCSFEATURE_8 = 0xc0084806

class DeviceState:
    CONNECTED = "connected"
    PERMISSION_DENIED = "permission_denied"
    CLAIMED_BY_VM = "claimed_by_vm"
    NOT_FOUND = "not_found"

class FreeWolfK8Driver:
    def __init__(self):
        self.device_node: Optional[str] = None
        self.fd: Optional[int] = None
        self.state: str = DeviceState.NOT_FOUND
        self.status_detail: str = ""

    def probe(self) -> Tuple[str, str]:
        """
        Inspect sysfs and hidraw nodes to locate Interface 1.
        Returns (state, detail_message).
        """
        usb_present = False
        target_vids = {f"{VID:04x}".lower()}
        target_pids = {f"{PID_WIRED:04x}".lower(), f"{PID_WIRELESS:04x}".lower()}

        # 1. Check if USB device is detected on USB bus
        for dev_path in glob.glob("/sys/bus/usb/devices/*"):
            v_file = os.path.join(dev_path, "idVendor")
            p_file = os.path.join(dev_path, "idProduct")
            if os.path.exists(v_file) and os.path.exists(p_file):
                try:
                    with open(v_file, "r") as f:
                        v = f.read().strip().lower()
                    with open(p_file, "r") as f:
                        p = f.read().strip().lower()
                    if v in target_vids and p in target_pids:
                        usb_present = True
                        # Check if Interface 1 is captured by usbfs (e.g. QEMU)
                        base = os.path.basename(dev_path)
                        if1_drv = os.path.join(dev_path, f"{base}:1.1/driver")
                        if os.path.exists(if1_drv):
                            drv = os.path.basename(os.readlink(if1_drv))
                            if drv == "usbfs":
                                self.state = DeviceState.CLAIMED_BY_VM
                                self.status_detail = f"Device attached to QEMU/VM via usbfs on {base}"
                                return self.state, self.status_detail
                except Exception:
                    pass

        # 2. Check hidraw nodes for Interface 1
        for h in sorted(glob.glob("/sys/class/hidraw/hidraw*")):
            uevent = os.path.join(h, "device/uevent")
            if os.path.exists(uevent):
                try:
                    with open(uevent, "r") as f:
                        content = f.read().upper()
                    if "1A2C:00007C80" in content or "1A2C:00007FFF" in content:
                        real_p = os.path.realpath(os.path.join(h, "device"))
                        # Interface 1 is the vendor control channel
                        if ":1.1" in real_p or ":1.1/" in real_p:
                            node = f"/dev/{os.path.basename(h)}"
                            self.device_node = node
                            if os.access(node, os.R_OK | os.W_OK):
                                self.state = DeviceState.CONNECTED
                                self.status_detail = f"Ready on {node}"
                                return self.state, self.status_detail
                            else:
                                self.state = DeviceState.PERMISSION_DENIED
                                self.status_detail = f"Found {node}, but write permission is required (run setup_udev.sh)"
                                return self.state, self.status_detail
                except Exception:
                    pass

        if usb_present:
            self.state = DeviceState.PERMISSION_DENIED
            self.status_detail = "USB device found, but kernel HID drivers not attached or accessible."
        else:
            self.state = DeviceState.NOT_FOUND
            self.status_detail = "FREE WOLF K8 keyboard not found on USB."
        return self.state, self.status_detail

    def open(self) -> bool:
        """Opens the hidraw device node."""
        self.probe()
        if self.state != DeviceState.CONNECTED or not self.device_node:
            return False
        try:
            self.fd = os.open(self.device_node, os.O_RDWR)
            return True
        except Exception as e:
            self.status_detail = f"Failed to open {self.device_node}: {e}"
            return False

    def close(self):
        """Closes the device node."""
        if self.fd is not None:
            try:
                os.close(self.fd)
            except Exception:
                pass
            self.fd = None

    def send_feature_report(self, payload: bytes) -> bool:
        """
        Sends an 8-byte Feature Report to Interface 1 using HIDIOCSFEATURE ioctl.
        """
        if self.fd is None:
            if not self.open():
                return False

        if len(payload) != 8:
            raise ValueError(f"Payload must be 8 bytes, got {len(payload)}")

        try:
            # Buffer must be mutable bytearray for ioctl
            buf = bytearray(payload)
            fcntl.ioctl(self.fd, HIDIOCSFEATURE_8, buf)
            return True
        except Exception as e:
            self.status_detail = f"Error sending feature report: {e}"
            self.close()
            return False

    def set_lighting(self, mode: LightMode, brightness: int = 5, speed: int = 4) -> bool:
        """
        Configures keyboard lighting mode, brightness (1-5), and speed (1-5).
        """
        packet = build_lighting_packet(mode.wire_id, brightness, speed)
        return self.send_feature_report(packet)

    def stream_music_packet(self, submode: int = 2, eq_data: bytes = b'\x23\x45\x67\x89') -> bool:
        """
        Sends a single Music Mode visualizer frame.
        """
        packet = build_music_packet(submode, eq_data)
        return self.send_feature_report(packet)
