//! Linux hidraw driver for FREE WOLF K8 Keyboard

use std::fs;
use crate::protocol::{LightMode, build_lighting_packet, build_music_packet};

// HIDIOCSFEATURE(8) = _IOC(_IOC_WRITE|_IOC_READ, 'H', 0x06, 8) = 0xc0084806
const HIDIOCSFEATURE_8: libc::c_ulong = 0xc0084806;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Connected,
    PermissionDenied,
    ClaimedByVm,
    NotFound,
}

impl DeviceState {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceState::Connected => "CONNECTED",
            DeviceState::PermissionDenied => "PERMISSION DENIED",
            DeviceState::ClaimedByVm => "CLAIMED BY VM",
            DeviceState::NotFound => "NOT DETECTED",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub state: DeviceState,
    pub detail: String,
    pub node: Option<String>,
}

pub struct FreeWolfK8Driver;

impl FreeWolfK8Driver {
    pub fn probe() -> ProbeResult {
        let mut usb_present = false;

        // 1. Check if USB device is detected on USB bus
        if let Ok(entries) = fs::read_dir("/sys/bus/usb/devices") {
            for entry in entries.flatten() {
                let path = entry.path();
                let v_file = path.join("idVendor");
                let p_file = path.join("idProduct");

                if let (Ok(v), Ok(p)) = (fs::read_to_string(&v_file), fs::read_to_string(&p_file)) {
                    let v = v.trim().to_lowercase();
                    let p = p.trim().to_lowercase();

                    if v == "1a2c" && (p == "7c80" || p == "7fff") {
                        usb_present = true;
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            let drv_path = path.join(format!("{}:1.1/driver", name));
                            if let Ok(link) = fs::read_link(&drv_path) {
                                if let Some(drv_name) = link.file_name().and_then(|n| n.to_str()) {
                                    if drv_name == "usbfs" {
                                        return ProbeResult {
                                            state: DeviceState::ClaimedByVm,
                                            detail: format!("Device attached to QEMU/VM via usbfs on {}", name),
                                            node: None,
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 2. Check hidraw nodes for Interface 1
        if let Ok(entries) = fs::read_dir("/sys/class/hidraw") {
            let mut hidraw_nodes: Vec<_> = entries.flatten().map(|e| e.path()).collect();
            hidraw_nodes.sort();

            for h in hidraw_nodes {
                let uevent_file = h.join("device/uevent");
                if let Ok(content) = fs::read_to_string(&uevent_file) {
                    let c_up = content.to_uppercase();
                    if c_up.contains("1A2C:00007C80") || c_up.contains("1A2C:00007FFF") {
                        let dev_path = h.join("device");
                        if let Ok(real_p) = fs::canonicalize(&dev_path) {
                            let p_str = real_p.to_string_lossy();
                            if p_str.contains(":1.1") {
                                if let Some(hname) = h.file_name().and_then(|n| n.to_str()) {
                                    let node = format!("/dev/{}", hname);

                                    let is_rw = unsafe {
                                        let c_node = std::ffi::CString::new(node.clone()).unwrap();
                                        libc::access(c_node.as_ptr(), libc::R_OK | libc::W_OK) == 0
                                    };

                                    if is_rw {
                                        return ProbeResult {
                                            state: DeviceState::Connected,
                                            detail: format!("Ready on {}", node),
                                            node: Some(node),
                                        };
                                    } else {
                                        return ProbeResult {
                                            state: DeviceState::PermissionDenied,
                                            detail: format!("Found {}, but write permission is required (run 'k8ctl setup-udev')", node),
                                            node: Some(node),
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if usb_present {
            ProbeResult {
                state: DeviceState::PermissionDenied,
                detail: "USB device found, but kernel HID drivers not attached or accessible.".to_string(),
                node: None,
            }
        } else {
            ProbeResult {
                state: DeviceState::NotFound,
                detail: "FREE WOLF K8 keyboard not detected on USB.".to_string(),
                node: None,
            }
        }
    }

    pub fn send_feature_report(node: &str, payload: &[u8; 8]) -> Result<(), String> {
        let c_node = std::ffi::CString::new(node).map_err(|e| e.to_string())?;

        unsafe {
            let fd = libc::open(c_node.as_ptr(), libc::O_RDWR);
            if fd < 0 {
                return Err(format!("Cannot open device node {}: {}", node, std::io::Error::last_os_error()));
            }

            let mut buf = *payload;
            let ret = libc::ioctl(fd, HIDIOCSFEATURE_8, buf.as_mut_ptr());
            libc::close(fd);

            if ret < 0 {
                return Err(format!("ioctl HIDIOCSFEATURE failed on {}: {}", node, std::io::Error::last_os_error()));
            }
        }

        Ok(())
    }

    pub fn set_lighting(node: &str, mode: &LightMode, brightness: u8, speed: u8) -> Result<(), String> {
        let packet = build_lighting_packet(mode.wire_id, brightness, speed);
        Self::send_feature_report(node, &packet)
    }

    pub fn stream_music(node: &str, submode: u8, eq_data: &[u8; 4]) -> Result<(), String> {
        let packet = build_music_packet(submode, eq_data);
        Self::send_feature_report(node, &packet)
    }
}
