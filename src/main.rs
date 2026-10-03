//! FREE WOLF K8 Unified Linux Binary (Dual GUI & CLI)

mod protocol;
mod driver;
mod config;
mod macro_mgr;
mod music;
mod udev;
mod i18n;
mod manual;
mod cli;
mod gui;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_name = args.first().and_then(|p| std::path::Path::new(p).file_name()).and_then(|n| n.to_str()).unwrap_or("");

    // 1. Invocation via 'k8ctl' symlink or alias -> CLI mode
    if prog_name == "k8ctl" {
        cli::run_cli(&args[1..]);
        return;
    }

    // 2. Invocation via 'k8gui' symlink or alias -> GUI mode
    if prog_name == "k8gui" {
        gui::run_gui();
        return;
    }

    // 3. No arguments -> Default to Modern GUI
    if args.len() <= 1 {
        gui::run_gui();
        return;
    }

    // 4. Explicit --gui / -g flag -> GUI mode
    if args[1] == "--gui" || args[1] == "-g" {
        gui::run_gui();
        return;
    }

    // 5. Explicit -c / --cli flag -> CLI mode
    if args[1] == "-c" || args[1] == "--cli" {
        cli::run_cli(&args[2..]);
        return;
    }

    // 6. Known CLI subcommands -> Auto-detect CLI mode directly!
    let known_subcommands = [
        "status", "setup-udev", "setup_udev", "list", "modes",
        "set", "music", "macro", "help", "udev", "-h", "--help",
    ];

    if known_subcommands.contains(&args[1].as_str()) {
        cli::run_cli(&args[1..]);
        return;
    }

    // Fallback: if unknown flag, pass to CLI help
    cli::run_cli(&args[1..]);
}
