#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RULE_FILE="${SCRIPT_DIR}/99-freewolf-k8.rules"

echo "=== FREE WOLF K8 Linux udev Setup ==="

if [ ! -f "$RULE_FILE" ]; then
    echo "Error: $RULE_FILE not found."
    exit 1
fi

echo "Installing udev rule to /etc/udev/rules.d/..."
sudo cp "$RULE_FILE" /etc/udev/rules.d/99-freewolf-k8.rules

echo "Reloading udev rules..."
sudo udevadm control --reload-rules
sudo udevadm trigger

# If any hidraw node for 1a2c is already created, make sure it has permissions right now
for dev in /dev/hidraw*; do
    if [ -e "$dev" ]; then
        hname=$(basename "$dev")
        uevent="/sys/class/hidraw/${hname}/device/uevent"
        if [ -f "$uevent" ] && grep -qi "1A2C" "$uevent"; then
            echo "Granting access to current node: $dev"
            sudo chmod 666 "$dev" || true
        fi
    fi
done

# Ensure uinput is loaded and accessible for macro playback
echo "Enabling /dev/uinput permissions..."
sudo modprobe uinput 2>/dev/null || true
if [ -e "/dev/uinput" ]; then
    sudo chmod 666 /dev/uinput || true
fi

echo "Setup complete! You can now run the app and play macros without sudo."
