#!/usr/bin/env bash
# ==============================================================================
# FREE WOLF K8 Linux Controller — Installer & Uninstaller
# ==============================================================================
# Usage:
#   ./install.sh -i   Install application, desktop entry, icons, and udev rules
#   ./install.sh -u   Uninstall all installed components cleanly
#   ./install.sh -h   Display help
# ==============================================================================

set -e

# Terminal colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_NAME="freewolf-k8"
TARGET_BIN="/usr/bin/${BIN_NAME}"
TARGET_K8GUI="/usr/bin/k8gui"
TARGET_K8CTL="/usr/bin/k8ctl"
SHARE_DIR="/usr/share/freewolf-k8"
DESKTOP_FILE="/usr/share/applications/freewolf-k8.desktop"
ICON_HICOLOR_DIR="/usr/share/icons/hicolor/512x512/apps"
ICON_HICOLOR="${ICON_HICOLOR_DIR}/freewolf-k8.png"
ICON_PIXMAP="/usr/share/pixmaps/freewolf-k8.png"
UDEV_RULE="/etc/udev/rules.d/99-freewolf-k8.rules"

show_help() {
    echo -e "${BOLD}FREE WOLF K8 Linux Controller — Installation Script${NC}"
    echo -e ""
    echo -e "${BOLD}Usage:${NC}"
    echo -e "  $0 [OPTION]"
    echo -e ""
    echo -e "${BOLD}Options:${NC}"
    echo -e "  ${GREEN}-i, --install${NC}     Install binary, desktop entries, icons, assets, and udev rules"
    echo -e "  ${RED}-u, --uninstall${NC}   Completely remove installed files and revert system integration"
    echo -e "  ${BLUE}-h, --help${NC}        Show this help message"
    echo -e ""
    echo -e "${BOLD}Examples:${NC}"
    echo -e "  sudo $0 -i    # Install system-wide"
    echo -e "  sudo $0 -u    # Uninstall system-wide"
    echo -e ""
}

check_root() {
    if [ "$(id -u)" -ne 0 ]; then
        echo -e "${YELLOW}Root privileges required. Re-running with sudo...${NC}"
        exec sudo "$0" "$@"
    fi
}

do_install() {
    check_root "$@"
    echo -e "${BOLD}${BLUE}==> Installing FREE WOLF K8 Linux Controller...${NC}"

    # 1. Locate or build binary
    SOURCE_BIN=""
    if [ -f "${SCRIPT_DIR}/target/release/${BIN_NAME}" ]; then
        SOURCE_BIN="${SCRIPT_DIR}/target/release/${BIN_NAME}"
    elif [ -f "${SCRIPT_DIR}/${BIN_NAME}" ]; then
        SOURCE_BIN="${SCRIPT_DIR}/${BIN_NAME}"
    else
        echo -e "${YELLOW}Compiled binary not found. Attempting to build with cargo...${NC}"
        if command -v cargo >/dev/null 2>&1; then
            (cd "${SCRIPT_DIR}" && cargo build --release)
            SOURCE_BIN="${SCRIPT_DIR}/target/release/${BIN_NAME}"
        else
            echo -e "${RED}Error: Cargo is not installed and no precompiled binary was found.${NC}"
            echo -e "Please run 'cargo build --release' first or place '${BIN_NAME}' in this directory."
            exit 1
        fi
    fi

    # 2. Install executable and symlinks
    echo -e "  -> Installing binary to ${TARGET_BIN}"
    install -Dm755 "${SOURCE_BIN}" "${TARGET_BIN}"
    ln -sf "${TARGET_BIN}" "${TARGET_K8GUI}"
    ln -sf "${TARGET_BIN}" "${TARGET_K8CTL}"
    echo -e "  -> Created CLI/GUI symlinks: ${TARGET_K8GUI}, ${TARGET_K8CTL}"

    # 3. Install shared assets
    echo -e "  -> Installing shared assets to ${SHARE_DIR}"
    mkdir -p "${SHARE_DIR}/assets"
    if [ -d "${SCRIPT_DIR}/assets" ]; then
        cp -r "${SCRIPT_DIR}/assets/"* "${SHARE_DIR}/assets/"
    fi

    # 4. Install Desktop Application Icon
    echo -e "  -> Installing application icons"
    mkdir -p "${ICON_HICOLOR_DIR}"
    mkdir -p "/usr/share/pixmaps"

    ICON_SRC=""
    if [ -f "${SCRIPT_DIR}/assets/DeviceDriver.png" ]; then
        ICON_SRC="${SCRIPT_DIR}/assets/DeviceDriver.png"
    elif [ -f "${SCRIPT_DIR}/FreeWolf-K8-Legacy/assets/DeviceDriver.png" ]; then
        ICON_SRC="${SCRIPT_DIR}/FreeWolf-K8-Legacy/assets/DeviceDriver.png"
    fi

    if [ -n "${ICON_SRC}" ]; then
        install -Dm644 "${ICON_SRC}" "${ICON_HICOLOR}"
        install -Dm644 "${ICON_SRC}" "${ICON_PIXMAP}"
    fi

    # 5. Install .desktop entry
    echo -e "  -> Installing desktop launcher to ${DESKTOP_FILE}"
    cat << EOF > "${DESKTOP_FILE}"
[Desktop Entry]
Type=Application
Name=FREE WOLF K8
GenericName=Mechanical Keyboard Controller
Comment=Native Linux Controller & Driver for FREE WOLF K8 Mechanical Keyboard
Exec=freewolf-k8
Icon=freewolf-k8
Terminal=false
Categories=Utility;Settings;HardwareSettings;
Keywords=keyboard;rgb;lighting;macros;freewolf;k8;gaming;
StartupNotify=true
StartupWMClass=freewolf-k8
EOF
    chmod 644 "${DESKTOP_FILE}"

    # 6. Install udev rules
    if [ -f "${SCRIPT_DIR}/99-freewolf-k8.rules" ]; then
        echo -e "  -> Installing udev rules to ${UDEV_RULE}"
        install -Dm644 "${SCRIPT_DIR}/99-freewolf-k8.rules" "${UDEV_RULE}"
        if command -v udevadm >/dev/null 2>&1; then
            udevadm control --reload-rules && udevadm trigger || true
        fi
        modprobe uinput 2>/dev/null || true
    fi

    # 7. Update caches
    echo -e "  -> Updating system desktop and icon databases"
    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database /usr/share/applications || true
    fi
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor 2>/dev/null || true
    fi

    echo ""
    echo -e "${GREEN}${BOLD}✓ Installation complete!${NC}"
    echo -e "  • Launch GUI via your desktop application menu (GNOME, KDE Plasma, XFCE, etc.)"
    echo -e "  • Or run ${BOLD}freewolf-k8${NC}, ${BOLD}k8gui${NC}, or ${BOLD}k8ctl${NC} from any terminal."
}

do_uninstall() {
    check_root "$@"
    echo -e "${BOLD}${RED}==> Uninstalling FREE WOLF K8 Linux Controller...${NC}"

    echo -e "  -> Removing binaries and symlinks"
    rm -f "${TARGET_BIN}" "${TARGET_K8GUI}" "${TARGET_K8CTL}"

    echo -e "  -> Removing desktop entry"
    rm -f "${DESKTOP_FILE}"

    echo -e "  -> Removing icons"
    rm -f "${ICON_HICOLOR}" "${ICON_PIXMAP}"

    echo -e "  -> Removing shared assets"
    rm -rf "${SHARE_DIR}"

    echo -e "  -> Removing udev rules"
    if [ -f "${UDEV_RULE}" ]; then
        rm -f "${UDEV_RULE}"
        if command -v udevadm >/dev/null 2>&1; then
            udevadm control --reload-rules && udevadm trigger || true
        fi
    fi

    echo -e "  -> Refreshing desktop and icon databases"
    if command -v update-desktop-database >/dev/null 2>&1; then
        update-desktop-database /usr/share/applications || true
    fi
    if command -v gtk-update-icon-cache >/dev/null 2>&1; then
        gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor 2>/dev/null || true
    fi

    echo ""
    echo -e "${GREEN}${BOLD}✓ Uninstallation complete! All components removed.${NC}"
}

# Parse options
case "$1" in
    -i|--install)
        do_install "$@"
        ;;
    -u|--uninstall)
        do_uninstall "$@"
        ;;
    -h|--help)
        show_help
        exit 0
        ;;
    *)
        show_help
        exit 1
        ;;
esac
