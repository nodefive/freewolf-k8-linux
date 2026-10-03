//! udev and permission management for FREE WOLF K8 on Linux

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub const UDEV_RULE_PATH: &str = "/etc/udev/rules.d/99-freewolf-k8.rules";

pub const UDEV_RULE_CONTENT: &str = r#"# FREE WOLF K8 Mechanical Keyboard (Wired USB: 0x1a2c:0x7c80, Wireless 2.4G: 0x1a2c:0x7fff)
# Grants user access to the hidraw configuration interface (Interface 1)
KERNEL=="hidraw*", ATTRS{idVendor}=="1a2c", ATTRS{idProduct}=="7c80", MODE="0666", TAG+="uaccess"
KERNEL=="hidraw*", ATTRS{idVendor}=="1a2c", ATTRS{idProduct}=="7fff", MODE="0666", TAG+="uaccess"

# Virtual input device access for macro keystroke playback
KERNEL=="uinput", MODE="0666", TAG+="uaccess"
"#;

pub fn get_setup_script() -> String {
    format!(
        "set -e\n\
         cat << 'EOF' > {}\n\
         {}\n\
         EOF\n\
         chmod 644 {}\n\
         udevadm control --reload-rules 2>/dev/null || true\n\
         udevadm trigger 2>/dev/null || true\n\
         for dev in /dev/hidraw*; do\n\
             if [ -e \"$dev\" ]; then\n\
                 hname=$(basename \"$dev\")\n\
                 uevent=\"/sys/class/hidraw/${{hname}}/device/uevent\"\n\
                 if [ -f \"$uevent\" ] && grep -qi \"1A2C\" \"$uevent\"; then\n\
                     chmod 666 \"$dev\" || true\n\
                 fi\n\
             fi\n\
         done\n\
         modprobe uinput 2>/dev/null || true\n\
         if [ -e \"/dev/uinput\" ]; then\n\
             chmod 666 /dev/uinput || true\n\
         fi\n",
        UDEV_RULE_PATH,
        UDEV_RULE_CONTENT.trim(),
        UDEV_RULE_PATH
    )
}

pub fn check_access() -> (bool, String) {
    // 1. Check if udev rule is installed
    if !Path::new(UDEV_RULE_PATH).exists() {
        return (false, format!("udev rule {} is not installed", UDEV_RULE_PATH));
    }

    // 2. Check connected keyboard hidraw nodes
    if let Ok(entries) = fs::read_dir("/sys/class/hidraw") {
        for entry in entries.flatten() {
            let uevent_file = entry.path().join("device/uevent");
            if let Ok(content) = fs::read_to_string(&uevent_file) {
                if content.to_uppercase().contains("1A2C") {
                    if let Some(hname) = entry.file_name().to_str() {
                        let dev_node = format!("/dev/{}", hname);
                        let is_rw = unsafe {
                            let c_node = std::ffi::CString::new(dev_node.clone()).unwrap();
                            libc::access(c_node.as_ptr(), libc::R_OK | libc::W_OK) == 0
                        };
                        if !is_rw {
                            return (false, format!("Device node {} requires read/write permissions", dev_node));
                        }
                    }
                }
            }
        }
    }

    // 3. Check /dev/uinput access
    let uinput_path = Path::new("/dev/uinput");
    if uinput_path.exists() {
        let is_w = unsafe {
            let c_path = std::ffi::CString::new("/dev/uinput").unwrap();
            libc::access(c_path.as_ptr(), libc::W_OK) == 0
        };
        if !is_w {
            return (false, "Access to /dev/uinput is denied (needed for macro playback)".to_string());
        }
    } else {
        return (false, "/dev/uinput device not found".to_string());
    }

    (true, "Full access granted".to_string())
}

pub fn run_setup_with_sudo(password: &str) -> (bool, String) {
    let script = get_setup_script();
    let mut child = match Command::new("sudo")
        .args(["-S", "bash", "-c", &script])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => return (false, format!("Failed to spawn sudo: {}", e)),
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(password.as_bytes());
        let _ = stdin.write_all(b"\n");
    }

    match child.wait_with_output() {
        Ok(out) => {
            if out.status.success() {
                (true, "Permissions successfully configured.".to_string())
            } else {
                let err_str = String::from_utf8_lossy(&out.stderr);
                if err_str.to_lowercase().contains("incorrect") || err_str.to_lowercase().contains("try again") {
                    (false, "Incorrect password. Please try again.".to_string())
                } else {
                    (false, format!("Configuration failed: {}", err_str.trim()))
                }
            }
        }
        Err(e) => (false, format!("Error waiting for command: {}", e)),
    }
}

pub fn run_setup_cli() -> bool {
    let (ok, msg) = check_access();
    if ok {
        println!("Permissions are already configured! Full access to /dev/hidraw and /dev/uinput is granted.");
        return true;
    }

    println!("Checking device access: {}", msg);
    println!("Root permissions are required to install udev rules and configure /dev/uinput.");
    print!("Enter sudo password: ");
    let _ = std::io::stdout().flush();

    match rpassword::read_password() {
        Ok(pw) => {
            let (success, detail) = run_setup_with_sudo(&pw);
            if success {
                println!("\nSUCCESS: {}", detail);
                println!("The FREE WOLF K8 keyboard and macro system can now be controlled without root privileges.");
                true
            } else {
                eprintln!("\nERROR: {}", detail);
                false
            }
        }
        Err(e) => {
            eprintln!("\nFailed to read password: {}", e);
            false
        }
    }
}
