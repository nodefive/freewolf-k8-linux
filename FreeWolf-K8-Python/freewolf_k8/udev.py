"""
udev and permission management for FREE WOLF K8 on Linux.
Handles checking access permissions for hidraw and uinput,
and installing udev rules with administrator elevation (sudo).
"""
import os
import sys
import glob
import subprocess
from typing import Tuple, Optional

UDEV_RULE_PATH = "/etc/udev/rules.d/99-freewolf-k8.rules"

UDEV_RULE_CONTENT = """# FREE WOLF K8 Mechanical Keyboard (Wired USB: 0x1a2c:0x7c80, Wireless 2.4G: 0x1a2c:0x7fff)
# Grants user access to the hidraw configuration interface (Interface 1)
KERNEL=="hidraw*", ATTRS{idVendor}=="1a2c", ATTRS{idProduct}=="7c80", MODE="0666", TAG+="uaccess"
KERNEL=="hidraw*", ATTRS{idVendor}=="1a2c", ATTRS{idProduct}=="7fff", MODE="0666", TAG+="uaccess"

# Virtual input device access for macro keystroke playback
KERNEL=="uinput", MODE="0666", TAG+="uaccess"
"""

SETUP_SCRIPT = f"""set -e
cat << 'EOF' > {UDEV_RULE_PATH}
{UDEV_RULE_CONTENT.strip()}
EOF

chmod 644 {UDEV_RULE_PATH}
udevadm control --reload-rules 2>/dev/null || true
udevadm trigger 2>/dev/null || true

for dev in /dev/hidraw*; do
    if [ -e "$dev" ]; then
        hname=$(basename "$dev")
        uevent="/sys/class/hidraw/${{hname}}/device/uevent"
        if [ -f "$uevent" ] && grep -qi "1A2C" "$uevent"; then
            chmod 666 "$dev" || true
        fi
    fi
done

modprobe uinput 2>/dev/null || true
if [ -e "/dev/uinput" ]; then
    chmod 666 /dev/uinput || true
fi
"""


def check_access() -> Tuple[bool, str]:
    """
    Checks if the user has full access to the keyboard's hidraw interface
    and /dev/uinput, and if the udev rule is installed.
    Returns (True, 'OK') or (False, reason).
    """
    # 1. Check if udev rule is installed
    if not os.path.exists(UDEV_RULE_PATH):
        return False, f"udev rule {UDEV_RULE_PATH} is not installed"

    # 2. Check connected keyboard hidraw nodes
    found_node = False
    for syspath in glob.glob("/sys/class/hidraw/hidraw*"):
        uevent_file = os.path.join(syspath, "device", "uevent")
        if os.path.isfile(uevent_file):
            try:
                with open(uevent_file, "r") as f:
                    content = f.read()
                if "1A2C" in content.upper():
                    dev_name = os.path.basename(syspath)
                    dev_node = f"/dev/{dev_name}"
                    found_node = True
                    if not os.access(dev_node, os.R_OK | os.W_OK):
                        return False, f"Device node {dev_node} requires read/write permissions"
            except Exception:
                pass

    # 3. Check /dev/uinput access
    if os.path.exists("/dev/uinput"):
        if not os.access("/dev/uinput", os.W_OK):
            return False, "Access to /dev/uinput is denied (needed for macro playback)"
    else:
        # uinput module might not be loaded yet
        return False, "/dev/uinput device not found"

    return True, "Full access granted"


def run_setup_with_sudo(password: str) -> Tuple[bool, str]:
    """
    Executes the udev setup script as root by passing the password to sudo -S.
    """
    try:
        proc = subprocess.Popen(
            ["sudo", "-S", "bash", "-c", SETUP_SCRIPT],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        stdout, stderr = proc.communicate(input=password + "\n", timeout=15)
        if proc.returncode == 0:
            return True, "Setup completed successfully."
        else:
            err_msg = stderr.strip() or stdout.strip() or "Permission denied or invalid password."
            # Clean up common sudo prompt text
            err_msg = err_msg.replace("[sudo] password for", "").strip()
            return False, err_msg
    except subprocess.TimeoutExpired:
        return False, "Authentication timed out."
    except Exception as e:
        return False, str(e)


def run_setup_cli(force_prompt: bool = False) -> bool:
    """
    Runs udev setup in the CLI. Prompts for sudo password via standard terminal sudo.
    """
    if os.geteuid() == 0:
        print("Running udev setup as root...")
        res = subprocess.run(["bash", "-c", SETUP_SCRIPT])
        return res.returncode == 0

    print("================================================================================")
    print(" FREE WOLF K8 Linux udev & Device Permissions Setup")
    print("================================================================================")
    print("This will install /etc/udev/rules.d/99-freewolf-k8.rules and configure")
    print("permissions for /dev/hidraw* and /dev/uinput.")
    print("Administrator privileges (sudo) are required.")
    print("-" * 80)

    try:
        res = subprocess.run(["sudo", "bash", "-c", SETUP_SCRIPT])
        if res.returncode == 0:
            print("--------------------------------------------------------------------------------")
            print("✓ Setup complete! udev rules installed and device permissions updated.")
            print("================================================================================")
            return True
        else:
            print("Error: Setup failed or authentication was cancelled.")
            return False
    except KeyboardInterrupt:
        print("\nSetup cancelled.")
        return False
    except Exception as e:
        print(f"Error executing sudo: {e}")
        return False


def prompt_password_dialog(parent, on_success=None, on_cancel=None):
    """
    Displays an Argonaut GNOME styled password dialog modal to authenticate sudo for udev setup.
    """
    import tkinter as tk

    BG_WIN = "#0e1019"
    BG_CARD = "#151829"
    BG_VIEW = "#101321"
    BORDER = "#232840"
    ACCENT = "#027ad7"
    ACCENT_HOVER = "#1a8fe5"
    FG_PRI = "#ffffff"
    FG_BODY = "#d8dee9"
    FG_MUTED = "#7e88a0"
    FG_ERR = "#ED5F5D"
    BTN_BG = "#1a1e32"
    BTN_HOVER = "#222842"
    FONT = ("Adwaita Sans", 10)
    FONT_BOLD = ("Adwaita Sans", 10, "bold")
    FONT_TITLE = ("Adwaita Sans", 11, "bold")
    FONT_SMALL = ("Adwaita Sans", 9)

    dialog = tk.Toplevel(parent)
    dialog.title("Authentication Required")
    dialog.configure(bg=BG_CARD, highlightbackground=BORDER, highlightthickness=1)
    dialog.geometry("460x240")
    dialog.transient(parent)
    dialog.grab_set()

    # Center dialog on parent window
    try:
        dialog.update_idletasks()
        parent.update_idletasks()
        x = parent.winfo_rootx() + (parent.winfo_width() // 2) - 230
        y = parent.winfo_rooty() + (parent.winfo_height() // 2) - 120
        dialog.geometry(f"+{max(0, x)}+{max(0, y)}")
    except Exception:
        pass

    # Header
    tk.Label(
        dialog, text="🔒  Administrator Authentication",
        font=FONT_TITLE, fg=ACCENT, bg=BG_CARD
    ).pack(pady=(16, 6), padx=18, anchor="w")

    tk.Frame(dialog, height=1, bg=BORDER).pack(fill="x", padx=18, pady=(0, 10))

    # Description
    desc = (
        "FREE WOLF K8 requires administrator privileges to configure USB device "
        "permissions (hidraw) and virtual input (/dev/uinput) for macros."
    )
    tk.Label(
        dialog, text=desc, font=FONT_SMALL,
        fg=FG_BODY, bg=BG_CARD, wraplength=420, justify="left"
    ).pack(padx=18, anchor="w")

    # Password Entry
    entry_pwd = tk.Entry(
        dialog, show="•", font=FONT,
        bg=BG_VIEW, fg=FG_PRI, insertbackground=FG_PRI,
        bd=1, relief="solid", highlightthickness=1,
        highlightbackground=BORDER, highlightcolor=ACCENT
    )
    entry_pwd.pack(fill="x", padx=18, pady=(12, 4))
    entry_pwd.focus_set()

    lbl_err = tk.Label(dialog, text="", font=FONT_SMALL, fg=FG_ERR, bg=BG_CARD)
    lbl_err.pack(padx=18, anchor="w")

    # Buttons
    btn_row = tk.Frame(dialog, bg=BG_CARD)
    btn_row.pack(side="bottom", fill="x", padx=18, pady=(0, 16))

    def on_cancel_click():
        dialog.grab_release()
        dialog.destroy()
        if on_cancel:
            on_cancel()

    def on_submit():
        pwd = entry_pwd.get()
        if not pwd:
            lbl_err.config(text="Password cannot be empty.", fg=FG_ERR)
            return

        lbl_err.config(text="Authenticating...", fg=ACCENT)
        dialog.update()

        ok, msg = run_setup_with_sudo(pwd)
        if ok:
            dialog.grab_release()
            dialog.destroy()
            if on_success:
                on_success()
        else:
            lbl_err.config(text="Authentication failed: Incorrect password or permission denied.", fg=FG_ERR)
            entry_pwd.delete(0, "end")
            entry_pwd.focus_set()

    btn_cancel = tk.Button(
        btn_row, text="Cancel", font=FONT_BOLD,
        bg=BTN_BG, fg=FG_BODY, activebackground=BTN_HOVER, activeforeground=FG_PRI,
        bd=0, relief="flat", highlightbackground=BORDER, highlightthickness=1,
        padx=14, pady=4, command=on_cancel_click
    )
    btn_cancel.pack(side="right", padx=(8, 0))

    btn_auth = tk.Button(
        btn_row, text="Authenticate", font=FONT_BOLD,
        bg=ACCENT, fg=FG_PRI, activebackground=ACCENT_HOVER, activeforeground=FG_PRI,
        bd=0, relief="flat", highlightbackground=ACCENT, highlightthickness=1,
        padx=14, pady=4, command=on_submit
    )
    btn_auth.pack(side="right")

    entry_pwd.bind("<Return>", lambda e: on_submit())
    dialog.bind("<Escape>", lambda e: on_cancel_click())
