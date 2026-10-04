"""
CLI Interface for FREE WOLF K8 Controller
"""
import sys
import time
import argparse
from .protocol import LIGHT_MODES, MODE_BY_NAME, MODE_BY_ID, LightMode
from .driver import FreeWolfK8Driver, DeviceState
from .music import MusicVisualizerEngine
from .udev import check_access, run_setup_cli

def print_status(driver: FreeWolfK8Driver):
    state, detail = driver.probe()
    print("========================================")
    print(" FREE WOLF K8 Linux Controller Status   ")
    print("========================================")
    print(f"Device State   : {state.upper()}")
    print(f"Device Node    : {driver.device_node or 'None'}")
    print(f"Detail         : {detail}")
    print("----------------------------------------")
    if state == DeviceState.PERMISSION_DENIED:
        print("Tip: Run 'k8ctl setup-udev' to configure permissions.")
    elif state == DeviceState.CLAIMED_BY_VM:
        print("Tip: Keyboard is currently attached to QEMU. Close the VM or detach USB to control it natively.")
    elif state == DeviceState.CONNECTED:
        print("Status: Keyboard is ready for native Linux configuration.")
    print("========================================")

def list_modes():
    print(f"{'ID':>2} | {'Wire':>4} | {'Mode Name':<22} | {'Brightness':<10} | {'Speed':<8} | Description")
    print("-" * 80)
    for m in LIGHT_MODES:
        br_str = "Yes (0-4)" if m.has_brightness else "No"
        sp_str = "Yes (0-4)" if m.has_speed else "No"
        if m.is_music:
            sp_str = "Rate (ms)"
        print(f"{m.id:2d} | 0x{m.wire_id:02x} | {m.name:<22} | {br_str:<10} | {sp_str:<8} | {m.description}")

def find_mode(mode_arg: str) -> LightMode:
    # Try ID
    try:
        val = int(mode_arg)
        if val in MODE_BY_ID:
            return MODE_BY_ID[val]
    except ValueError:
        pass

    # Try name exact or substring
    q = mode_arg.lower().replace(" ", "_")
    for k, m in MODE_BY_NAME.items():
        if q == k or q in k:
            return m

    raise ValueError(f"Unknown mode '{mode_arg}'. Run 'k8ctl list' to see all valid modes.")

TOPIC_KEY_MAP = {
    "specs": 0, "overview": 0, "specifications": 0, "hardware": 0, "0": 0,
    "connect": 1, "connectivity": 1, "wireless": 1, "bluetooth": 1, "bt": 1, "1": 1,
    "hotkeys": 2, "os": 2, "layout": 2, "windows": 2, "mac": 2, "f1": 2, "2": 2,
    "lighting": 3, "rgb": 3, "light": 3, "led": 3, "3": 3,
    "gaming": 4, "presets": 4, "fps": 4, "lol": 4, "moba": 4, "custom": 4, "4": 4,
    "battery": 5, "power": 5, "sleep": 5, "charging": 5, "5": 5,
    "switches": 6, "hotswap": 6, "switch": 6, "care": 6, "keys": 6, "6": 6,
    "troubleshoot": 7, "troubleshooting": 7, "fix": 7, "faq": 7, "7": 7,
    "driver": 8, "linux": 8, "cli": 8, "udev": 8, "8": 8,
}

def print_full_cli_help(driver=None):
    """Prints comprehensive usage, status, command examples, and shortcuts."""
    status_line = "Checking..."
    if driver is None:
        driver = FreeWolfK8Driver()
    state, detail = driver.probe()
    if state == DeviceState.CONNECTED:
        status_line = f"\033[1;32mCONNECTED\033[0m ({driver.device_node}, VID: 0x1A2C, PID: 0x7C80)"
    elif state == DeviceState.PERMISSION_DENIED:
        status_line = f"\033[1;31mPERMISSION DENIED\033[0m ({driver.device_node}) - Run 'k8ctl setup-udev'"
    elif state == DeviceState.CLAIMED_BY_VM:
        status_line = "\033[1;33mCLAIMED BY VM\033[0m (Attached to QEMU guest)"
    else:
        status_line = "\033[1;30mDISCONNECTED\033[0m (Check USB cable or 2.4G wireless dongle)"

    print("=" * 80)
    print(" FREE WOLF K8 Linux Controller (k8ctl)")
    print(" Native CLI for FREE WOLF K8 Tri-Mode (USB / 2.4G / BT5.0) Mechanical Keyboard")
    print("=" * 80)
    print(f"Device State : {status_line}")
    print("-" * 80)
    print("USAGE:")
    print("  k8ctl <command> [arguments...] [options...]")
    print()
    print("AVAILABLE COMMANDS:")
    print("  status                     Display keyboard connection, permissions, and hidraw info")
    print("  setup-udev                 Install udev rules and configure device permissions (sudo)")
    print("  list                       List all 21 lighting modes with IDs, parameters & descriptions")
    print("  set <mode> [options]       Set active RGB lighting mode by name or numeric ID (0-20)")
    print("                             Options:")
    print("                               -b, --brightness <0-4>  Brightness level (0=Off, 4=Max, default: 4)")
    print("                               -s, --speed <0-4>       Animation speed (0=Slow, 4=Fast, default: 4)")
    print("  music [options]            Stream live real-time audio FFT visualizer to keyboard LEDs")
    print("                             Options:")
    print("                               -m, --submode <1|2>     Visualizer pattern (1=Center, 2=Spectrum, def: 2)")
    print("                               -d, --delay <ms>        Update interval in ms (33, 66, 100, default: 66)")
    print("  macro <action> [options]   Manage and play back macros via Linux /dev/uinput virtual keyboard")
    print("                             Actions:")
    print("                               list                    List all saved macros")
    print("                               show <id>               Show keystroke timeline for a macro")
    print("                               play <id> [-r N] [-d S] Execute macro keystrokes into active window")
    print("                               export <id> <file.json> Export macro to JSON file")
    print("                               import <file.json>      Import macro from JSON file")
    print("                               delete <id>             Delete a saved macro")
    print("  help [topic]               Display user manual & technical documentation")
    print("                             Topics: specs, connect, hotkeys, lighting, gaming, battery,")
    print("                                     switches, troubleshoot, driver, or topic index 0-8")
    print("  udev                       Print udev rules for unprivileged /dev/hidraw and /dev/uinput")
    print()
    print("COMMAND EXAMPLES:")
    print("  k8ctl status                         # Check device connection and driver readiness")
    print("  k8ctl setup-udev                     # Configure Linux permissions for USB and macros")
    print("  k8ctl list                           # View all 21 lighting modes")
    print("  k8ctl set breathing -b 3 -s 2        # Set Breathing mode (Brightness 3, Speed 2)")
    print("  k8ctl set 'neon stream' -b 4         # Set Neon Stream mode at max brightness")
    print("  k8ctl set steady -b 4                # Set full static backlighting")
    print("  k8ctl set 0                          # Turn off all RGB backlights (sleep/battery saver)")
    print("  k8ctl music -m 2                     # Launch live audio spectrum visualizer")
    print("  k8ctl macro list                     # List recorded macros")
    print("  k8ctl macro play 1 -r 2 -d 2.0       # Play macro ID 1 twice after 2s countdown")
    print("  k8ctl help hotkeys                   # View F1-F12 and Windows/macOS key shortcuts")
    print()
    print("HARDWARE SHORTCUTS (Fn):")
    print("  Fn + \\|         Cycle through 19 dynamic RGB lighting effects")
    print("  Fn + ↑ / ↓     Adjust brightness (0% / Off to 100%)")
    print("  Fn + ← / →     Adjust animation speed (5 dynamic steps)")
    print("  Fn + Backspace Toggle all backlighting On / Off")
    print("  Fn + 1 / 2 / 3 Gaming presets: 1=FPS, 2=LOL/MOBA, 3=Office (37 keys)")
    print("  Fn + ~         Record custom lighting map to onboard EEPROM (Hold Fn+~)")
    print("  Fn + A / S     Switch between Windows Mode (Fn+A) and macOS Mode (Fn+S)")
    print("  Fn + Win       Toggle Windows Key Lock / Gaming Mode")
    print("  Fn + Q / W / E Switch to Bluetooth profile 1 / 2 / 3 (Hold 3-5s to pair)")
    print("  Fn + R         2.4 GHz wireless pairing mode (Hold 3-5s)")
    print("=" * 80)

def print_topic_help(topic_arg: str):
    """Prints manual topic content formatted for terminal viewing."""
    key = topic_arg.lower().strip()
    idx = TOPIC_KEY_MAP.get(key)
    if idx is None:
        try:
            val = int(key)
            if 0 <= val <= 8:
                idx = val
        except ValueError:
            pass
    if idx is None:
        print(f"Unknown topic '{topic_arg}'.")
        print("Available topics: specs, connect, hotkeys, lighting, gaming, battery, switches, troubleshoot, driver (or 0-8)")
        return

    from .manual import get_topic_titles, get_topic_content
    titles = get_topic_titles("en")
    icon, title = titles[idx]

    print("=" * 80)
    print(f" {icon}  {title.upper()}")
    print("=" * 80)

    blocks = get_topic_content(idx, "en")
    for block in blocks:
        btype = block[0]
        if btype == "title":
            print(f"\n{block[1]}")
            print("-" * len(block[1]))
        elif btype == "h2":
            print(f"\n\033[1;34m▶ {block[1]}\033[0m")
        elif btype == "body":
            print(f"\n{block[1]}")
        elif btype == "bullet":
            print(f"  • {block[1]}")
        elif btype == "key":
            print(f"  \033[1;32m[{block[1]:<16}]\033[0m  {block[2]}")
        elif btype == "table_row":
            print(f"  \033[1m{block[1]:<18}\033[0m : {block[2]}")
        elif btype == "code":
            print(f"    \033[1;36m{block[1]}\033[0m")
        elif btype == "tip":
            print(f"\n  \033[1;33m💡 Tip: {block[1]}\033[0m")
    print("=" * 80)

def main():
    driver = FreeWolfK8Driver()

    # When no option is given, display the comprehensive help guide
    if len(sys.argv) <= 1:
        print_full_cli_help(driver)
        return

    parser = argparse.ArgumentParser(
        description="FREE WOLF K8 Linux Configuration Utility",
        add_help=False
    )
    parser.add_argument("-h", "--help", action="store_true", help="Show help and usage guide")
    subparsers = parser.add_subparsers(dest="command")

    # help
    help_p = subparsers.add_parser("help", help="Display user manual & documentation topics")
    help_p.add_argument("topic", nargs="?", default=None, help="Topic name or number (specs, connect, hotkeys, lighting, gaming, battery, switches, troubleshoot, driver, 0-8)")

    # status
    subparsers.add_parser("status", help="Show keyboard connection and driver status")

    # list
    subparsers.add_parser("list", help="List all lighting modes")

    # udev
    subparsers.add_parser("udev", help="Print udev rules for unprivileged access")
    subparsers.add_parser("setup-udev", help="Install udev rules and configure device permissions (sudo)")
    subparsers.add_parser("setup_udev", help="Alias for setup-udev")

    # set
    set_parser = subparsers.add_parser("set", help="Set lighting mode")
    set_parser.add_argument("mode", help="Mode name or ID (e.g. 'neon_stream', 'breathing', '4')")
    set_parser.add_argument("-b", "--brightness", type=int, default=4, choices=range(0, 5), help="Brightness level (0-4, 0=off, default: 4)")
    set_parser.add_argument("-s", "--speed", type=int, default=4, choices=range(0, 5), help="Speed level (0-4, default: 4)")

    # music
    music_parser = subparsers.add_parser("music", help="Stream real-time Music Mode visualizer frames")
    music_parser.add_argument("-m", "--submode", type=int, default=2, choices=[1, 2], help="Music submode (1 or 2, default: 2)")
    music_parser.add_argument("-d", "--delay", type=int, default=66, choices=[33, 66, 100], help="Transmission interval in ms (default: 66)")

    # macro
    macro_parser = subparsers.add_parser("macro", help="Manage macros")
    macro_sub = macro_parser.add_subparsers(dest="macro_action")

    # macro list
    macro_sub.add_parser("list", help="List all saved macros")

    # macro show
    show_p = macro_sub.add_parser("show", help="Show details of a macro")
    show_p.add_argument("id", type=int, help="Macro ID")

    # macro export
    exp_p = macro_sub.add_parser("export", help="Export a macro to JSON")
    exp_p.add_argument("id", type=int, help="Macro ID")
    exp_p.add_argument("file", help="Destination file path")

    # macro import
    imp_p = macro_sub.add_parser("import", help="Import a macro from JSON")
    imp_p.add_argument("file", help="Source JSON file path")

    # macro delete
    del_p = macro_sub.add_parser("delete", help="Delete a macro")
    del_p.add_argument("id", type=int, help="Macro ID")

    # macro play
    play_p = macro_sub.add_parser("play", help="Execute / play back a macro into the active window")
    play_p.add_argument("id", type=int, help="Macro ID to play")
    play_p.add_argument("-r", "--repeat", type=int, default=None, help="Repeat count (default: from macro settings)")
    play_p.add_argument("-d", "--delay-start", type=float, default=2.0, help="Initial delay in seconds before playing (default: 2.0s)")

    args = parser.parse_args()

    if getattr(args, "help", False) or args.command == "help":
        topic = getattr(args, "topic", None)
        if topic:
            print_topic_help(topic)
        else:
            print_full_cli_help(driver)
        return

    if args.command in ("setup-udev", "setup_udev"):
        ok = run_setup_cli(force_prompt=True)
        sys.exit(0 if ok else 1)

    # Automatically check permissions before commands requiring device access
    if args.command in ("set", "music", "macro"):
        has_access, reason = check_access()
        if not has_access:
            print(f"\n[!] Access notice: {reason}")
            print("Requesting administrator privileges (sudo) to configure device permissions...")
            ok = run_setup_cli()
            if not ok:
                print("Error: Cannot proceed without device access permissions.")
                sys.exit(1)
            driver = FreeWolfK8Driver()

    if args.command == "macro":
        from .macro import MacroManager
        mgr = MacroManager()

        if args.macro_action == "list" or not args.macro_action:
            print(f"{'ID':>3} | {'Macro Name':<20} | {'Repeat':<8} | {'Actions':<8} | Delay Mode")
            print("-" * 65)
            mode_names = {0: "Record", 1: "None", 2: "Default"}
            for m in mgr.macros:
                d_mode = mode_names.get(m.delay_type, "Unknown")
                if m.delay_type == 2:
                    d_mode += f" ({m.default_delay}ms)"
                print(f"{m.id:3d} | {m.name:<20} | {m.repeat_time:<8} | {len(m.actions):<8} | {d_mode}")
            return

        elif args.macro_action == "show":
            macro = mgr.get_macro(args.id)
            if not macro:
                print(f"Error: Macro with ID {args.id} not found.")
                sys.exit(1)
            print(f"Macro ID   : {macro.id}")
            print(f"Name       : {macro.name}")
            print(f"Repeat Time: {macro.repeat_time}")
            print(f"Delay Mode : {macro.delay_type}")
            print(f"Default MS : {macro.default_delay}")
            print("-" * 50)
            print(f"{'#':>3} | {'Description':<18} | {'Action':<8} | {'Delay(ms)':<10} | Keycode")
            print("-" * 50)
            for idx, a in enumerate(macro.actions):
                print(f"{idx+1:3d} | {a.desc:<18} | {a.action:<8} | {a.delay_ms:<10} | 0x{a.keycode:02x}")
            return

        elif args.macro_action == "export":
            macro = mgr.get_macro(args.id)
            if not macro:
                print(f"Error: Macro with ID {args.id} not found.")
                sys.exit(1)
            mgr.export_macro(args.id, args.file)
            print(f"Exported '{macro.name}' to {args.file}")
            return

        elif args.macro_action == "import":
            try:
                m = mgr.import_macro(args.file)
                print(f"Imported '{m.name}' with ID {m.id} ({len(m.actions)} actions).")
            except Exception as e:
                print(f"Failed to import macro: {e}")
                sys.exit(1)
            return

        elif args.macro_action == "delete":
            macro = mgr.get_macro(args.id)
            if not macro:
                print(f"Error: Macro with ID {args.id} not found.")
                sys.exit(1)
            mgr.delete_macro(args.id)
            print(f"Deleted macro ID {args.id} ('{macro.name}').")
            return

        elif args.macro_action == "play":
            macro = mgr.get_macro(args.id)
            if not macro:
                print(f"Error: Macro with ID {args.id} not found.")
                sys.exit(1)
            if not macro.actions:
                print(f"Error: Macro '{macro.name}' has no actions to play.")
                sys.exit(1)

            from .macro import MacroPlayer
            player = MacroPlayer()
            if not player.is_available():
                print(f"Error: Cannot play macro: {player._init_error}")
                print("Tip: Run 'k8ctl setup-udev' to grant permissions to /dev/uinput.")
                sys.exit(1)

            repeats = args.repeat if args.repeat is not None else max(1, macro.repeat_time)
            print(f"Preparing to play '{macro.name}' ({len(macro.actions)} actions, repeat: {repeats})...")
            if args.delay_start > 0:
                print(f"Switch to target window now! Starting in {args.delay_start:g}s...")
                for sec in range(int(args.delay_start), 0, -1):
                    print(f"  {sec}...")
                    time.sleep(1.0)
                rem = args.delay_start - int(args.delay_start)
                if rem > 0:
                    time.sleep(rem)

            print("▶ Playing macro keystrokes...")
            def on_step(rep, act_idx, act):
                print(f"  [{rep}/{repeats}] Action {act_idx}: {act.desc} ({act.action}) - delay {act.delay_ms}ms")

            ok, msg = player.play(macro, repeat_count=repeats, on_step=on_step)
            print(f"Result: {msg}")
            return

    if args.command == "status":
        print_status(driver)
        return

    if args.command == "list":
        list_modes()
        return

    if args.command == "udev":
        print('KERNEL=="hidraw*", ATTRS{idVendor}=="1a2c", ATTRS{idProduct}=="7c80", MODE="0666", TAG+="uaccess"')
        print('KERNEL=="hidraw*", ATTRS{idVendor}=="1a2c", ATTRS{idProduct}=="7fff", MODE="0666", TAG+="uaccess"')
        return

    if args.command == "set":
        try:
            mode = find_mode(args.mode)
        except ValueError as e:
            print(f"Error: {e}")
            sys.exit(1)

        state, detail = driver.probe()
        if state != DeviceState.CONNECTED:
            print(f"Cannot set lighting: {detail}")
            sys.exit(1)

        if not driver.open():
            print(f"Failed to open device: {driver.status_detail}")
            sys.exit(1)

        ok = driver.set_lighting(mode, brightness=args.brightness, speed=args.speed)
        driver.close()
        if ok:
            print(f"Successfully applied mode '{mode.name}' (Brightness={args.brightness}, Speed={args.speed})")
        else:
            print(f"Failed to send lighting command: {driver.status_detail}")
            sys.exit(1)

    elif args.command == "music":
        state, detail = driver.probe()
        if state != DeviceState.CONNECTED:
            print(f"Cannot start music visualizer: {detail}")
            sys.exit(1)

        if not driver.open():
            print(f"Failed to open device: {driver.status_detail}")
            sys.exit(1)

        print(f"Starting Music Mode stream (SubMode={args.submode}, Rate={args.delay}ms / {1000//args.delay}Hz)...")
        print("Press Ctrl+C to stop.")
        engine = MusicVisualizerEngine(driver)
        engine.start(submode=args.submode, delay_ms=args.delay)
        try:
            while True:
                time.sleep(0.5)
        except KeyboardInterrupt:
            print("\nStopping visualizer...")
            engine.stop()
            driver.close()
            print("Visualizer stopped.")

if __name__ == "__main__":
    main()
