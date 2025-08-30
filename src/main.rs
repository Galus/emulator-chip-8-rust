// Copyright (c) 2024 galus. All Rights Reserved.
//    __                        _                                __
//   / /_/\__        __ _  __ _| |_   _ ___             __/\__  / /
//  / /\    /       / _` |/ _` | | | | / __|            \    / / /
// / / /_  _\      | (_| | (_| | | |_| \__ \            /_  _\/ /
///_/    \/         \__, |\__,_|_|\__,_|___/              \/ /_/
//                  |___/
use color_eyre::{eyre::eyre, Result};

mod emojis;
mod emu;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use emu::Emulator;

use emojis::EMOJIS as E;
use ratatui::DefaultTerminal; // Avoid Emoji Nightmares
use std::env::args;
use std::env::temp_dir;
use std::fs::read;
use std::io::stdout;
use tui_logger::{
    init_logger, set_default_level, set_log_file, TuiLoggerFile, TuiLoggerLevelOutput,
};

#[macro_use]
extern crate log;
use log::{debug, info};

fn setup_logging() -> Result<()> {
    init_logger(log::LevelFilter::Trace).unwrap();
    set_default_level(log::LevelFilter::Trace);

    let mut dir = temp_dir();
    dir.push("chip8.log");
    let dir_str = dir.to_str().ok_or(eyre!("Failed to get log file path"))?;
    let file_options = TuiLoggerFile::new(dir_str)
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_file(false)
        .output_separator(':');
    set_log_file(file_options);
    debug!(target:"App", "Logging to {}", dir_str);
    debug!(target:"App", "Logging initialized");

    Ok(())
}

// -------------------------------------------
// Utility
// Terminal Managment Functions
// -------------------------------------------

/// Initialize the terminal
fn init_terminal() -> Result<DefaultTerminal> {
    trace!(target:"tui", "Initializing terminal");
    enable_raw_mode()?; // takes input w/o w8n 4 newline, prevents keys being echo'd back
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    set_panic_hook();

    let mut terminal = ratatui::init(); // ratatui.rs has 'let mut terminal'
    Ok(terminal)
}

/// Restore the terminal to its original state
fn restore_terminal() -> Result<()> {
    trace!(target:"tui", "Restoring terminal");
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        hook(panic_info);
    }))
}

fn main() -> Result<()> {
    color_eyre::install()?; // error hooks
    setup_logging()?;

    info!("{} Initializing emulator", E["dynamite"]);
    let mut emu: Emulator = Emulator::new();

    info!("\t{} Loading fonts into emulator...", E["pen"]);
    let _ = emu.memory.load_font();
    return

    let rom_path: String = args()
        .nth(1)
        .ok_or(eyre!("Please provide a path to a ROM file"))?;

    info!("\t{} Reading rom {}...", E["eye"], rom_path);
    let rom_data = read(rom_path).expect(&format!("Could not read ROM file from: {}", rom_path));

    info!("\t{} Loading rom into emulator...", E["joystick"]);
    emu.load_rom(&rom_data);

    info!("\t{} Running app...", E["runner"]);
    let mut terminal = init_terminal().unwrap();
    emu.run(&mut terminal);

    info!("{} Exiting...", E["handwave"]);
    Ok(())
}
