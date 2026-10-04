"""
Comprehensive User Manual & Technical Documentation for FREEWOLF K8.
Compiled from official manuals and project driver specifications.
Supported Languages: English, Português, Español, Français, Deutsch.
"""
from typing import Dict, List, Any, Tuple

TOPICS_METADATA = [
    {
        "id": "overview",
        "icon": "📖",
        "titles": {
            "en": "Overview & Specs",
            "pt": "Visão Geral e Specs",
            "es": "Visión General y Specs",
            "fr": "Aperçu & Spécifications",
            "de": "Übersicht & Specs",
        },
    },
    {
        "id": "connectivity",
        "icon": "⚡",
        "titles": {
            "en": "Tri-Mode Connectivity",
            "pt": "Conectividade Tri-Modo",
            "es": "Conectividad Tri-Modo",
            "fr": "Connectivité Tri-Mode",
            "de": "Drei-Modus-Verbindung",
        },
    },
    {
        "id": "os_layouts",
        "icon": "💻",
        "titles": {
            "en": "Windows & Mac Layout",
            "pt": "Modos Windows e Mac",
            "es": "Modos Windows y Mac",
            "fr": "Modes Windows & Mac",
            "de": "Windows- & Mac-Modus",
        },
    },
    {
        "id": "rgb_lighting",
        "icon": "💡",
        "titles": {
            "en": "RGB Lighting Controls",
            "pt": "Iluminação RGB e Efeitos",
            "es": "Iluminación y Atajos",
            "fr": "Éclairage RGB & Effets",
            "de": "RGB-Beleuchtung & Effekte",
        },
    },
    {
        "id": "gaming_presets",
        "icon": "🎮",
        "titles": {
            "en": "Gaming & Custom Keys",
            "pt": "Perfis Gamer e DIY",
            "es": "Perfiles Gamer y DIY",
            "fr": "Profils Gaming & DIY",
            "de": "Gaming & Eigene Profile",
        },
    },
    {
        "id": "battery_power",
        "icon": "🔋",
        "titles": {
            "en": "Battery & Power Saving",
            "pt": "Bateria e Economia",
            "es": "Batería y Ahorro",
            "fr": "Batterie & Économie",
            "de": "Akku & Energiesparen",
        },
    },
    {
        "id": "hotswap_maintenance",
        "icon": "🔧",
        "titles": {
            "en": "Hot-Swap & Switches",
            "pt": "Troca de Switches",
            "es": "Cambio de Switches",
            "fr": "Switchs Amovibles",
            "de": "Switch-Wechsel & Pflege",
        },
    },
    {
        "id": "troubleshooting",
        "icon": "🛠",
        "titles": {
            "en": "Troubleshooting Guide",
            "pt": "Solução de Problemas",
            "es": "Solución de Problemas",
            "fr": "Guide de Dépannage",
            "de": "Fehlerbehebung",
        },
    },
    {
        "id": "linux_tools",
        "icon": "🐧",
        "titles": {
            "en": "Linux Driver & CLI",
            "pt": "Driver Linux e CLI",
            "es": "Driver Linux y CLI",
            "fr": "Pilote Linux & CLI",
            "de": "Linux-Treiber & CLI",
        },
    },
]


def get_topic_titles(lang: str) -> List[Tuple[str, str]]:
    """Returns list of (icon, title) for all topics in the requested language."""
    res = []
    for item in TOPICS_METADATA:
        icon = item["icon"]
        title = item["titles"].get(lang, item["titles"]["en"])
        res.append((icon, title))
    return res


def get_topic_content(topic_idx: int, lang: str) -> List[Tuple[str, ...]]:
    """Returns formatted content blocks for a specific topic and language."""
    if topic_idx < 0 or topic_idx >= len(TOPICS_METADATA):
        topic_idx = 0
    topic_id = TOPICS_METADATA[topic_idx]["id"]

    # English content definitions
    if lang == "pt":
        return _get_content_pt(topic_id)
    elif lang == "es":
        return _get_content_es(topic_id)
    elif lang == "fr":
        return _get_content_fr(topic_id)
    elif lang == "de":
        return _get_content_de(topic_id)
    return _get_content_en(topic_id)


def _get_content_en(topic_id: str) -> List[Tuple[str, ...]]:
    if topic_id == "overview":
        return [
            ("title", "FREEWOLF K8 — Technical Overview & Specifications"),
            ("body", "The FREEWOLF K8 is a high-performance 80% (100-key) mechanical gaming keyboard featuring universal Tri-Mode connectivity (USB-C wired, 2.4 GHz wireless, and Bluetooth 5.0 with 3 device profiles). Equipped with hot-swappable mechanical switch sockets, vibrant RGB per-key backlighting, and a massive 4000 mAh rechargeable battery, it delivers seamless productivity and gaming across Linux, Windows, and macOS."),
            ("h2", "Hardware Specifications"),
            ("table_row", "Model", "FREEWOLF K8 Tri-Mode Mechanical Keyboard"),
            ("table_row", "Layout", "80% Compact (100 Keys with full numeric keypad)"),
            ("table_row", "Dimensions", "395 mm × 142 mm × 40 mm (15.55 × 5.59 × 1.57 in)"),
            ("table_row", "Weight", "Approximately 850 grams"),
            ("table_row", "Switches", "Hot-Swappable Mechanical (Standard 3-Pin sockets, Blue Switch variant)"),
            ("table_row", "Keycaps", "PBT Double-Injection (textured, oil-resistant, wear-proof)"),
            ("table_row", "Anti-Ghosting", "Full-Key Punchless (Full N-Key Rollover / NKRO)"),
            ("table_row", "Battery", "4000 mAh high-capacity rechargeable lithium-ion battery"),
            ("table_row", "Connectivity", "Tri-Mode: USB-C Wired, 2.4 GHz Wireless, Bluetooth 5.0 (BT1, BT2, BT3)"),
            ("table_row", "USB Hardware ID", "VID: 0x1A2C  |  PID: 0x7C80  (Interface 1 HID)"),
            ("table_row", "Compatibility", "Linux, Windows 11/10/8/7, macOS, Android, iOS"),
            ("h2", "Package Contents"),
            ("bullet", "FREEWOLF K8 Mechanical Gaming Keyboard"),
            ("bullet", "Braided USB-C to USB-A Connection & Charging Cable"),
            ("bullet", "2.4 GHz USB Wireless Nano-Receiver (stored in magnetic base slot)"),
            ("bullet", "Precision Wire Keycap Puller Tool"),
            ("bullet", "Metal Switch Puller Tool"),
            ("bullet", "Two (2) Spare 3-Pin Mechanical Switches"),
            ("bullet", "Official User Manual & Quick Start Documentation"),
            ("tip", "The keyboard base includes dual-stage ergonomic tilt kickstands and non-slip rubber pads for enhanced stability and comfortable typing angles."),
        ]

    elif topic_id == "connectivity":
        return [
            ("title", "Tri-Mode Multi-Device Connectivity"),
            ("body", "The K8 keyboard allows connecting up to five devices simultaneously across three connection modes: USB-C Wired, 2.4 GHz Wireless, and Bluetooth 5.0 (3 channels)."),
            ("h2", "1. USB-C Wired Mode"),
            ("bullet", "Set the hardware slider switch (on the rear/side) to 'USB' or 'Wired'."),
            ("bullet", "Connect the USB-C cable to the keyboard and the USB-A connector to your computer."),
            ("bullet", "The keyboard is recognized automatically by the Linux driver. Zero input latency; charges battery simultaneously."),
            ("h2", "2. 2.4 GHz Ultra-Low Latency Wireless Mode"),
            ("bullet", "Remove the 2.4 GHz USB receiver from the dedicated magnetic slot on the keyboard underside."),
            ("bullet", "Plug the USB receiver into an available USB 2.0/3.0 port on your computer."),
            ("bullet", "Set the keyboard slider switch to '2.4G'."),
            ("bullet", "Connection establishes automatically. If signal is lost or to re-pair:"),
            ("key", "FN + R", "Hold for 3–5 seconds to initiate 2.4 GHz pairing (indicator flashes rapidly)."),
            ("h2", "3. Bluetooth 5.0 Mode (3 Paired Devices)"),
            ("bullet", "Set the keyboard slider switch to 'BT'."),
            ("bullet", "To pair a device to one of the 3 Bluetooth memory slots:"),
            ("key", "FN + Q", "Long-press 3–5s for BT Channel 1 (LED flashes rapidly; pair 'FREEWOLF K8' on host)."),
            ("key", "FN + W", "Long-press 3–5s for BT Channel 2 (LED flashes rapidly; pair 'FREEWOLF K8' on host)."),
            ("key", "FN + E", "Long-press 3–5s for BT Channel 3 (LED flashes rapidly; pair 'FREEWOLF K8' on host)."),
            ("bullet", "To quickly switch between paired Bluetooth devices at any time:"),
            ("key", "FN + Q / W / E", "Short press to swap between Device 1, Device 2, or Device 3 instantly."),
            ("tip", "When switching between paired Bluetooth devices, the indicator flashes slowly once and stays solid upon reconnection."),
        ]

    elif topic_id == "os_layouts":
        return [
            ("title", "Operating System Layouts & Function Shortcuts"),
            ("body", "The FREEWOLF K8 includes dedicated hardware layout profiles for Windows and macOS, alongside a full row of F1–F12 multimedia functions."),
            ("h2", "OS Layout Switching"),
            ("key", "FN + A", "Switch to Windows Mode (Standard PC layout, Windows key active, Ctrl/Alt standard)."),
            ("key", "FN + S", "Switch to macOS Mode (Swaps Option and Command keys to match native Mac layout)."),
            ("key", "FN + Win", "Windows Key Lock / Unlock (Gaming Mode: disables Win key to prevent desktop popups)."),
            ("h2", "F1 – F12 Multimedia Hotkeys"),
            ("table_row", "FN + F1", "Open Default Media Player"),
            ("table_row", "FN + F2", "Volume Down"),
            ("table_row", "FN + F3", "Volume Up"),
            ("table_row", "FN + F4", "Mute Audio"),
            ("table_row", "FN + F5", "Previous Track"),
            ("table_row", "FN + F6", "Next Track"),
            ("table_row", "FN + F7", "Play / Pause Audio & Video"),
            ("table_row", "FN + F8", "Stop Playback"),
            ("table_row", "FN + F9", "Launch Web Browser"),
            ("table_row", "FN + F10", "Open Email Application"),
            ("table_row", "FN + F11", "Open File Explorer / My Computer"),
            ("table_row", "FN + F12", "Open Calculator"),
        ]

    elif topic_id == "rgb_lighting":
        return [
            ("title", "RGB Lighting & Hardware Shortcuts"),
            ("body", "The K8 keyboard features 21 distinct RGB lighting modes, including 19 dynamic animations, static illumination, and software-streamed Music Visualizer mode."),
            ("h2", "On-Board Lighting Controls"),
            ("key", "FN + |", "Cycle through 19 dynamic RGB lighting effects (Steady, Breathing, Neon Stream, Wave, Ripples, Marquee, Snake, Rotating Storm, Stars, etc.)."),
            ("key", "FN + ↑", "Increase Backlight Brightness (5 levels: 0% / Off to 100%)."),
            ("key", "FN + ↓", "Decrease Backlight Brightness. Level 0 turns off all backlighting completely."),
            ("key", "FN + →", "Increase Animation Speed (5 dynamic speed steps)."),
            ("key", "FN + ←", "Decrease Animation Speed (5 dynamic speed steps)."),
            ("h2", "Software Control in Linux"),
            ("bullet", "Use the 'Light' tab in this app to click any lighting mode directly, adjust brightness (0–4) and speed (0–4) with zero delay."),
            ("bullet", "Music Mode: Streams real-time audio FFT frequency spectrum from PulseAudio/PipeWire to keyboard LEDs."),
            ("bullet", "In terminal: run 'k8ctl set <id|name> --brightness <0-4> --speed <0-4>'."),
            ("tip", "Setting brightness to 0 turns off all LED power, extending battery life up to 35 days in wireless mode."),
        ]

    elif topic_id == "gaming_presets":
        return [
            ("title", "Gaming Backlight Presets & Custom Recording"),
            ("body", "The K8 keyboard features three factory-programmed gaming backlight profiles, plus the ability to record custom illumination maps directly to on-board memory."),
            ("h2", "Built-In Gaming Presets"),
            ("key", "FN + 1!", "FPS Mode — Illuminates W, A, S, D, and the 4 Arrow navigation keys."),
            ("key", "FN + 2@", "LOL / MOBA Mode — Illuminates Q, W, E, R, D, F, G, V, B, Tab, Space, 1–6, and Esc."),
            ("key", "FN + 3#", "Office Mode — Illuminates full 26 letters (A–Z), punctuation, and Arrow keys."),
            ("h2", "How to Record Custom Backlight Maps"),
            ("bullet", "Step 1: Press FN + 1!, FN + 2@, or FN + 3# to choose the preset slot you want to customize."),
            ("bullet", "Step 2: Press FN + ~ (Tilde) to enter recording mode. The indicator LED starts flashing rapidly."),
            ("bullet", "Step 3: Press any key on the keyboard to toggle its backlight LED on or off."),
            ("bullet", "Step 4: Press FN + ~ again to save the custom pattern into the keyboard's non-volatile EEPROM memory."),
            ("tip", "Custom lighting maps recorded via FN + ~ persist across power cycles and work in both wired and wireless modes without requiring software."),
        ]

    elif topic_id == "battery_power":
        return [
            ("title", "Battery Specifications & Power Saving"),
            ("body", "Powered by a high-capacity 4000 mAh rechargeable lithium-ion battery, the K8 keyboard offers class-leading wireless battery life and intelligent multi-stage power saving."),
            ("h2", "Battery Life"),
            ("table_row", "Battery Capacity", "4000 mAh Lithium-Ion Rechargeable"),
            ("table_row", "Backlight Active", "Approximately 15 days (standard daily gaming/office use)"),
            ("table_row", "Backlight Off", "Up to 35 days continuous usage"),
            ("table_row", "Charging Time", "Approx. 4–5 hours via 5V/1A USB connection"),
            ("h2", "Intelligent Sleep Modes"),
            ("bullet", "5-Minute Idle Sleep: If no keys are pressed for 5 minutes, the RGB backlight automatically turns off to conserve battery."),
            ("bullet", "30-Minute Deep Sleep (Hibernation): After 30 minutes of inactivity, the keyboard enters ultra-low power hibernation mode."),
            ("bullet", "Instant Wake-Up: Press any key twice to wake the keyboard from sleep and resume typing immediately."),
            ("h2", "Charging & LED Indicators"),
            ("bullet", "Connect the provided USB-C cable to any standard computer USB port or 5V USB wall adapter."),
            ("bullet", "The battery indicator LED illuminates during charging and turns off when the battery reaches 100% full capacity."),
            ("tip", "To maximize battery longevity, avoid letting the battery sit completely drained for extended periods."),
        ]

    elif topic_id == "hotswap_maintenance":
        return [
            ("title", "Hot-Swappable Switches & Care"),
            ("body", "The FREEWOLF K8 features universal hot-swappable PCB sockets supporting standard 3-pin mechanical switches without any soldering required."),
            ("h2", "Switch Compatibility"),
            ("bullet", "Supports standard 3-pin mechanical switches (Outemu, Gateron, Cherry MX, Kailh, AKKO, etc.)."),
            ("bullet", "Two (2) spare switches, a wire keycap puller, and a metal switch puller are included in the package."),
            ("h2", "Step-by-Step Switch Replacement"),
            ("bullet", "1. Remove Keycap: Hook the wire keycap puller under the corners of the keycap and pull straight upwards."),
            ("bullet", "2. Remove Switch: Place the switch puller prongs into the top and bottom latch clips of the switch housing. Squeeze gently to release latches and pull straight vertically out of the PCB socket."),
            ("bullet", "3. Inspect Pins: Check the two copper contact pins on the bottom of the replacement switch. Ensure both pins are 100% straight and unbent."),
            ("bullet", "4. Install Switch: Align the two metal pins and center post with the PCB socket holes. Press straight down firmly until the switch snaps securely into the steel plate."),
            ("bullet", "5. Test & Replace: Test key registration before replacing the keycap."),
            ("h2", "Cleaning & Maintenance"),
            ("bullet", "Always disconnect the cable and turn off the wireless power switch before cleaning."),
            ("bullet", "Use compressed air or a soft brush to remove dust and debris between keycaps."),
            ("bullet", "Wipe keycaps with a slightly damp microfiber cloth. Never use alcohol, acetone, or harsh solvents."),
            ("tip", "Never force a switch into the PCB socket if you feel resistance. Remove it and verify that the copper pins are not bent."),
        ]

    elif topic_id == "troubleshooting":
        return [
            ("title", "Troubleshooting Guide & Solutions"),
            ("body", "Common issues, diagnostic steps, and solutions based on official manufacturer guidance and Linux driver architecture."),
            ("h2", "1. Keyboard not responding in Wired Mode"),
            ("bullet", "Check Mode Switch: Ensure the slider switch on the rear/side is set to 'USB' / 'Wired'."),
            ("bullet", "Cable Connection: Verify the USB-C cable is firmly seated into both the keyboard and computer."),
            ("bullet", "Linux Permissions: Check if the device status badge shows 'Permissions Required'. If so, run 'k8ctl setup-udev' to grant non-root access to /dev/hidraw*."),
            ("h2", "2. 2.4 GHz Wireless not connecting"),
            ("bullet", "Dongle Placement: Ensure the 2.4G USB nano-receiver is plugged directly into a working USB port."),
            ("bullet", "Mode Switch: Set the slider switch to '2.4G'."),
            ("bullet", "Re-Pairing: Press and hold FN + R for 3–5 seconds until the status indicator flashes rapidly, then move the keyboard close to the USB receiver."),
            ("h2", "3. Bluetooth connection or pairing fails"),
            ("bullet", "Mode Switch: Ensure the switch is set to 'BT'."),
            ("bullet", "Pairing Mode: Long-press FN + Q, FN + W, or FN + E for 3–5 seconds until the LED flashes rapidly. On your device, remove any existing 'FREEWOLF K8' entry and scan again."),
            ("bullet", "Interference: Ensure distance is within 10 meters and avoid dense physical obstructions."),
            ("h2", "4. RGB Backlight does not turn on or is dim"),
            ("bullet", "Brightness Level: Press FN + ↑ multiple times to increase brightness (it may have been set to 0/off)."),
            ("bullet", "Cycle Modes: Press FN + | to cycle through lighting effects."),
            ("bullet", "Power Saving: If idle for more than 5 minutes, press any key twice to wake from sleep."),
            ("bullet", "Low Battery: In wireless mode, low battery automatically disables backlight. Connect USB-C to charge."),
            ("h2", "5. Specific key does not register"),
            ("bullet", "Remove Keycap & Switch: Use the included pullers to extract the switch."),
            ("bullet", "Check Pins: Inspect the bottom copper pins. If bent, straighten carefully with tweezers or replace with one of the included spare switches."),
            ("bullet", "Clean Socket: Blow any dust out of the hot-swap socket before reinserting."),
        ]

    elif topic_id == "linux_tools":
        return [
            ("title", "Linux Native Driver & CLI Utility (k8ctl)"),
            ("body", "This application provides a completely native Linux driver, GUI configurator, and command-line utility for the FREEWOLF K8 keyboard, eliminating any need for Windows software or Wine."),
            ("h2", "Hardware Architecture"),
            ("table_row", "USB Vendor ID", "0x1A2C"),
            ("table_row", "USB Product ID", "0x7C80"),
            ("table_row", "Control Interface", "HID Interface 1 (/dev/hidraw*) — Output EP 0x02, Input EP 0x82"),
            ("table_row", "Macro Subsystem", "Linux Kernel /dev/uinput virtual keyboard"),
            ("table_row", "Audio Capture", "PulseAudio / PipeWire monitor stream"),
            ("h2", "Linux Permissions (k8ctl setup-udev)"),
            ("body", "Linux restricts direct access to hidraw and uinput device nodes by default. Run the setup command once:"),
            ("code", "k8ctl setup-udev"),
            ("body", "This installs /etc/udev/rules.d/99-freewolf-k8.rules and tags the keyboard with uaccess, allowing seamless plug-and-play without requiring sudo or root."),
            ("h2", "Command-Line Tool (k8ctl)"),
            ("body", "Control your keyboard from scripts, terminal, or keybindings:"),
            ("code", "k8ctl status"),
            ("bullet", "Show device detection state, hardware node, and driver readiness."),
            ("code", "k8ctl list"),
            ("bullet", "List all 21 available lighting modes with their IDs."),
            ("code", "k8ctl set <id|name> [--brightness 0-4] [--speed 0-4]"),
            ("bullet", "Set lighting mode, brightness, and speed. Example: 'k8ctl set 2 --brightness 4 --speed 2'."),
            ("code", "k8ctl music [--pattern 1|2] [--delay ms]"),
            ("bullet", "Stream real-time audio visualizer directly from terminal."),
            ("code", "k8ctl macro list"),
            ("bullet", "List all saved macros."),
            ("code", "k8ctl macro play <id>"),
            ("bullet", "Play macro using kernel virtual keyboard device."),
        ]

    return []


def _get_content_pt(topic_id: str) -> List[Tuple[str, ...]]:
    if topic_id == "overview":
        return [
            ("title", "FREEWOLF K8 — Visão Geral e Especificações Técnicas"),
            ("body", "O FREEWOLF K8 é um teclado mecânico gamer de alto desempenho no formato 80% (100 teclas), com conectividade Tri-Modo universal (USB-C cabeado, 2.4 GHz sem fio e Bluetooth 5.0 com 3 canais). Equipado com soquetes hot-swap mecânicos, iluminação RGB tecla a tecla e uma potente bateria recarregável de 4000 mAh, oferece máxima produtividade e jogabilidade no Linux, Windows e macOS."),
            ("h2", "Especificações de Hardware"),
            ("table_row", "Modelo", "FREEWOLF K8 Tri-Mode Mechanical Keyboard"),
            ("table_row", "Layout", "80% Compacto (100 teclas com teclado numérico integrado)"),
            ("table_row", "Dimensões", "395 mm × 142 mm × 40 mm (15,55 × 5,59 × 1,57 pol)"),
            ("table_row", "Peso", "Aproximadamente 850 gramas"),
            ("table_row", "Switches", "Mecânicos Hot-Swap (Soquetes padrão 3 pinos, Blue Switch)"),
            ("table_row", "Keycaps", "PBT Double-Shot Injection (texturizadas, resistentes a óleo)"),
            ("table_row", "Anti-Ghosting", "Full-Key Rollover (NKRO / 100% anti-ghosting sem bloqueio)"),
            ("table_row", "Bateria", "4000 mAh recarregável de íon de lítio"),
            ("table_row", "Conexão", "Tri-Modo: USB-C Cabeado, 2.4 GHz Sem Fio, Bluetooth 5.0 (BT1, BT2, BT3)"),
            ("table_row", "Identificação USB", "VID: 0x1A2C  |  PID: 0x7C80  (Interface 1 HID)"),
            ("table_row", "Compatibilidade", "Linux, Windows 11/10/8/7, macOS, Android, iOS"),
            ("h2", "Conteúdo da Embalagem"),
            ("bullet", "Teclado Mecânico Gamer FREEWOLF K8"),
            ("bullet", "Cabo Trançado USB-C para USB-A"),
            ("bullet", "Receptor Nano sem fio USB 2.4 GHz (armazenado na base)"),
            ("bullet", "Extrator de Keycaps"),
            ("bullet", "Extrator Metálico de Switches"),
            ("bullet", "Dois (2) Switches Mecânicos Sobressalentes"),
            ("bullet", "Manual do Usuário Oficial"),
            ("tip", "A base do teclado possui pés retráteis com ajuste de inclinação em dois níveis e borrachas antiderrapantes para maior conforto e estabilidade."),
        ]

    elif topic_id == "connectivity":
        return [
            ("title", "Conectividade Tri-Modo e Multi-Dispositivos"),
            ("body", "O teclado K8 permite conectar até cinco dispositivos simultaneamente por meio de três modos: Cabo USB-C, Sem Fio 2.4 GHz e Bluetooth 5.0 (3 canais)."),
            ("h2", "1. Modo Cabeado USB-C"),
            ("bullet", "Mova o seletor na traseira/lateral para a posição 'USB' ou 'Wired'."),
            ("bullet", "Conecte o cabo USB-C ao teclado e o conector USB-A ao computador."),
            ("bullet", "Reconhecimento instantâneo no Linux com latência zero e carregamento simultâneo."),
            ("h2", "2. Modo Sem Fio 2.4 GHz de Baixa Latência"),
            ("bullet", "Retire o nano-receptor 2.4 GHz do compartimento magnético na parte inferior do teclado."),
            ("bullet", "Conecte o receptor a uma porta USB do computador."),
            ("bullet", "Mova o seletor para '2.4G'."),
            ("bullet", "Conexão automática. Em caso de perda de sincronismo:"),
            ("key", "FN + R", "Segure por 3–5 segundos para parear novamente com o receptor 2.4G."),
            ("h2", "3. Modo Bluetooth 5.0 (3 Dispositivos)"),
            ("bullet", "Mova o seletor para a posição 'BT'."),
            ("bullet", "Para parear em um dos 3 canais de memória Bluetooth:"),
            ("key", "FN + Q", "Segure por 3–5s para o Canal 1 (LED pisca rápido; pareie 'FREEWOLF K8' no sistema)."),
            ("key", "FN + W", "Segure por 3–5s para o Canal 2 (LED pisca rápido; pareie 'FREEWOLF K8' no sistema)."),
            ("key", "FN + E", "Segure por 3–5s para o Canal 3 (LED pisca rápido; pareie 'FREEWOLF K8' no sistema)."),
            ("bullet", "Para alternar rapidamente entre os dispositivos pareados:"),
            ("key", "FN + Q / W / E", "Toque rápido para trocar instantaneamente entre Dispositivo 1, 2 ou 3."),
        ]

    elif topic_id == "os_layouts":
        return [
            ("title", "Modos de Sistema Operacional e Teclas de Função"),
            ("body", "O teclado possui perfis de layout dedicados para Windows e macOS, além de atalhos multimídia nas teclas F1 a F12."),
            ("h2", "Troca de Layout de Sistema"),
            ("key", "FN + A", "Ativar Modo Windows (Layout PC padrão, tecla Win ativa, Alt normal)."),
            ("key", "FN + S", "Ativar Modo macOS (Inverte Option e Command para layout nativo Mac)."),
            ("key", "FN + Win", "Bloquear / Desbloquear Tecla Windows (Modo Gamer evita sair do jogo acidentalmente)."),
            ("h2", "Atalhos Multimídia F1 a F12"),
            ("table_row", "FN + F1", "Abrir Player de Mídia"),
            ("table_row", "FN + F2", "Diminuir Volume"),
            ("table_row", "FN + F3", "Aumentar Volume"),
            ("table_row", "FN + F4", "Silenciar Áudio (Mudo)"),
            ("table_row", "FN + F5", "Faixa Anterior"),
            ("table_row", "FN + F6", "Próxima Faixa"),
            ("table_row", "FN + F7", "Reproduzir / Pausar"),
            ("table_row", "FN + F8", "Parar Reprodução"),
            ("table_row", "FN + F9", "Abrir Navegador Web"),
            ("table_row", "FN + F10", "Abrir Aplicativo de E-mail"),
            ("table_row", "FN + F11", "Abrir Meu Computador / Gerenciador de Arquivos"),
            ("table_row", "FN + F12", "Abrir Calculadora"),
        ]

    elif topic_id == "rgb_lighting":
        return [
            ("title", "Iluminação RGB e Atalhos de Hardware"),
            ("body", "O K8 possui 21 modos de iluminação, incluindo 19 efeitos dinâmicos de fábrica, modo estático e visualizador de música por software."),
            ("h2", "Controles de Iluminação pelo Teclado"),
            ("key", "FN + |", "Alternar entre os 19 efeitos RGB dinâmicos."),
            ("key", "FN + ↑", "Aumentar Brilho dos LEDs (5 níveis: 0% / Desligado a 100%)."),
            ("key", "FN + ↓", "Diminuir Brilho dos LEDs. O nível 0 desliga completamente os LEDs."),
            ("key", "FN + →", "Aumentar Velocidade das animações RGB (5 passos)."),
            ("key", "FN + ←", "Diminuir Velocidade das animações RGB (5 passos)."),
            ("h2", "Controle via Software no Linux"),
            ("bullet", "Na aba 'Luz' deste aplicativo, clique diretamente em qualquer efeito e ajuste brilho e velocidade em tempo real."),
            ("bullet", "Modo Música: Converte o áudio do PulseAudio/PipeWire em ondas de iluminação no teclado."),
            ("bullet", "No terminal: use 'k8ctl set <id|nome> --brightness <0-4> --speed <0-4>'."),
        ]

    elif topic_id == "gaming_presets":
        return [
            ("title", "Perfis Gamer e Gravação de Iluminação"),
            ("body", "O teclado inclui três perfis pré-programados para jogos, além da gravação de mapa customizado diretamente na memória interna."),
            ("h2", "Perfis Gamer Integrados"),
            ("key", "FN + 1!", "Modo FPS — Ilumina W, A, S, D e as 4 teclas direcionais."),
            ("key", "FN + 2@", "Modo LOL / MOBA — Ilumina Q, W, E, R, D, F, G, V, B, Tab, Espaço, 1–6 e Esc."),
            ("key", "FN + 3#", "Modo Escritório — Ilumina as 26 letras (A–Z), pontuação e setas."),
            ("h2", "Como Gravar Iluminação Personalizada"),
            ("bullet", "1. Pressione FN + 1!, FN + 2@ ou FN + 3# para selecionar a posição que deseja customizar."),
            ("bullet", "2. Pressione FN + ~ (Til) para entrar no modo de gravação. O LED indicador começará a piscar."),
            ("bullet", "3. Pressione as teclas que deseja ligar ou desligar individualmente."),
            ("bullet", "4. Pressione FN + ~ novamente para salvar o mapa na memória EEPROM do teclado."),
        ]

    elif topic_id == "battery_power":
        return [
            ("title", "Bateria e Economia de Energia"),
            ("body", "Com bateria interna de 4000 mAh, o K8 oferece autonomia prolongada e gerenciamento inteligente de energia."),
            ("h2", "Autonomia da Bateria"),
            ("table_row", "Capacidade", "4000 mAh Íon de Lítio Recarregável"),
            ("table_row", "Com RGB Ligado", "Aprox. 15 dias de uso diário normal"),
            ("table_row", "Com RGB Desligado", "Até 35 dias de uso contínuo"),
            ("table_row", "Tempo de Carga", "Aprox. 4 a 5 horas via porta USB 5V/1A"),
            ("h2", "Modos Inteligentes de Suspensão"),
            ("bullet", "Repouso de 5 Minutos: O RGB desliga automaticamente após 5 minutos de inatividade."),
            ("bullet", "Hibernação de 30 Minutos: O teclado entra em modo de sono profundo após 30 minutos sem toques."),
            ("bullet", "Como Acordar: Pressione qualquer tecla duas vezes para reativar a conexão imediatamente."),
        ]

    elif topic_id == "hotswap_maintenance":
        return [
            ("title", "Troca de Switches Hot-Swap e Manutenção"),
            ("body", "O FREEWOLF K8 possui soquetes hot-swap universais compatíveis com switches mecânicos de 3 pinos sem solda."),
            ("h2", "Compatibilidade de Switches"),
            ("bullet", "Compatível com switches padrão de 3 pinos: Outemu, Gateron, Cherry MX, Kailh, AKKO, etc."),
            ("bullet", "Acompanha dois switches sobressalentes, extrator de keycaps e extrator de switches."),
            ("h2", "Passo a Passo para Substituição"),
            ("bullet", "1. Remover Keycap: Encaixe o extrator de keycaps e puxe verticalmente para cima."),
            ("bullet", "2. Remover Switch: Encaixe as garras do extrator nas travas superior e inferior do switch. Aperte suavemente e puxe reto para cima, sem entortar."),
            ("bullet", "3. Inspecionar Pinos: Verifique se os 2 pinos de cobre do novo switch estão perfeitamente retos."),
            ("bullet", "4. Inserir Switch: Alinhe os pinos com os orifícios da placa e pressione reto até ouvir o clique de encaixe."),
            ("bullet", "5. Testar: Teste o acionamento da tecla antes de recolocar a keycap."),
        ]

    elif topic_id == "troubleshooting":
        return [
            ("title", "Guia de Solução de Problemas"),
            ("body", "Diagnósticos e soluções recomendadas com base nas diretrizes oficiais e arquitetura do driver Linux."),
            ("h2", "1. Teclado não responde no modo USB"),
            ("bullet", "Verifique o seletor: confira se a chave traseira está em 'USB' ou 'Wired'."),
            ("bullet", "Conexão do cabo: certifique-se de que o cabo USB-C está bem plugado."),
            ("bullet", "Permissões Linux: se o badge mostrar 'Permissão Necessária', execute 'k8ctl setup-udev' para liberar o acesso a /dev/hidraw* sem sudo."),
            ("h2", "2. Sem fio 2.4 GHz não conecta"),
            ("bullet", "Receptor conectado: verifique se o nano-receptor USB está em uma porta USB funcionando."),
            ("bullet", "Seletor: confira se a chave traseira está em '2.4G'."),
            ("bullet", "Sincronizar: segure FN + R por 3 a 5 segundos até o LED piscar rapidamente."),
            ("h2", "3. Falha na conexão ou pareamento Bluetooth"),
            ("bullet", "Seletor em BT: certifique-se de que a chave traseira está em 'BT'."),
            ("bullet", "Modo Pareamento: segure FN + Q, FN + W ou FN + E por 5 segundos até o LED piscar rápido. No PC/celular, remova pareamentos antigos e busque novamente."),
            ("h2", "4. Iluminação RGB apagada ou fraca"),
            ("bullet", "Ajuste o brilho: aperte FN + ↑ várias vezes para aumentar a intensidade."),
            ("bullet", "Alterne modos: pressione FN + | para ciclar entre os efeitos."),
            ("bullet", "Bateria fraca: no modo sem fio, bateria fraca desliga o LED para economizar energia. Conecte o cabo USB para recarregar."),
            ("h2", "5. Uma tecla específica parou de funcionar"),
            ("bullet", "Remova a keycap e o switch com os extratores incluídos. Verifique se os pinos de contato entortaram. Desdobre com cuidado ou troque pelo switch reserva."),
        ]

    elif topic_id == "linux_tools":
        return [
            ("title", "Driver Nativo Linux e Ferramentas CLI (k8ctl)"),
            ("body", "Este projeto fornece controle nativo completo no Linux, sem necessidade de programas Windows ou Wine."),
            ("h2", "Identificação de Hardware"),
            ("table_row", "USB VID", "0x1A2C"),
            ("table_row", "USB PID", "0x7C80"),
            ("table_row", "Endpoint", "HID Interface 1 (/dev/hidraw*) — EP 0x02 Saída, EP 0x82 Entrada"),
            ("table_row", "Macros", "Módulo de kernel Linux /dev/uinput"),
            ("table_row", "Visualizador de Áudio", "Fluxo monitor nativo PulseAudio / PipeWire"),
            ("h2", "Configuração de Permissões (k8ctl setup-udev)"),
            ("body", "O Linux restringe o acesso direto a hidraw e uinput por padrão. Execute o comando de configuração uma única vez:"),
            ("code", "k8ctl setup-udev"),
            ("body", "O script instala a regra /etc/udev/rules.d/99-freewolf-k8.rules, liberando o dispositivo para usuários normais sem necessidade de sudo."),
            ("h2", "Comandos da Ferramenta de Linha de Comando (k8ctl)"),
            ("code", "k8ctl status"),
            ("bullet", "Exibe status da conexão e dispositivo detectado."),
            ("code", "k8ctl list"),
            ("bullet", "Lista todos os 21 modos de iluminação com seus IDs."),
            ("code", "k8ctl set <id|nome> [--brightness 0-4] [--speed 0-4]"),
            ("bullet", "Configura modo, brilho e velocidade da iluminação via terminal."),
            ("code", "k8ctl music [--pattern 1|2] [--delay ms]"),
            ("bullet", "Inicia o visualizador de áudio em tempo real pelo terminal."),
            ("code", "k8ctl macro list / k8ctl macro play <id>"),
            ("bullet", "Gerencia e executa macros diretamente via terminal ou scripts."),
        ]

    return []


def _get_content_es(topic_id: str) -> List[Tuple[str, ...]]:
    if topic_id == "overview":
        return [
            ("title", "FREEWOLF K8 — Visión General y Especificaciones Técnicas"),
            ("body", "El FREEWOLF K8 es un teclado mecánico para gaming de alto rendimiento en formato 80% (100 teclas), con conectividad Tri-Modo universal (cable USB-C, inalámbrico 2.4 GHz y Bluetooth 5.0 con 3 perfiles). Equipado con zócalos de interruptores mecánicos intercambiables en caliente (hot-swap), retroiluminación RGB tecla por tecla y una batería recargable de 4000 mAh, ofrece máxima productividad y rendimiento para juegos en Linux, Windows y macOS."),
            ("h2", "Especificaciones de Hardware"),
            ("table_row", "Modelo", "FREEWOLF K8 Tri-Mode Mechanical Keyboard"),
            ("table_row", "Formato", "80% Compacto (100 teclas con teclado numérico integrado)"),
            ("table_row", "Dimensiones", "395 mm × 142 mm × 40 mm (15,55 × 5,59 × 1,57 in)"),
            ("table_row", "Peso", "Aproximadamente 850 gramos"),
            ("table_row", "Interruptores", "Mecánicos Hot-Swap (Zócalos estándar de 3 pines, Blue Switch)"),
            ("table_row", "Teclas (Keycaps)", "PBT Double-Shot (texturizadas, resistentes al desgaste)"),
            ("table_row", "Anti-Ghosting", "Full-Key Rollover (NKRO / 100% anti-ghosting)"),
            ("table_row", "Batería", "4000 mAh de iones de litio recargable"),
            ("table_row", "Conectividad", "Tri-Modo: Cable USB-C, Inalámbrico 2.4 GHz, Bluetooth 5.0 (BT1, BT2, BT3)"),
            ("table_row", "ID de Hardware USB", "VID: 0x1A2C  |  PID: 0x7C80  (Interfaz 1 HID)"),
            ("table_row", "Compatibilidad", "Linux, Windows 11/10/8/7, macOS, Android, iOS"),
            ("h2", "Contenido del Paquete"),
            ("bullet", "Teclado mecánico para gaming FREEWOLF K8"),
            ("bullet", "Cable trenzado de conexión y carga USB-C a USB-A"),
            ("bullet", "Receptor nano inalámbrico USB 2.4 GHz (guardado en la base magnética)"),
            ("bullet", "Extractor de teclas de alambre"),
            ("bullet", "Extractor metálico de interruptores (switches)"),
            ("bullet", "Dos (2) interruptores mecánicos de repuesto de 3 pines"),
            ("bullet", "Manual de usuario oficial y guía de inicio rápido"),
            ("tip", "La base del teclado incluye patas retráctiles con ajuste ergonómico de inclinación en dos niveles y almohadillas antideslizantes para mayor estabilidad y comodidad."),
        ]

    elif topic_id == "connectivity":
        return [
            ("title", "Conectividad Tri-Modo y Multidispositivo"),
            ("body", "El teclado K8 permite conectar hasta cinco dispositivos simultáneamente mediante tres modos de conexión: Cable USB-C, Inalámbrico 2.4 GHz y Bluetooth 5.0 (3 canales)."),
            ("h2", "1. Modo Cableado USB-C"),
            ("bullet", "Coloque el interruptor selector (en la parte trasera/lateral) en 'USB' o 'Wired'."),
            ("bullet", "Conecte el cable USB-C al teclado y el extremo USB-A al ordenador."),
            ("bullet", "Reconocimiento automático en Linux con latencia cero de entrada y carga simultánea de la batería."),
            ("h2", "2. Modo Inalámbrico 2.4 GHz de Ultrabaja Latencia"),
            ("bullet", "Retire el nano-receptor 2.4 GHz de la ranura magnética en la parte inferior del teclado."),
            ("bullet", "Conecte el receptor a un puerto USB disponible del ordenador."),
            ("bullet", "Coloque el interruptor selector en '2.4G'."),
            ("bullet", "La conexión se establece automáticamente. En caso de pérdida de sincronización:"),
            ("key", "FN + R", "Mantenga presionado durante 3–5 segundos para iniciar el emparejamiento 2.4 GHz (el indicador parpadea rápidamente)."),
            ("h2", "3. Modo Bluetooth 5.0 (3 Dispositivos Emparejados)"),
            ("bullet", "Coloque el interruptor selector en 'BT'."),
            ("bullet", "Para emparejar un dispositivo en una de las 3 ranuras de memoria Bluetooth:"),
            ("key", "FN + Q", "Mantenga presionado 3–5 s para el Canal BT 1 (el LED parpadea rápido; empareje 'FREEWOLF K8' en el sistema)."),
            ("key", "FN + W", "Mantenga presionado 3–5 s para el Canal BT 2 (el LED parpadea rápido; empareje 'FREEWOLF K8' en el sistema)."),
            ("key", "FN + E", "Mantenga presionado 3–5 s para el Canal BT 3 (el LED parpadea rápido; empareje 'FREEWOLF K8' en el sistema)."),
            ("bullet", "Para alternar rápidamente entre los dispositivos Bluetooth emparejados:"),
            ("key", "FN + Q / W / E", "Toque brevemente para cambiar al instante entre Dispositivo 1, 2 o 3."),
            ("tip", "Al cambiar de dispositivo Bluetooth, el indicador parpadea lentamente una vez y permanece fijo al reconectarse."),
        ]

    elif topic_id == "os_layouts":
        return [
            ("title", "Modos de Sistema Operativo y Atajos de Función"),
            ("body", "El FREEWOLF K8 incluye perfiles de diseño dedicados por hardware para Windows y macOS, junto con una fila completa de funciones multimedia F1–F12."),
            ("h2", "Cambio de Diseño de Sistema Operativo"),
            ("key", "FN + A", "Modo Windows (Diseño estándar de PC, tecla Win activa, Ctrl/Alt estándar)."),
            ("key", "FN + S", "Modo macOS (Intercambia teclas Option y Command según el diseño nativo de Mac)."),
            ("key", "FN + Win", "Bloqueo / Desbloqueo de Tecla Windows (Modo Gaming: desactiva Win para evitar salidas accidentales)."),
            ("h2", "Atajos Multimedia F1 – F12"),
            ("table_row", "FN + F1", "Abrir reproductor multimedia predeterminado"),
            ("table_row", "FN + F2", "Bajar volumen"),
            ("table_row", "FN + F3", "Subir volumen"),
            ("table_row", "FN + F4", "Silenciar audio (Mute)"),
            ("table_row", "FN + F5", "Pista anterior"),
            ("table_row", "FN + F6", "Pista siguiente"),
            ("table_row", "FN + F7", "Reproducir / Pausar"),
            ("table_row", "FN + F8", "Detener reproducción"),
            ("table_row", "FN + F9", "Abrir navegador web"),
            ("table_row", "FN + F10", "Abrir cliente de correo electrónico"),
            ("table_row", "FN + F11", "Abrir Explorador de archivos / Mi Equipo"),
            ("table_row", "FN + F12", "Abrir calculadora"),
        ]

    elif topic_id == "rgb_lighting":
        return [
            ("title", "Iluminación RGB y Atajos de Hardware"),
            ("body", "El teclado K8 cuenta con 21 modos de iluminación RGB distintos, incluyendo 19 animaciones dinámicas, iluminación estática y modo de visualizador musical por software."),
            ("h2", "Controles de Iluminación Integrados"),
            ("key", "FN + |", "Alternar entre los 19 efectos dinámicos RGB (Estático, Respiración, Flujo Neón, Onda, Ondulaciones, etc.)."),
            ("key", "FN + ↑", "Aumentar brillo de la retroiluminación (5 niveles: 0% / Apagado a 100%)."),
            ("key", "FN + ↓", "Disminuir brillo de la retroiluminación. El nivel 0 apaga completamente los LEDs."),
            ("key", "FN + →", "Aumentar velocidad de animación (5 pasos dinámicos)."),
            ("key", "FN + ←", "Disminuir velocidad de animación (5 pasos dinámicos)."),
            ("h2", "Control por Software en Linux"),
            ("bullet", "En la pestaña 'Iluminación' de esta app, haga clic en cualquier efecto y ajuste brillo (0–4) y retardo (0–4) en tiempo real."),
            ("bullet", "Modo Música: Transmite el espectro de frecuencias FFT de PulseAudio/PipeWire directamente a los LEDs del teclado."),
            ("bullet", "En la terminal: ejecute 'k8ctl set <id|nombre> --brightness <0-4> --speed <0-4>'."),
            ("tip", "Ajustar el brillo a 0 apaga los LEDs por completo, extendiendo la batería hasta 35 días en modo inalámbrico."),
        ]

    elif topic_id == "gaming_presets":
        return [
            ("title", "Perfiles de Iluminación para Juegos y Grabación Personalizada"),
            ("body", "El teclado cuenta con tres perfiles de iluminación preprogramados de fábrica para juegos, además de la capacidad de grabar mapas personalizados directamente en la memoria interna."),
            ("h2", "Perfiles Gamer Integrados"),
            ("key", "FN + 1!", "Modo FPS — Ilumina W, A, S, D y las 4 teclas de dirección."),
            ("key", "FN + 2@", "Modo LOL / MOBA — Ilumina Q, W, E, R, D, F, G, V, B, Tab, Espacio, 1–6 y Esc."),
            ("key", "FN + 3#", "Modo Oficina — Ilumina las 26 letras (A–Z), puntuación y teclas de dirección."),
            ("h2", "Cómo Grabar Mapas de Iluminación Personalizados"),
            ("bullet", "Paso 1: Presione FN + 1!, FN + 2@ o FN + 3# para seleccionar el perfil que desea personalizar."),
            ("bullet", "Paso 2: Presione FN + ~ (Virgulilla) para ingresar al modo de grabación. El LED indicador parpadeará rápidamente."),
            ("bullet", "Paso 3: Presione cualquier tecla del teclado para encender o apagar su LED individualmente."),
            ("bullet", "Paso 4: Presione FN + ~ nuevamente para guardar el patrón en la memoria EEPROM no volátil del teclado."),
            ("tip", "Los mapas grabados con FN + ~ se conservan al apagar el teclado y funcionan en modos con cable e inalámbrico sin software adicional."),
        ]

    elif topic_id == "battery_power":
        return [
            ("title", "Especificaciones de Batería y Ahorro de Energía"),
            ("body", "Alimentado por una batería recargable de iones de litio de alta capacidad de 4000 mAh, el K8 ofrece una excelente autonomía y gestión inteligente de energía."),
            ("h2", "Duración de la Batería"),
            ("table_row", "Capacidad de la Batería", "4000 mAh de iones de litio recargable"),
            ("table_row", "Con RGB Encendido", "Aprox. 15 días (uso diario estándar de juegos/oficina)"),
            ("table_row", "Con RGB Apagado", "Hasta 35 días de uso continuo"),
            ("table_row", "Tiempo de Carga", "Aprox. 4 a 5 horas mediante puerto USB 5V/1A"),
            ("h2", "Modos Inteligentes de Suspensión"),
            ("bullet", "Reposo de 5 minutos: Tras 5 minutos sin presionar teclas, la retroiluminación RGB se apaga automáticamente para ahorrar energía."),
            ("bullet", "Hibernación de 30 minutos: Tras 30 minutos de inactividad, el teclado entra en modo de hibernación de consumo ultrabajo."),
            ("bullet", "Reactivación Instantánea: Presione cualquier tecla dos veces para reactivar la conexión y continuar escribiendo de inmediato."),
            ("h2", "Carga e Indicadores LED"),
            ("bullet", "Conecte el cable USB-C a cualquier puerto USB del ordenador o adaptador de pared de 5V."),
            ("bullet", "El LED indicador se ilumina durante la carga y se apaga al completarse al 100%."),
            ("tip", "Para maximizar la vida útil de la batería, evite dejarla completamente descargada durante períodos prolongados."),
        ]

    elif topic_id == "hotswap_maintenance":
        return [
            ("title", "Interruptores Hot-Swap y Mantenimiento"),
            ("body", "El FREEWOLF K8 cuenta con zócalos universales hot-swap en la placa base compatibles con interruptores mecánicos de 3 pines sin necesidad de soldar."),
            ("h2", "Compatibilidad de Interruptores"),
            ("bullet", "Compatible con interruptores mecánicos estándar de 3 pines (Outemu, Gateron, Cherry MX, Kailh, AKKO, etc.)."),
            ("bullet", "Incluye dos (2) interruptores de repuesto, un extractor de teclas y un extractor metálico de interruptores en el paquete."),
            ("h2", "Reemplazo de Interruptores Paso a Paso"),
            ("bullet", "1. Quitar Tecla: Enganche el extractor de teclas por debajo de las esquinas y tire verticalmente hacia arriba."),
            ("bullet", "2. Quitar Interruptor: Coloque las puntas del extractor en las pestañas superior e inferior del interruptor. Presione suavemente y tire en línea recta hacia arriba sin doblar."),
            ("bullet", "3. Inspeccionar Pines: Verifique los dos pines de contacto de cobre en la parte inferior del interruptor nuevo. Asegúrese de que ambos estén 100% rectos."),
            ("bullet", "4. Instalar Interruptor: Alinee los pines metálicos con los orificios del zócalo de la placa. Presione firmemente hacia abajo hasta que encaje en la placa metálica."),
            ("bullet", "5. Probar y Colocar Tecla: Pruebe el registro de la tecla antes de volver a colocar la tecla (keycap)."),
            ("h2", "Limpieza y Cuidados"),
            ("bullet", "Desconecte siempre el cable y apague el interruptor de energía inalámbrica antes de limpiar."),
            ("bullet", "Use aire comprimido o un cepillo suave para eliminar polvo y residuos entre las teclas."),
            ("bullet", "Limpie las teclas con un paño de microfibra ligeramente humedecido. Nunca use alcohol, acetona ni disolventes agresivos."),
            ("tip", "Nunca fuerce un interruptor en el zócalo si siente resistencia. Retírelo y verifique que los pines de cobre no se hayan doblado."),
        ]

    elif topic_id == "troubleshooting":
        return [
            ("title", "Guía de Solución de Problemas y Diagnósticos"),
            ("body", "Problemas frecuentes, pasos de diagnóstico y soluciones basados en las directrices oficiales del fabricante y la arquitectura del driver Linux."),
            ("h2", "1. El teclado no responde en Modo Cableado"),
            ("bullet", "Selector de modo: Asegúrese de que el interruptor trasero/lateral esté en 'USB' o 'Wired'."),
            ("bullet", "Conexión del cable: Verifique que el cable USB-C esté firmemente insertado tanto en el teclado como en el ordenador."),
            ("bullet", "Permisos en Linux: Si el indicador muestra 'Permisos Requeridos', ejecute 'k8ctl setup-udev' para otorgar acceso sin root a /dev/hidraw*."),
            ("h2", "2. El modo inalámbrico 2.4 GHz no se conecta"),
            ("bullet", "Ubicación del receptor: Asegúrese de que el nano-receptor USB 2.4G esté conectado a un puerto USB funcional."),
            ("bullet", "Interruptor de modo: Coloque el interruptor en '2.4G'."),
            ("bullet", "Reemparejamiento: Mantenga presionado FN + R durante 3 a 5 segundos hasta que el indicador parpadee rápidamente, luego acerque el teclado al receptor."),
            ("h2", "3. La conexión o emparejamiento Bluetooth falla"),
            ("bullet", "Selector en BT: Asegúrese de que el interruptor esté en 'BT'."),
            ("bullet", "Modo de emparejamiento: Mantenga presionado FN + Q, FN + W o FN + E durante 3 a 5 segundos hasta que el LED parpadee rápidamente. En su ordenador o teléfono, elimine cualquier entrada previa de 'FREEWOLF K8' y vuelva a buscar."),
            ("bullet", "Interferencias: Asegúrese de estar dentro de un rango de 10 metros y evite obstáculos densos."),
            ("h2", "4. La retroiluminación RGB no enciende o está tenue"),
            ("bullet", "Nivel de brillo: Presione FN + ↑ varias veces para aumentar el brillo (puede estar configurado en 0/apagado)."),
            ("bullet", "Cambiar efectos: Presione FN + | para ciclar entre los efectos luminosos."),
            ("bullet", "Ahorro de energía: Si estuvo inactivo más de 5 minutos, presione cualquier tecla dos veces para reactivarlo."),
            ("bullet", "Batería baja: En modo inalámbrico, la batería baja apaga los LEDs automáticamente. Conecte el cable USB-C para cargar."),
            ("h2", "5. Una tecla específica no responde"),
            ("bullet", "Extraiga la tecla y el interruptor con las herramientas incluidas. Verifique los pines de cobre inferiores; si están doblados, enderécelos con cuidado con pinzas o instale uno de los interruptores de repuesto."),
        ]

    elif topic_id == "linux_tools":
        return [
            ("title", "Driver Nativo Linux y Utilidad CLI (k8ctl)"),
            ("body", "Esta aplicación proporciona un driver completamente nativo para Linux, configurador GUI y utilidad de línea de comandos para el teclado FREEWOLF K8, sin depender de software Windows ni Wine."),
            ("h2", "Arquitectura de Hardware"),
            ("table_row", "USB Vendor ID", "0x1A2C"),
            ("table_row", "USB Product ID", "0x7C80"),
            ("table_row", "Interfaz de Control", "HID Interfaz 1 (/dev/hidraw*) — EP 0x02 Salida, EP 0x82 Entrada"),
            ("table_row", "Subsistema de Macros", "Dispositivo de teclado virtual Linux Kernel /dev/uinput"),
            ("table_row", "Captura de Audio", "Flujo monitor nativo PulseAudio / PipeWire"),
            ("h2", "Permisos en Linux (k8ctl setup-udev)"),
            ("body", "Linux restringe el acceso directo a hidraw y uinput por defecto. Ejecute el comando de configuración una sola vez:"),
            ("code", "k8ctl setup-udev"),
            ("body", "Esto instala la regla /etc/udev/rules.d/99-freewolf-k8.rules y asigna uaccess al teclado, permitiendo el uso normal sin requerir sudo ni root."),
            ("h2", "Herramienta de Línea de Comandos (k8ctl)"),
            ("body", "Controle su teclado mediante scripts, terminal o atajos:"),
            ("code", "k8ctl status"),
            ("bullet", "Muestra el estado de detección, nodo de dispositivo y disponibilidad del driver."),
            ("code", "k8ctl list"),
            ("bullet", "Lista los 21 modos de iluminación con sus identificadores."),
            ("code", "k8ctl set <id|nombre> [--brightness 0-4] [--speed 0-4]"),
            ("bullet", "Configura modo, brillo y retardo/velocidad. Ejemplo: 'k8ctl set 2 --brightness 4 --speed 2'."),
            ("code", "k8ctl music [--pattern 1|2] [--delay ms]"),
            ("bullet", "Transmite el visualizador de audio en tiempo real desde la terminal."),
            ("code", "k8ctl macro list"),
            ("bullet", "Lista todas las macros guardadas."),
            ("code", "k8ctl macro play <id>"),
            ("bullet", "Ejecuta macros usando el dispositivo de teclado virtual del kernel."),
        ]

    return []


def _get_content_fr(topic_id: str) -> List[Tuple[str, ...]]:
    if topic_id == "overview":
        return [
            ("title", "FREEWOLF K8 — Aperçu Technique et Spécifications"),
            ("body", "Le FREEWOLF K8 est un clavier mécanique de jeu haute performance au format 80% (100 touches), doté d'une connectivité Tri-Mode universelle (USB-C filaire, sans fil 2,4 GHz et Bluetooth 5.0 avec 3 profils). Équipé de sockets mécaniques remplaçables à chaud (hot-swap), d'un rétroéclairage RVB touche par touche et d'une batterie rechargeable de 4000 mAh, il assure une excellente productivité et expérience de jeu sous Linux, Windows et macOS."),
            ("h2", "Spécifications Matérielles"),
            ("table_row", "Modèle", "FREEWOLF K8 Tri-Mode Mechanical Keyboard"),
            ("table_row", "Format", "80% Compact (100 touches avec pavé numérique intégré)"),
            ("table_row", "Dimensions", "395 mm × 142 mm × 40 mm (15,55 × 5,59 × 1,57 po)"),
            ("table_row", "Poids", "Environ 850 grammes"),
            ("table_row", "Switchs", "Mécaniques Hot-Swap (Sockets standard 3 broches, Blue Switch)"),
            ("table_row", "Touches (Keycaps)", "PBT Double-Injection (texturées, haute durabilité)"),
            ("table_row", "Anti-Ghosting", "Full-Key Rollover (NKRO / 100% anti-ghosting sans blocage)"),
            ("table_row", "Batterie", "4000 mAh Lithium-Ion Rechargeable"),
            ("table_row", "Connectivité", "Tri-Mode : USB-C Filaire, Sans Fil 2,4 GHz, Bluetooth 5.0 (BT1, BT2, BT3)"),
            ("table_row", "Identifiant USB", "VID: 0x1A2C  |  PID: 0x7C80  (Interface 1 HID)"),
            ("table_row", "Compatibilité", "Linux, Windows 11/10/8/7, macOS, Android, iOS"),
            ("h2", "Contenu de la Boîte"),
            ("bullet", "Clavier Mécanique Gamer FREEWOLF K8"),
            ("bullet", "Câble Tressé USB-C vers USB-A pour connexion et charge"),
            ("bullet", "Nano-Récepteur USB Sans Fil 2,4 GHz (logé dans la base magnétique)"),
            ("bullet", "Extracteur de Touches en fil métallique"),
            ("bullet", "Extracteur Métallique de Switchs"),
            ("bullet", "Deux (2) Switchs Mécaniques de Rechange à 3 broches"),
            ("bullet", "Manuel d'Utilisation Officiel et Guide de Démarrage"),
            ("tip", "La base du clavier comprend des pieds rétractables ergonomiques à deux niveaux d'inclinaison et des patins antidérapants pour un confort et une stabilité accrus."),
        ]

    elif topic_id == "connectivity":
        return [
            ("title", "Connectivité Tri-Mode et Multi-Appareils"),
            ("body", "Le clavier K8 permet de connecter jusqu'à cinq appareils simultanément grâce à trois modes de connexion : Câble USB-C, Sans Fil 2,4 GHz et Bluetooth 5.0 (3 canaux)."),
            ("h2", "1. Mode Filaire USB-C"),
            ("bullet", "Réglez le commutateur matériel (à l'arrière/sur le côté) sur 'USB' ou 'Wired'."),
            ("bullet", "Branchez le câble USB-C au clavier et la fiche USB-A à votre ordinateur."),
            ("bullet", "Détection instantanée sous Linux avec zéro latence et chargement simultané de la batterie."),
            ("h2", "2. Mode Sans Fil 2,4 GHz à Très Faible Latence"),
            ("bullet", "Retirez le nano-récepteur 2,4 GHz de son logement magnétique sous le clavier."),
            ("bullet", "Branchez le récepteur sur un port USB disponible de l'ordinateur."),
            ("bullet", "Réglez le commutateur sur '2.4G'."),
            ("bullet", "Connexion automatique. En cas de perte de signal :"),
            ("key", "FN + R", "Maintenez enfoncé pendant 3 à 5 secondes pour réinitialiser l'appairage 2,4 GHz (le voyant clignote rapidement)."),
            ("h2", "3. Mode Bluetooth 5.0 (3 Appareils Mémorisés)"),
            ("bullet", "Réglez le commutateur sur 'BT'."),
            ("bullet", "Pour appairer un appareil sur l'un des 3 profils mémorisés :"),
            ("key", "FN + Q", "Appui long 3 à 5 s pour le Canal 1 (la LED clignote vite ; appairez 'FREEWOLF K8' sur l'hôte)."),
            ("key", "FN + W", "Appui long 3 à 5 s pour le Canal 2 (la LED clignote vite ; appairez 'FREEWOLF K8' sur l'hôte)."),
            ("key", "FN + E", "Appui long 3 à 5 s pour le Canal 3 (la LED clignote vite ; appairez 'FREEWOLF K8' sur l'hôte)."),
            ("bullet", "Pour basculer rapidement entre les appareils appairés :"),
            ("key", "FN + Q / W / E", "Appui court pour basculer instantanément vers l'Appareil 1, 2 ou 3."),
            ("tip", "Lors du basculement entre appareils Bluetooth, le voyant clignote lentement une fois puis reste allumé en continu."),
        ]

    elif topic_id == "os_layouts":
        return [
            ("title", "Modes de Système d'Exploitation et Raccourcis Multimédia"),
            ("body", "Le FREEWOLF K8 intègre des profils matériels dédiés pour Windows et macOS, ainsi qu'une rangée complète de touches multimédias F1 à F12."),
            ("h2", "Basculement de Système d'Exploitation"),
            ("key", "FN + A", "Passer en Mode Windows (Disposition PC standard, touche Win active, Ctrl/Alt standard)."),
            ("key", "FN + S", "Passer en Mode macOS (Inverse les touches Option et Commande pour la disposition native Mac)."),
            ("key", "FN + Win", "Verrouillage / Déverrouillage Touche Windows (Mode Gamer pour éviter les retours intempestifs sur le bureau)."),
            ("h2", "Raccourcis Multimédia F1 à F12"),
            ("table_row", "FN + F1", "Ouvrir le lecteur multimédia par défaut"),
            ("table_row", "FN + F2", "Diminuer le volume"),
            ("table_row", "FN + F3", "Augmenter le volume"),
            ("table_row", "FN + F4", "Couper le son (Muet)"),
            ("table_row", "FN + F5", "Piste précédente"),
            ("table_row", "FN + F6", "Piste suivante"),
            ("table_row", "FN + F7", "Lecture / Pause"),
            ("table_row", "FN + F8", "Arrêter la lecture"),
            ("table_row", "FN + F9", "Ouvrir le navigateur web"),
            ("table_row", "FN + F10", "Ouvrir le client de messagerie"),
            ("table_row", "FN + F11", "Ouvrir le gestionnaire de fichiers / Mon Ordinateur"),
            ("table_row", "FN + F12", "Ouvrir la calculatrice"),
        ]

    elif topic_id == "rgb_lighting":
        return [
            ("title", "Éclairage RVB et Raccourcis Matériels"),
            ("body", "Le clavier K8 propose 21 modes d'éclairage RVB, dont 19 animations dynamiques intégrées, un rétroéclairage fixe et un mode visualiseur audio logiciel."),
            ("h2", "Contrôles de l'Éclairage au Clavier"),
            ("key", "FN + |", "Faire défiler les 19 effets RVB dynamiques (Fixe, Respiration, Flux Néon, Vague, Ondulations, etc.)."),
            ("key", "FN + ↑", "Augmenter la luminosité (5 niveaux : 0% / Éteint à 100%)."),
            ("key", "FN + ↓", "Diminuer la luminosité. Le niveau 0 éteint complètement les LED."),
            ("key", "FN + →", "Augmenter la vitesse d'animation (5 paliers dynamiques)."),
            ("key", "FN + ←", "Diminuer la vitesse d'animation (5 paliers dynamiques)."),
            ("h2", "Contrôle Logiciel sous Linux"),
            ("bullet", "Dans l'onglet 'Éclairage' de cette application, cliquez directement sur n'importe quel effet et ajustez luminosité et vitesse en temps réel."),
            ("bullet", "Mode Musique : Transmet le spectre FFT audio de PulseAudio/PipeWire directement vers les LED du clavier."),
            ("bullet", "Dans le terminal : lancez 'k8ctl set <id|nom> --brightness <0-4> --speed <0-4>'."),
            ("tip", "Régler la luminosité sur 0 coupe l'alimentation des LED, prolongeant l'autonomie jusqu'à 35 jours en mode sans fil."),
        ]

    elif topic_id == "gaming_presets":
        return [
            ("title", "Profils de Jeu et Enregistrement Personnalisé"),
            ("body", "Le clavier dispose de trois profils d'éclairage préprogrammés pour le jeu, ainsi que de la possibilité d'enregistrer des motifs personnalisés directement dans la mémoire interne."),
            ("h2", "Profils Gamer Intégrés"),
            ("key", "FN + 1!", "Mode FPS — Allume W, A, S, D et les 4 touches fléchées."),
            ("key", "FN + 2@", "Mode LOL / MOBA — Allume Q, W, E, R, D, F, G, V, B, Tab, Espace, 1–6 et Échap."),
            ("key", "FN + 3#", "Mode Bureautique — Allume les 26 lettres (A–Z), la ponctuation et les flèches."),
            ("h2", "Enregistrer un Profil d'Éclairage Personnalisé"),
            ("bullet", "Étape 1 : Appuyez sur FN + 1!, FN + 2@ ou FN + 3# pour choisir l'emplacement à personnaliser."),
            ("bullet", "Étape 2 : Appuyez sur FN + ~ (Tilde) pour entrer en mode d'enregistrement. Le voyant LED commence à clignoter rapidement."),
            ("bullet", "Étape 3 : Appuyez sur les touches souhaitées pour activer ou éteindre individuellement leur LED."),
            ("bullet", "Étape 4 : Appuyez à nouveau sur FN + ~ pour sauvegarder le profil dans la mémoire EEPROM du clavier."),
            ("tip", "Les profils enregistrés avec FN + ~ restent en mémoire après extinction et fonctionnent avec ou sans fil sans aucun logiciel."),
        ]

    elif topic_id == "battery_power":
        return [
            ("title", "Batterie et Économie d'Énergie"),
            ("body", "Doté d'une batterie lithium-ion haute capacité de 4000 mAh, le K8 offre une excellente autonomie et une gestion intelligente de l'énergie à plusieurs niveaux."),
            ("h2", "Autonomie de la Batterie"),
            ("table_row", "Capacité de la Batterie", "4000 mAh Lithium-Ion Rechargeable"),
            ("table_row", "Avec RVB Allumé", "Environ 15 jours (utilisation quotidienne standard)"),
            ("table_row", "Avec RVB Éteint", "Jusqu'à 35 jours d'utilisation continue"),
            ("table_row", "Temps de Charge", "Env. 4 à 5 heures via un port USB 5V/1A"),
            ("h2", "Modes de Veille Intelligents"),
            ("bullet", "Veille après 5 minutes : Après 5 minutes d'inactivité, l'éclairage RVB s'éteint automatiquement pour économiser l'énergie."),
            ("bullet", "Hibernation après 30 minutes : Après 30 minutes sans frappe, le clavier entre en veille profonde ultra-économique."),
            ("bullet", "Sortie de Veille Instantanée : Appuyez deux fois sur n'importe quelle touche pour réactiver la liaison immédiatement."),
            ("h2", "Recharge et Indicateurs LED"),
            ("bullet", "Branchez le câble USB-C fourni sur un port USB d'ordinateur ou un adaptateur secteur 5V standard."),
            ("bullet", "Le voyant LED reste allumé pendant la charge et s'éteint une fois la batterie pleine à 100%."),
            ("tip", "Pour maximiser la longévité de la batterie, évitez de la laisser entièrement déchargée pendant de longues périodes."),
        ]

    elif topic_id == "hotswap_maintenance":
        return [
            ("title", "Switchs Amovibles Hot-Swap et Entretien"),
            ("body", "Le FREEWOLF K8 intègre des sockets hot-swap universels compatibles avec les switchs mécaniques standard à 3 broches sans aucune soudure."),
            ("h2", "Compatibilité des Switchs"),
            ("bullet", "Compatible avec les switchs mécaniques 3 broches (Outemu, Gateron, Cherry MX, Kailh, AKKO, etc.)."),
            ("bullet", "Deux (2) switchs de rechange, un extracteur de touches et un extracteur métallique de switchs sont fournis."),
            ("h2", "Remplacement des Switchs Étape par Étape"),
            ("bullet", "1. Retirer la Touche : Placez l'extracteur de touche sous les coins et tirez droit vers le haut."),
            ("bullet", "2. Retirer le Switch : Placez les griffes de l'extracteur sur les ergots supérieur et inférieur du switch. Pressez délicatement et tirez droit vers le haut sans tordre."),
            ("bullet", "3. Inspecter les Broches : Vérifiez les deux broches en cuivre du nouveau switch pour vous assurer qu'elles sont parfaitement droites."),
            ("bullet", "4. Insérer le Switch : Alignez les broches avec les trous du socket et enfoncez fermement jusqu'à l'enclenchement dans la plaque métallique."),
            ("bullet", "5. Tester et Remonter : Vérifiez la frappe de la touche avant de réinstaller la touche (keycap)."),
            ("h2", "Nettoyage et Entretien"),
            ("bullet", "Débranchez toujours le câble et éteignez le commutateur sans fil avant tout nettoyage."),
            ("bullet", "Utilisez de l'air comprimé ou une brosse souple pour éliminer la poussière entre les touches."),
            ("bullet", "Nettoyez les touches avec un chiffon en microfibre légèrement humide. N'utilisez jamais d'alcool ni de solvants agressifs."),
            ("tip", "Ne forcez jamais un switch dans son socket en cas de résistance. Retirez-le et assurez-vous que les broches de cuivre ne sont pas pliées."),
        ]

    elif topic_id == "troubleshooting":
        return [
            ("title", "Guide de Dépannage et Solutions"),
            ("body", "Problèmes fréquents, étapes de diagnostic et solutions recommandées d'après les manuels officiels et l'architecture du pilote Linux."),
            ("h2", "1. Le clavier ne répond pas en Mode Filaire"),
            ("bullet", "Commutateur de mode : Vérifiez que le sélecteur arrière/latéral est positionné sur 'USB' ou 'Wired'."),
            ("bullet", "Branchement du câble : Vérifiez que le câble USB-C est bien enfoncé dans le clavier et l'ordinateur."),
            ("bullet", "Autorisations sous Linux : Si le statut indique 'Permissions Nécessaires', lancez 'k8ctl setup-udev' pour autoriser l'accès sans root à /dev/hidraw*."),
            ("h2", "2. Le mode sans fil 2,4 GHz ne se connecte pas"),
            ("bullet", "Branchement du dongle : Assurez-vous que le nano-récepteur USB 2,4G est bien branché sur un port USB fonctionnel."),
            ("bullet", "Commutateur de mode : Positionnez le sélecteur sur '2.4G'."),
            ("bullet", "Réappairage : Maintenez FN + R pendant 3 à 5 secondes jusqu'à ce que le voyant clignote rapidement, puis approchez le clavier du récepteur."),
            ("h2", "3. Échec de connexion ou d'appairage Bluetooth"),
            ("bullet", "Commutateur sur BT : Vérifiez que le sélecteur est bien sur 'BT'."),
            ("bullet", "Mode Appairage : Maintenez FN + Q, FN + W ou FN + E pendant 3 à 5 secondes jusqu'au clignotement rapide. Sur votre appareil, supprimez les anciens profils 'FREEWOLF K8' et relancez la recherche."),
            ("bullet", "Portée : Veillez à rester dans un rayon de 10 mètres sans obstacle massif."),
            ("h2", "4. Le rétroéclairage RVB est éteint ou trop faible"),
            ("bullet", "Niveau de luminosité : Appuyez plusieurs fois sur FN + ↑ pour augmenter l'intensité (elle était peut-être à 0)."),
            ("bullet", "Changement de mode : Appuyez sur FN + | pour faire défiler les effets."),
            ("bullet", "Économie d'énergie : En cas d'inactivité de plus de 5 minutes, appuyez deux fois sur une touche pour réactiver le clavier."),
            ("bullet", "Batterie faible : En mode sans fil, une batterie faible coupe automatiquement les LED. Branchez le câble USB-C pour recharger."),
            ("h2", "5. Une touche spécifique ne fonctionne plus"),
            ("bullet", "Retirez la touche et le switch à l'aide des outils fournis. Vérifiez si les broches en cuivre sont pliées. Redressez-les avec une pince ou remplacez le switch par l'un des switchs de rechange inclus."),
        ]

    elif topic_id == "linux_tools":
        return [
            ("title", "Pilote Natif Linux et Outil CLI (k8ctl)"),
            ("body", "Cette suite fournit un pilote entièrement natif sous Linux, une interface graphique de configuration et un outil en ligne de commande pour le clavier FREEWOLF K8, sans logiciel Windows ni Wine."),
            ("h2", "Architecture Matérielle"),
            ("table_row", "USB Vendor ID", "0x1A2C"),
            ("table_row", "USB Product ID", "0x7C80"),
            ("table_row", "Interface de Contrôle", "HID Interface 1 (/dev/hidraw*) — Sortie EP 0x02, Entrée EP 0x82"),
            ("table_row", "Sous-système Macro", "Périphérique de clavier virtuel Linux Kernel /dev/uinput"),
            ("table_row", "Capture Audio", "Flux moniteur natif PulseAudio / PipeWire"),
            ("h2", "Autorisations sous Linux (k8ctl setup-udev)"),
            ("body", "Linux restreint par défaut l'accès direct aux nœuds hidraw et uinput. Lancez la configuration une seule fois :"),
            ("code", "k8ctl setup-udev"),
            ("body", "Cette commande installe la règle /etc/udev/rules.d/99-freewolf-k8.rules et attribue uaccess au clavier pour une utilisation sans sudo ni root."),
            ("h2", "Outil en Ligne de Commande (k8ctl)"),
            ("body", "Contrôlez votre clavier depuis vos scripts, votre terminal ou vos raccourcis :"),
            ("code", "k8ctl status"),
            ("bullet", "Affiche l'état de détection, le nœud matériel et la disponibilité du pilote."),
            ("code", "k8ctl list"),
            ("bullet", "Liste l'ensemble des 21 modes d'éclairage avec leurs identifiants."),
            ("code", "k8ctl set <id|nom> [--brightness 0-4] [--speed 0-4]"),
            ("bullet", "Configure le mode, la luminosité et la vitesse. Exemple : 'k8ctl set 2 --brightness 4 --speed 2'."),
            ("code", "k8ctl music [--pattern 1|2] [--delay ms]"),
            ("bullet", "Diffuse le visualiseur audio en temps réel depuis le terminal."),
            ("code", "k8ctl macro list"),
            ("bullet", "Liste toutes les macros enregistrées."),
            ("code", "k8ctl macro play <id>"),
            ("bullet", "Exécute une macro via le clavier virtuel du noyau Linux."),
        ]

    return []


def _get_content_de(topic_id: str) -> List[Tuple[str, ...]]:
    if topic_id == "overview":
        return [
            ("title", "FREEWOLF K8 — Technische Übersicht und Spezifikationen"),
            ("body", "Die FREEWOLF K8 ist eine leistungsstarke mechanische Gaming-Tastatur im 80%-Kompaktformat (100 Tasten) mit universeller Drei-Modus-Konnektivität (USB-C-Kabel, 2,4-GHz-Funk und Bluetooth 5.0 mit 3 Profilen). Ausgestattet mit Hot-Swap-fähigen mechanischen Switch-Sockeln, Einzeltasten-RGB-Beleuchtung und einem 4000-mAh-Akku bietet sie maximale Produktivität und Gaming-Leistung unter Linux, Windows und macOS."),
            ("h2", "Hardware-Spezifikationen"),
            ("table_row", "Modell", "FREEWOLF K8 Tri-Mode Mechanical Keyboard"),
            ("table_row", "Layout", "80% Kompakt (100 Tasten mit integriertem Ziffernblock)"),
            ("table_row", "Abmessungen", "395 mm × 142 mm × 40 mm (15,55 × 5,59 × 1,57 Zoll)"),
            ("table_row", "Gewicht", "Ca. 850 Gramm"),
            ("table_row", "Schalter (Switches)", "Mechanisch Hot-Swap (Standard 3-Pin-Sockel, Blue Switch)"),
            ("table_row", "Tastenkappen (Keycaps)", "PBT Double-Injection (texturiert, verschleißfest)"),
            ("table_row", "Anti-Ghosting", "Full-Key Rollover (NKRO / 100% Anti-Ghosting ohne Blockaden)"),
            ("table_row", "Akku", "4000 mAh Lithium-Ionen wiederaufladbar"),
            ("table_row", "Konnektivität", "Drei-Modus: USB-C-Kabel, 2,4 GHz kabellos, Bluetooth 5.0 (BT1, BT2, BT3)"),
            ("table_row", "USB-Hardware-ID", "VID: 0x1A2C  |  PID: 0x7C80  (Interface 1 HID)"),
            ("table_row", "Kompatibilität", "Linux, Windows 11/10/8/7, macOS, Android, iOS"),
            ("h2", "Lieferumfang"),
            ("bullet", "Mechanische Gaming-Tastatur FREEWOLF K8"),
            ("bullet", "Umflochtenes USB-C-auf-USB-A-Verbindungs- und Ladekabel"),
            ("bullet", "2,4-GHz-USB-Funk-Nano-Empfänger (im Magnetfach auf der Unterseite)"),
            ("bullet", "Präzisions-Tastenkappenabzieher aus Draht"),
            ("bullet", "Metallischer Switch-Abzieher"),
            ("bullet", "Zwei (2) mechanische Ersatz-Switches (3-Pin)"),
            ("bullet", "Offizielles Benutzerhandbuch und Schnellstartanleitung"),
            ("tip", "Die Unterseite der Tastatur verfügt über zweistufig verstellbare ergonomische Klappfüße und rutschfeste Gummipolster für sicheren Stand und optimalen Schreibkomfort."),
        ]

    elif topic_id == "connectivity":
        return [
            ("title", "Drei-Modus-Verbindung und Multi-Geräte-Betrieb"),
            ("body", "Die K8-Tastatur unterstützt bis zu fünf Geräte gleichzeitig über drei Verbindungsmodi: USB-C-Kabel, 2,4-GHz-Funk und Bluetooth 5.0 (3 Kanäle)."),
            ("h2", "1. USB-C-Kabelmodus"),
            ("bullet", "Schieben Sie den Hardware-Schalter auf der Rückseite/Seite auf 'USB' oder 'Wired'."),
            ("bullet", "Verbinden Sie das USB-C-Kabel mit der Tastatur und den USB-A-Stecker mit dem Computer."),
            ("bullet", "Sofortige Erkennung unter Linux ohne Eingabeverzögerung bei gleichzeitigem Laden des Akkus."),
            ("h2", "2. 2,4-GHz-Funkmodus mit extrem geringer Latenz"),
            ("bullet", "Entnehmen Sie den 2,4-GHz-Nano-Empfänger aus dem magnetischen Aufbewahrungsfach auf der Unterseite."),
            ("bullet", "Stecken Sie den Empfänger in einen freien USB-Port Ihres Computers."),
            ("bullet", "Schieben Sie den Schalter auf '2.4G'."),
            ("bullet", "Die Verbindung erfolgt automatisch. Bei Signalverlust oder zur Neukopplung:"),
            ("key", "FN + R", "3–5 Sekunden gedrückt halten, um das 2,4-GHz-Pairing zu starten (Anzeige blinkt schnell)."),
            ("h2", "3. Bluetooth 5.0-Modus (3 gekoppelte Geräte)"),
            ("bullet", "Schieben Sie den Schalter auf 'BT'."),
            ("bullet", "So koppeln Sie ein Gerät auf einem der 3 Bluetooth-Speicherplätze:"),
            ("key", "FN + Q", "3–5 Sek. lang drücken für BT-Kanal 1 (LED blinkt schnell; koppeln Sie 'FREEWOLF K8' am Gerät)."),
            ("key", "FN + W", "3–5 Sek. lang drücken für BT-Kanal 2 (LED blinkt schnell; koppeln Sie 'FREEWOLF K8' am Gerät)."),
            ("key", "FN + E", "3–5 Sek. lang drücken für BT-Kanal 3 (LED blinkt schnell; koppeln Sie 'FREEWOLF K8' am Gerät)."),
            ("bullet", "So wechseln Sie blitzschnell zwischen gekoppelten Bluetooth-Geräten:"),
            ("key", "FN + Q / W / E", "Kurz drücken, um sofort zu Gerät 1, Gerät 2 oder Gerät 3 zu wechseln."),
            ("tip", "Beim Umschalten zwischen Bluetooth-Geräten blinkt die Anzeige einmal langsam auf und leuchtet nach erfolgreicher Wiederverbindung dauerhaft."),
        ]

    elif topic_id == "os_layouts":
        return [
            ("title", "Betriebssystem-Layouts und Multimedia-Tastenkombinationen"),
            ("body", "Die FREEWOLF K8 bietet fest integrierte Layout-Profile für Windows und macOS sowie eine vollständige Reihe von F1–F12 Multimedia-Funktionen."),
            ("h2", "Betriebssystem-Umschaltung"),
            ("key", "FN + A", "Windows-Modus aktivieren (Standard-PC-Layout, aktive Windows-Taste, Standard-Strg/Alt)."),
            ("key", "FN + S", "macOS-Modus aktivieren (Tauscht Option- und Command-Tasten für das native Mac-Layout)."),
            ("key", "FN + Win", "Windows-Tastensperre / Gaming-Modus (Deaktiviert Win-Taste gegen versehentliche Spielunterbrechungen)."),
            ("h2", "Multimedia-Tastenkombinationen F1 – F12"),
            ("table_row", "FN + F1", "Standard-Medienplayer öffnen"),
            ("table_row", "FN + F2", "Lautstärke verringern"),
            ("table_row", "FN + F3", "Lautstärke erhöhen"),
            ("table_row", "FN + F4", "Ton stummschalten"),
            ("table_row", "FN + F5", "Vorheriger Titel"),
            ("table_row", "FN + F6", "Nächster Titel"),
            ("table_row", "FN + F7", "Wiedergabe / Pause"),
            ("table_row", "FN + F8", "Wiedergabe stoppen"),
            ("table_row", "FN + F9", "Webbrowser öffnen"),
            ("table_row", "FN + F10", "E-Mail-Programm öffnen"),
            ("table_row", "FN + F11", "Dateimanager / Arbeitsplatz öffnen"),
            ("table_row", "FN + F12", "Taschenrechner öffnen"),
        ]

    elif topic_id == "rgb_lighting":
        return [
            ("title", "RGB-Beleuchtung und Hardware-Tastenkombinationen"),
            ("body", "Die K8-Tastatur verfügt über 21 verschiedene RGB-Beleuchtungsmodi, darunter 19 dynamische Animationen, statische Beleuchtung und softwaregesteuerten Musik-Visualisierer."),
            ("h2", "Integrierte Beleuchtungssteuerung"),
            ("key", "FN + |", "Durchschalten der 19 dynamischen RGB-Effekte (Statisch, Atmung, Neon-Strom, Welle, Wellenringe usw.)."),
            ("key", "FN + ↑", "Helligkeit erhöhen (5 Stufen: 0% / Aus bis 100%)."),
            ("key", "FN + ↓", "Helligkeit verringern. Stufe 0 schaltet alle LEDs komplett ab."),
            ("key", "FN + →", "Animationsgeschwindigkeit erhöhen (5 dynamische Stufen)."),
            ("key", "FN + ←", "Animationsgeschwindigkeit verringern (5 dynamische Stufen)."),
            ("h2", "Software-Steuerung unter Linux"),
            ("bullet", "Auf dem Reiter 'Beleuchtung' können Sie jeden Modus direkt anklicken und Helligkeit (0–4) sowie Tempo (0–4) ohne Verzögerung einstellen."),
            ("bullet", "Musikmodus: Wandelt das PulseAudio/PipeWire-Audiosignal in Echtzeit in LED-Lichtwellen auf der Tastatur um."),
            ("bullet", "Im Terminal: 'k8ctl set <id|name> --brightness <0-4> --speed <0-4>' ausführen."),
            ("tip", "Das Reduzieren der Helligkeit auf 0 schaltet die LEDs ab und verlängert die Akkulaufzeit im Funkbetrieb auf bis zu 35 Tage."),
        ]

    elif topic_id == "gaming_presets":
        return [
            ("title", "Gaming-Beleuchtungsprofile und eigene Aufnahme"),
            ("body", "Die Tastatur enthält drei werksseitig vorprogrammierte Gaming-Beleuchtungsprofile sowie die Möglichkeit, eigene Beleuchtungsmasken direkt im Onboard-Speicher abzulegen."),
            ("h2", "Integrierte Gaming-Profile"),
            ("key", "FN + 1!", "FPS-Modus — Beleuchtet W, A, S, D und die 4 Pfeiltasten."),
            ("key", "FN + 2@", "LOL / MOBA-Modus — Beleuchtet Q, W, E, R, D, F, G, V, B, Tab, Leertaste, 1–6 und Esc."),
            ("key", "FN + 3#", "Office-Modus — Beleuchtet alle 26 Buchstaben (A–Z), Satzzeichen und Pfeiltasten."),
            ("h2", "Eigene Beleuchtungsmasken aufnehmen"),
            ("bullet", "Schritt 1: Drücken Sie FN + 1!, FN + 2@ oder FN + 3#, um den gewünschten Speicherplatz auszuwählen."),
            ("bullet", "Schritt 2: Drücken Sie FN + ~ (Tilde), um den Aufnahmemodus zu starten. Die LED-Anzeige beginnt schnell zu blinken."),
            ("bullet", "Schritt 3: Drücken Sie die Tasten, deren Beleuchtung Sie individuell ein- oder ausschalten möchten."),
            ("bullet", "Schritt 4: Drücken Sie erneut FN + ~, um das Muster dauerhaft im internen EEPROM-Speicher der Tastatur zu sichern."),
            ("tip", "Über FN + ~ gespeicherte Profile bleiben nach dem Ausschalten erhalten und funktionieren im Kabel- und Funkbetrieb ohne zusätzliche Software."),
        ]

    elif topic_id == "battery_power":
        return [
            ("title", "Akkuspezifikationen und Energiesparmodi"),
            ("body", "Ausgestattet mit einem 4000-mAh-Lithium-Ionen-Akku bietet die K8 lange kabellose Laufzeiten und ein intelligentes mehrstufiges Energiemanagement."),
            ("h2", "Akkulaufzeit"),
            ("table_row", "Akkukapazität", "4000 mAh Lithium-Ionen wiederaufladbar"),
            ("table_row", "Mit aktiver RGB-Beleuchtung", "Ca. 15 Tage (bei typischer täglicher Nutzung)"),
            ("table_row", "Ohne Beleuchtung (LEDs aus)", "Bis zu 35 Tage kontinuierliche Nutzung"),
            ("table_row", "Ladezeit", "Ca. 4–5 Stunden über einen 5V/1A USB-Anschluss"),
            ("h2", "Intelligente Ruhemodi"),
            ("bullet", "5-Minuten-Ruhezustand: Nach 5 Minuten ohne Tastenbetätigung schaltet sich die RGB-Beleuchtung automatisch ab."),
            ("bullet", "30-Minuten-Tiefschlaf: Nach 30 Minuten Inaktivität wechselt die Tastatur in den stromsparenden Tiefschlafmodus."),
            ("bullet", "Sofortiges Aufwecken: Drücken Sie eine beliebige Taste zweimal, um die Verbindung sofort wieder zu aktivieren."),
            ("h2", "Laden und LED-Anzeigen"),
            ("bullet", "Schließen Sie das mitgelieferte USB-C-Kabel an einen USB-Port des Computers oder ein 5V-Netzteil an."),
            ("bullet", "Die Lade-LED leuchtet während des Ladevorgangs und erlischt, sobald der Akku zu 100% geladen ist."),
            ("tip", "Vermeiden Sie es, den Akku über längere Zeiträume vollständig entladen zu lagern, um seine Lebensdauer zu maximieren."),
        ]

    elif topic_id == "hotswap_maintenance":
        return [
            ("title", "Hot-Swap-Schalter und Pflege"),
            ("body", "Die FREEWOLF K8 besitzt universelle Hot-Swap-Sockel auf der Platine, die den werkzeuglosen Wechsel von 3-Pin-Schaltern ohne Lötarbeiten ermöglichen."),
            ("h2", "Schalter-Kompatibilität"),
            ("bullet", "Unterstützt mechanische 3-Pin-Standard-Switches (Outemu, Gateron, Cherry MX, Kailh, AKKO usw.)."),
            ("bullet", "Zwei (2) Ersatz-Switches, ein Draht-Tastenkappenabzieher und ein Schalterabzieher aus Metall sind im Lieferumfang enthalten."),
            ("h2", "Schritt-für-Schritt-Schalterwechsel"),
            ("bullet", "1. Tastenkappe entfernen: Tastenkappenabzieher unter den Ecken ansetzen und senkrecht nach oben ziehen."),
            ("bullet", "2. Schalter entfernen: Zangen des Schalterabziehers an der oberen und unteren Verriegelungslasche ansetzen, sanft zusammendrücken und gerade nach oben herausziehen."),
            ("bullet", "3. Kontakte prüfen: Prüfen Sie die beiden Kupferkontakte des neuen Schalters auf einwandfreie Geradheit."),
            ("bullet", "4. Schalter einsetzen: Pins an den Sockelöffnungen ausrichten und fest nach unten drücken, bis der Switch in der Metallplatte einrastet."),
            ("bullet", "5. Testen und Montieren: Tastenfunktion vor dem Aufsetzen der Tastenkappe überprüfen."),
            ("h2", "Reinigung und Pflege"),
            ("bullet", "Vor dem Reinigen stets das Kabel trennen und den Funkschalter ausschalten."),
            ("bullet", "Druckluft oder einen weichen Pinsel verwenden, um Staub und Partikel zwischen den Tasten zu entfernen."),
            ("bullet", "Tastenkappen mit einem leicht angefeuchteten Mikrofasertuch abwischen. Niemals Alkohol oder scharfe Lösungsmittel verwenden."),
            ("tip", "Niemals Gewalt anwenden, falls beim Einsetzen Widerstand spürbar ist. Schalter herausnehmen und Kupferpins auf Verbiegung prüfen."),
        ]

    elif topic_id == "troubleshooting":
        return [
            ("title", "Fehlerbehebung und Lösungen"),
            ("body", "Häufige Fragen, Diagnosehinweise und Lösungen basierend auf den Herstellerhandbüchern und der Linux-Treiberarchitektur."),
            ("h2", "1. Tastatur reagiert im Kabelmodus nicht"),
            ("bullet", "Schalterstellung prüfen: Vergewissern Sie sich, dass der Schalter auf 'USB' oder 'Wired' steht."),
            ("bullet", "Kabelverbindung: Prüfen Sie, ob das USB-C-Kabel fest in Tastatur und Computer eingesteckt ist."),
            ("bullet", "Linux-Berechtigungen: Zeigt das Statussymbol 'Berechtigung erforderlich', führen Sie 'k8ctl setup-udev' aus, um den Zugriff auf /dev/hidraw* freizuschalten."),
            ("h2", "2. 2,4-GHz-Funk stellt keine Verbindung her"),
            ("bullet", "Empfänger prüfen: Stellen Sie sicher, dass der 2,4G-USB-Nano-Empfänger an einem funktionierenden USB-Port angeschlossen ist."),
            ("bullet", "Schalterstellung: Schieben Sie den Schalter auf '2.4G'."),
            ("bullet", "Neukopplung: Halten Sie FN + R für 3 bis 5 Sekunden gedrückt, bis die Anzeige schnell blinkt, und bringen Sie die Tastatur nah an den Empfänger."),
            ("h2", "3. Bluetooth-Kopplung oder -Verbindung schlägt fehl"),
            ("bullet", "Schalter auf BT: Vergewissern Sie sich, dass der Schalter auf 'BT' steht."),
            ("bullet", "Kopplungsmodus: Halten Sie FN + Q, FN + W oder FN + E für 3–5 Sekunden gedrückt, bis die LED schnell blinkt. Löschen Sie alte 'FREEWOLF K8'-Einträge auf Ihrem Gerät und scannen Sie neu."),
            ("bullet", "Reichweite: Achten Sie auf einen Abstand unter 10 Metern ohne dichte Hindernisse."),
            ("h2", "4. RGB-Beleuchtung leuchtet nicht oder ist schwach"),
            ("bullet", "Helligkeitsstufe: Drücken Sie mehrfach FN + ↑, um die Helligkeit zu erhöhen (eventuell war sie auf 0/aus gestellt)."),
            ("bullet", "Effekt wechseln: Drücken Sie FN + |, um die Beleuchtungsmodi durchzuschalten."),
            ("bullet", "Energiesparmodus: Bei Inaktivität über 5 Minuten drücken Sie eine beliebige Taste zweimal, um die Beleuchtung aufzuwecken."),
            ("bullet", "Niedriger Akkustand: Im Funkbetrieb schaltet ein schwacher Akku die LEDs ab. Schließen Sie das USB-C-Kabel zum Laden an."),
            ("h2", "5. Eine bestimmte Taste löst nicht aus"),
            ("bullet", "Ziehen Sie Tastenkappe und Switch mit den beiliegenden Abziehern ab. Überprüfen Sie die unteren Kupferpins auf Verbiegungen. Richten Sie sie vorsichtig mit einer Pinzette aus oder setzen Sie einen der beiliegenden Ersatz-Switches ein."),
        ]

    elif topic_id == "linux_tools":
        return [
            ("title", "Nativer Linux-Treiber und CLI-Werkzeug (k8ctl)"),
            ("body", "Diese Anwendung bietet einen vollständig nativen Linux-Treiber, eine GUI-Konfiguration und ein Befehlszeilenwerkzeug für die FREEWOLF K8-Tastatur, ganz ohne Windows-Software oder Wine."),
            ("h2", "Hardware-Architektur"),
            ("table_row", "USB-Hersteller-ID (VID)", "0x1A2C"),
            ("table_row", "USB-Produkt-ID (PID)", "0x7C80"),
            ("table_row", "Steuerschnittstelle", "HID Interface 1 (/dev/hidraw*) — Ausgabe EP 0x02, Eingabe EP 0x82"),
            ("table_row", "Makro-Subsystem", "Linux Kernel /dev/uinput virtuelles Tastaturgerät"),
            ("table_row", "Audio-Erfassung", "Nativer PulseAudio / PipeWire Monitor-Stream"),
            ("h2", "Linux-Berechtigungen (k8ctl setup-udev)"),
            ("body", "Linux beschränkt den Direktzugriff auf hidraw- und uinput-Geräteknoten standardmäßig. Führen Sie den Setup-Befehl einmalig aus:"),
            ("code", "k8ctl setup-udev"),
            ("body", "Dies installiert /etc/udev/rules.d/99-freewolf-k8.rules und versieht das Gerät mit uaccess, sodass es ohne sudo oder Root-Rechte genutzt werden kann."),
            ("h2", "Befehlszeilenwerkzeug (k8ctl)"),
            ("body", "Steuern Sie Ihre Tastatur aus Skripten, dem Terminal oder per Tastenkürzel:"),
            ("code", "k8ctl status"),
            ("bullet", "Zeigt Erkennungsstatus, Geräteknoten und Treiberbereitschaft."),
            ("code", "k8ctl list"),
            ("bullet", "Listet alle 21 Beleuchtungsmodi mit ihren IDs auf."),
            ("code", "k8ctl set <id|name> [--brightness 0-4] [--speed 0-4]"),
            ("bullet", "Stellt Modus, Helligkeit und Tempo ein. Beispiel: 'k8ctl set 2 --brightness 4 --speed 2'."),
            ("code", "k8ctl music [--pattern 1|2] [--delay ms]"),
            ("bullet", "Startet den Echtzeit-Audio-Visualisierer direkt aus dem Terminal."),
            ("code", "k8ctl macro list"),
            ("bullet", "Listet alle gespeicherten Makros auf."),
            ("code", "k8ctl macro play <id>"),
            ("bullet", "Führt Makros über das virtuelle Tastaturgerät des Linux-Kernels aus."),
        ]

    return []
