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

use emojis::EMOJIS as E; // Avoid Emoji Nightmares
use emu::Emulator;
use std::env::args;
use std::env::temp_dir;
use std::fs::read;
use tui_logger::{
    init_logger, set_default_level, set_log_file, TuiLoggerFile, TuiLoggerLevelOutput,
};

#[macro_use]
extern crate log;

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

fn main() -> Result<()> {
    color_eyre::install()?; // error hooks
    setup_logging()?;

    println!("{} Initializing emulator", E["dynamite"]);
    let mut emu: Emulator = Emulator::new();

    println!("\t{} Loading fonts into emulator...", E["pen"]);
    let _ = emu.load_font();

    let rom_path: String = args()
        .nth(1)
        .ok_or(eyre!("Please provide a path to a ROM file"))?;

    println!("\t{} Reading rom {}...", E["eye"], rom_path);
    let rom_data = read(rom_path)?;
    emu.cpu.memory.rom = rom_data;

    println!("\t{} Loading rom into emulator...", E["joystick"]);
    let _ = emu.load_rom(); // clears emu.rom_buffer

    println!("\t{} Initializing terminal...", E["computer"]);
    let mut terminal = emu.cpu.memory.gpu.init()?;

    println!("\t{} Running app...", E["runner"]);

    loop {
        let _ = emu.cpu.fetch_opcode();
        if let Err(err) = emu.cpu.process() {
            eprintln!("failed to process.: {}", err);
            break;
        }

        emu.cpu.memory.gpu.run(&mut terminal)?;

        // Trying to figure out how to have above return a fn ptr
        // display
        // inpup

        // add a conditioal break
        break;
    }

    if let Err(err) = emu.cpu.memory.gpu.restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }

    println!("{} Exiting...", E["handwave"]);
    Ok(())
}
