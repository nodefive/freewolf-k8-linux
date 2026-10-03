use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crate::driver::{FreeWolfK8Driver, DeviceState};
use crate::macro_mgr::{MacroManager, UinputPlayer};
use crate::manual::get_topics;
use crate::music::MusicVisualizerEngine;
use crate::protocol::{LIGHT_MODES, find_mode};
use crate::udev::{run_setup_cli, UDEV_RULE_CONTENT};

static MUSIC_RUNNING: AtomicBool = AtomicBool::new(true);

extern "C" fn sigint_handler(_: libc::c_int) {
    MUSIC_RUNNING.store(false, Ordering::SeqCst);
}

pub fn print_status() {
    let probe = FreeWolfK8Driver::probe();
    println!("========================================");
    println!(" FREE WOLF K8 Linux Controller Status   ");
    println!("========================================");
    println!("Device State   : {}", probe.state.as_str());
    println!("Device Node    : {}", probe.node.unwrap_or_else(|| "None".to_string()));
    println!("Detail         : {}", probe.detail);
    println!("----------------------------------------");
    match probe.state {
        DeviceState::PermissionDenied => {
            println!("Tip: Run 'freewolf-k8 setup-udev' to configure permissions.");
        }
        DeviceState::ClaimedByVm => {
            println!("Tip: Keyboard is attached to QEMU/VM. Close the VM to control it natively.");
        }
        DeviceState::Connected => {
            println!("Status: Keyboard is ready for native Linux configuration.");
        }
        DeviceState::NotFound => {
            println!("Tip: Connect keyboard via USB cable or ensure 2.4G receiver is inserted.");
        }
    }
    println!("========================================");
}

pub fn list_modes() {
    println!("{:>2} | {:>4} | {:<22} | {:<10} | {:<8} | Description", "ID", "Wire", "Mode Name", "Brightness", "Speed");
    println!("{}", "-".repeat(80));
    for m in LIGHT_MODES {
        let br_str = if m.has_brightness() { "Yes (0-4)" } else { "No" };
        let sp_str = if m.is_music() {
            "Rate (ms)"
        } else if m.has_speed() {
            "Yes (0-4)"
        } else {
            "No"
        };
        println!("{:2} | 0x{:02x} | {:<22} | {:<10} | {:<8} | {}", m.id, m.wire_id, m.name, br_str, sp_str, m.description);
    }
}

pub fn run_cli(args: &[String]) {
    if args.is_empty() || args[0] == "-h" || args[0] == "--help" || args[0] == "-c" || args[0] == "--cli" {
        print_full_help();
        return;
    }

    match args[0].as_str() {
        "status" => print_status(),
        "setup-udev" | "setup_udev" => {
            run_setup_cli();
        }
        "list" | "modes" => list_modes(),
        "udev" => {
            println!("{}", UDEV_RULE_CONTENT.trim());
        }
        "set" => {
            let mut mode_str: Option<String> = None;
            let mut brightness = 4u8;
            let mut speed = 4u8;

            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "-m" | "--mode" => {
                        if i + 1 < args.len() {
                            mode_str = Some(args[i + 1].clone());
                            i += 2;
                            continue;
                        }
                    }
                    "-b" | "--brightness" => {
                        if i + 1 < args.len() {
                            brightness = args[i + 1].parse().unwrap_or(4).min(4);
                            i += 2;
                            continue;
                        }
                    }
                    "-s" | "--speed" => {
                        if i + 1 < args.len() {
                            speed = args[i + 1].parse().unwrap_or(4).min(4);
                            i += 2;
                            continue;
                        }
                    }
                    val if !val.starts_with('-') && mode_str.is_none() => {
                        mode_str = Some(val.to_string());
                    }
                    _ => {}
                }
                i += 1;
            }

            let mode_arg = match mode_str {
                Some(s) => s,
                None => {
                    eprintln!("Usage: freewolf-k8 set <mode_name_or_id> [-b <0-4>] [-s <0-4>]");
                    eprintln!("       freewolf-k8 set -m <mode> [-b <0-4>] [-s <0-4>]");
                    return;
                }
            };

            let mode = match find_mode(&mode_arg) {
                Some(m) => m,
                None => {
                    eprintln!("Error: Unknown lighting mode '{}'. Run 'freewolf-k8 list' to view valid modes.", mode_arg);
                    return;
                }
            };

            let probe = FreeWolfK8Driver::probe();
            if let Some(node) = probe.node {
                match FreeWolfK8Driver::set_lighting(&node, mode, brightness, speed) {
                    Ok(_) => println!("Successfully set lighting mode to '{}' (Brightness: {}, Speed: {})", mode.name, brightness, speed),
                    Err(e) => eprintln!("Failed to set lighting: {}", e),
                }
            } else {
                eprintln!("Error: FREE WOLF K8 keyboard is not accessible ({})", probe.detail);
            }
        }
        "music" => {
            let mut submode = 2u8;
            let mut delay = 66u64;

            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "-m" | "--submode" => {
                        if i + 1 < args.len() {
                            submode = args[i + 1].parse().unwrap_or(2).clamp(1, 2);
                            i += 2;
                            continue;
                        }
                    }
                    "-d" | "--delay" => {
                        if i + 1 < args.len() {
                            delay = args[i + 1].parse().unwrap_or(66).max(10);
                            i += 2;
                            continue;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }

            println!("Starting Music Mode Visualizer (Pattern: Mode {}, Frequency: {}ms)...", submode, delay);
            println!("Press Ctrl+C to stop.");

            MUSIC_RUNNING.store(true, Ordering::SeqCst);
            unsafe {
                libc::signal(libc::SIGINT, sigint_handler as *const () as libc::sighandler_t);
            }

            let mut engine = MusicVisualizerEngine::new();
            engine.start(submode, delay);

            while MUSIC_RUNNING.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(200));
            }
            engine.stop();
            println!("\nVisualizer stopped.");
        }
        "macro" => {
            handle_macro_command(&args[1..]);
        }
        "help" => {
            let topic_arg = args.get(1).map(|s| s.as_str()).unwrap_or("0");
            let topics = get_topics("en");
            let found = if let Ok(idx) = topic_arg.parse::<usize>() {
                topics.into_iter().find(|t| t.id == idx)
            } else {
                let q = topic_arg.to_lowercase();
                topics.into_iter().find(|t| t.title.to_lowercase().contains(&q))
            };

            if let Some(t) = found {
                println!("{}", t.content);
            } else {
                println!("Available Help Topics:");
                for t in get_topics("en") {
                    println!("  {:2} : {} {}", t.id, t.icon, t.title);
                }
                println!("\nUsage: freewolf-k8 help <topic_number_or_name>");
            }
        }
        unknown => {
            eprintln!("Unknown command: '{}'. Run 'freewolf-k8 --help' for usage.", unknown);
        }
    }
}

fn handle_macro_command(args: &[String]) {
    let mgr = MacroManager::load();
    if args.is_empty() || args[0] == "list" {
        println!("Saved Macros ({}):", mgr.macros.len());
        println!("--------------------------------------------------");
        for m in &mgr.macros {
            println!("  [{:>2}] {:<20} | Actions: {:>3} | Repeat: {:>2}", m.id, m.name, m.actions.len(), m.repeat_time);
        }
        return;
    }

    match args[0].as_str() {
        "show" => {
            if args.len() < 2 {
                eprintln!("Usage: freewolf-k8 macro show <id>");
                return;
            }
            let id: u32 = args[1].parse().unwrap_or(0);
            if let Some(m) = mgr.macros.iter().find(|m| m.id == id) {
                println!("Macro: '{}' (ID: {}, Repeat: {})", m.name, m.id, m.repeat_time);
                println!("{:<24} | {:<8} | Delay(ms)", "Action Description", "State");
                println!("{}", "-".repeat(50));
                for a in &m.actions {
                    println!("{:<24} | {:<8} | {}", a.desc, a.action, a.delay_ms);
                }
            } else {
                eprintln!("Macro with ID {} not found.", id);
            }
        }
        "play" => {
            if args.len() < 2 {
                eprintln!("Usage: freewolf-k8 macro play <id> [-d <countdown_secs>]");
                return;
            }
            let id: u32 = args[1].parse().unwrap_or(0);
            if let Some(m) = mgr.macros.iter().find(|m| m.id == id) {
                let player = match UinputPlayer::new() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Virtual keyboard unavailable: {}", e);
                        return;
                    }
                };

                println!("Switch to your target window! Playing macro '{}' in 2 seconds...", m.name);
                thread::sleep(Duration::from_secs(2));
                println!("Executing macro...");
                player.play(m);
                println!("Macro completed successfully.");
            } else {
                eprintln!("Macro with ID {} not found.", id);
            }
        }
        "delete" => {
            if args.len() < 2 {
                eprintln!("Usage: freewolf-k8 macro delete <id>");
                return;
            }
            let id: u32 = args[1].parse().unwrap_or(0);
            let mut mgr = mgr;
            if mgr.delete_macro(id) {
                println!("Macro {} deleted.", id);
            } else {
                eprintln!("Macro {} not found.", id);
            }
        }
        _ => {
            eprintln!("Unknown macro action. Options: list, show <id>, play <id>, delete <id>");
        }
    }
}


pub fn print_full_help() {
    let probe = FreeWolfK8Driver::probe();
    let status_str = match probe.state {
        DeviceState::Connected => format!("CONNECTED ({})", probe.node.unwrap_or_default()),
        DeviceState::PermissionDenied => format!("PERMISSION DENIED ({})", probe.node.unwrap_or_default()),
        DeviceState::ClaimedByVm => "CLAIMED BY VM".to_string(),
        DeviceState::NotFound => "DISCONNECTED".to_string(),
    };

    println!("================================================================================");
    println!(" FREE WOLF K8 Linux Controller (freewolf-k8)");
    println!(" Native Driver & CLI for FREE WOLF K8 Tri-Mode Mechanical Keyboard");
    println!("================================================================================");
    println!("Device State : {}", status_str);
    println!("--------------------------------------------------------------------------------");
    println!("USAGE:");
    println!("  freewolf-k8 [command] [arguments...] [options...]");
    println!("  (Running with no arguments launches the graphical user interface)\n");
    println!("COMMANDS:");
    println!("  status                     Display keyboard connection and hidraw info");
    println!("  setup-udev                 Install udev rules and configure permissions (sudo)");
    println!("  list                       List all 21 lighting modes with IDs & descriptions");
    println!("  set <mode> [options]       Set active RGB lighting mode by name or ID (0-20)");
    println!("                             Options:");
    println!("                               -b, --brightness <0-4>  Brightness level (default: 4)");
    println!("                               -s, --speed <0-4>       Animation speed (default: 4)");
    println!("  music [options]            Stream live real-time audio FFT visualizer to LEDs");
    println!("                             Options:");
    println!("                               -m, --submode <1|2>     Visualizer pattern (default: 2)");
    println!("                               -d, --delay <ms>        Update interval in ms (def: 66)");
    println!("  macro <action> [options]   Manage and play back macros via Linux /dev/uinput");
    println!("                             Actions: list, show <id>, play <id>, delete <id>");
    println!("  help [topic]               Display user manual & technical documentation");
    println!("  udev                       Print udev rules for unprivileged /dev/hidraw");
    println!("  --gui, -g                  Explicitly launch the graphical interface");
    println!("  --cli, -c                  Run in command-line mode\n");
    println!("HARDWARE SHORTCUTS (Fn):");
    println!("  Fn + \\|         Cycle through 19 dynamic RGB lighting effects");
    println!("  Fn + ↑ / ↓     Adjust brightness (0% / Off to 100%)");
    println!("  Fn + ← / →     Adjust animation speed (5 dynamic steps)");
    println!("  Fn + Backspace Toggle all backlighting On / Off");
    println!("  Fn + 1 / 2 / 3 Gaming presets: 1=FPS, 2=LOL/MOBA, 3=Office");
    println!("  Fn + A / S     Switch between Windows Mode (Fn+A) and macOS Mode (Fn+S)");
    println!("  Fn + Win       Toggle Windows Key Lock (Gaming Mode)");
    println!("  Fn + Q / W / E Switch Bluetooth profile 1 / 2 / 3 (Hold 3-5s to pair)");
    println!("  Fn + R         2.4 GHz wireless pairing mode (Hold 3-5s)");
    println!("================================================================================");
}
