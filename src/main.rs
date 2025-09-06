#![allow(dead_code)]
#![allow(unused_variables)]
// Copyright (c) 2024-2025 galus. All Rights Reserved.
//    __                        _                                __
//   / /_/\__        __ _  __ _| |_   _ ___             __/\__  / /
//  / /\    /       / _` |/ _` | | | | / __|            \    / / /
// / / /_  _\      | (_| | (_| | | |_| \__ \            /_  _\/ /
///_/    \/         \__, |\__,_|_|\__,_|___/              \/ /_/
//                  |___/

#[macro_use]
extern crate log;
use log::{debug, info};

use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::DefaultTerminal;

use std::env::{args, current_dir};

use tui_logger::{
    init_logger, set_default_level, set_log_file, ExtLogRecord, LogFormatter, TuiLoggerFile,
    TuiLoggerLevelOutput,
};

use color_eyre::{eyre::eyre, Result};

mod emojis;
mod emu;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use emu::Emulator;

use emojis::EMOJIS as E;
use std::fs::read;
use std::io::stdout;

fn setup_logging() -> Result<()> {
    init_logger(log::LevelFilter::Trace)?;
    set_default_level(log::LevelFilter::Trace);

    let mut dir = current_dir()?;
    dir.push("chip8.log");
    let dir_str = dir.to_str().ok_or(eyre!("Failed to get log file path"))?;
    println!("log dir {}", dir_str);
    let file_options = TuiLoggerFile::new(dir_str)
        .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
        .output_file(true)
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

    #[allow(unused_mut)]
    let mut terminal = ratatui::init(); // ratatui.rs has 'let mut terminal'
    Ok(terminal)
}

/// Restore the terminal to its original state
fn restore_terminal() -> Result<()> {
    trace!(target:"tui", "Restoring terminal");
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    ratatui::restore();
    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        hook(panic_info);
    }))
}

//--------------------------------------------------------------
// Logging
// Ugly LogFormatter
//--------------------------------------------------------------

struct MyLogFormatter {}
impl LogFormatter for MyLogFormatter {
    fn min_width(&self) -> u16 {
        4
    }
    fn format(&self, _width: usize, evt: &ExtLogRecord) -> Vec<Line> {
        let mut lines = vec![];
        match evt.level {
            log::Level::Error => {
                let st = Style::new(); //.red().bold();
                let sp = Span::styled("======", st);
                let mayday = Span::from(" MAYDAY MAYDAY ".to_string());
                let sp2 = Span::styled("======", st);
                lines.push(Line::from(vec![sp, mayday, sp2]).alignment(Alignment::Center));
                lines.push(
                    Line::from(format!("{}: {}", evt.level, evt.msg()))
                        .alignment(Alignment::Center),
                );
            }
            _ => {
                lines.push(Line::from(format!("{}: {}", evt.level, evt.msg())));
            }
        };

        match evt.level {
            log::Level::Error => {
                let st = Style::new(); //.blue().bold();
                let sp = Span::styled("======", st);
                let mayday = Span::from(" MAYDAY SEEN ? ".to_string());
                let sp2 = Span::styled("======", st);
                lines.push(Line::from(vec![sp, mayday, sp2]).alignment(Alignment::Center));
            }
            _ => {}
        };
        lines
    }
}

fn main() -> Result<()> {
    color_eyre::install()?; // error hooks
    setup_logging()?;

    info!("{} Initializing emulator", E["dynamite"]);
    let mut emu: Emulator = Emulator::new();

    info!("\t{} Loading fonts into emulator...", E["pen"]);
    let _ = emu.memory.load_font();

    let rom_path: String = args()
        .nth(1)
        .ok_or(eyre!("Please provide a path to a ROM file"))?;

    info!("\t{} Reading rom {}...", E["eye"], rom_path);
    let rom_data = read(&rom_path).expect(&format!("Could not read ROM file from: {}", rom_path));

    info!("\t{} Loading rom into emulator...", E["joystick"]);
    let _ = emu.load_rom(&rom_data);

    info!("\t{} Running app...", E["runner"]);
    let mut terminal = init_terminal().unwrap();
    let _ = emu.run(&mut terminal);
    let _ = terminal.clear();

    info!("{} Exiting...", E["handwave"]);
    let _ = restore_terminal();

    // dont forget to flush ;)
    log::logger().flush();

    Ok(())
}
