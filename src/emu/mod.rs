mod cpu;
pub mod gpu;
mod input;
mod iset;
mod mem;
mod timer;

use crate::emojis::EMOJIS as E;
use cpu::Cpu;
use gpu::Gpu;
use input::Keypad;
use mem::Memory;
use timer::Timer; // Avoid Emoji Nightmares

use color_eyre::{eyre::bail, Report, Result};
use std::time::{self, Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use tui_logger::{LevelFilter, TuiWidgetState};

use std::sync::mpsc;
use std::thread;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug)]
enum AppEvent {
    KeyEvent(KeyEvent),
    CounterChanged(Option<u16>),
}

//#[derive(Debug)]
pub struct Emulator {
    pub cpu: Cpu,
    pub gpu: Gpu,
    pub memory: Memory,
    pub should_quit: bool,
    pub timers: Timer,
    pub progress_counter: Option<u16>,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            memory: Memory::new(),
            timers: Timer::new(1),
            should_quit: false,
            progress_counter: None,
        }
    }

    /// Renders the Gpu on the left
    /// Renders the Logs on the right
    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        self.gpu.render(chunks[0], frame.buffer_mut());

        let log_block = Block::bordered().title("Log Output");
        let log_content = Paragraph::new("Placeholder for log output");
        log_content
            .block(log_block)
            .render(chunks[1], frame.buffer_mut());
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), String> {
        self.memory.load_rom(rom_data);
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<u8> {
        match key_event.code {
            KeyCode::Char('0') => {
                //TODO: figure out what i was doing here self.exit();
                return Ok(255);
            }
            KeyCode::Left => {
                self.decrement_counter()?;
                return Ok(254);
            }
            KeyCode::Right => {
                self.increment_counter()?;
                return Ok(253);
            }

            // Chip8 valid 16 chars
            KeyCode::Char('1') => Ok::<u8, Report>(0),
            KeyCode::Char('2') => Ok(1),
            KeyCode::Char('3') => Ok(2),
            KeyCode::Char('4') => Ok(3),
            KeyCode::Char('q') => {
                self.should_quit = true;
                Ok(4)
            }
            KeyCode::Char('w') => Ok(5),
            KeyCode::Char('e') => Ok(6),
            KeyCode::Char('r') => Ok(7),
            KeyCode::Char('a') => Ok(8),
            KeyCode::Char('s') => Ok(9),
            KeyCode::Char('d') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.should_quit = true;
                }
                Ok(10)
            }
            KeyCode::Char('f') => Ok(11),
            KeyCode::Char('z') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.should_quit = true;
                }
                Ok(12)
            }
            KeyCode::Char('x') => Ok(13),
            KeyCode::Char('c') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.should_quit = true;
                }
                Ok(14)
            }
            KeyCode::Char('v') => Ok(15),
            _ => Ok(222),
        }
        //Ok(111)
    }

    /// The print_memory function has been moved to the Memory module
    pub fn print_memory(&self) {
        for (i, byte) in self.memory.ram.iter().enumerate() {
            if i % 16 == 0 {
                println!("\n{:04X}: ", i);
            }
            print!("{:02X} ", byte);
        }
        println!();
    }

    // galus: There is an overflow bug here left for educational porpoises 🎓 🐬
    fn increment_counter(&mut self) -> Result<()> {
        match self.progress_counter {
            Some(value) => {
                if value >= 2 {
                    bail!("counter overflow");
                }
                self.progress_counter = Some(value + 1);
            }
            None => self.progress_counter = Some(1),
        }
        Ok(())
    }

    fn decrement_counter(&mut self) -> Result<()> {
        match self.progress_counter {
            Some(value) => {
                if value > 0 {
                    self.progress_counter = Some(value - 1);
                }
            }
            None => {
                // nothing
            }
        }
        Ok(())
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        info!("\t{} Running Emulator...", E["computer"]);
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        let progress_tx = tx.clone();
        println!("spawning io thread");
        thread::spawn(move || io_thread(event_tx));

        // for testing right now...
        println!("spawning progress bar thread");
        thread::spawn(move || progress_task(progress_tx));

        println!("spawning other background tasks");
        thread::spawn(move || background_task());
        thread::spawn(move || background_task2());

        while !self.should_quit {
            self.timers.tick();
            let _ = self
                .cpu
                .tick(&mut self.memory, &mut self.gpu, &mut self.timers);

            match rx.recv() {
                Ok(AppEvent::KeyEvent(key_event)) => {
                    // if its q, quit
                    info!("rx.recv got KeyCode {:?}", key_event.code);
                    self.handle_key_event(key_event);
                }
                Ok(AppEvent::CounterChanged(x)) => {
                    info!("counter changed {:?}", x);
                    self.progress_counter = x;
                }
                Err(_) => {
                    error!("Core thread Sender disconnected. Exitting.");
                    break;
                }
            }

            terminal.draw(|frame| {
                self.draw(frame);
            });
        }

        Ok(())
    }
} // end Impl Emulator

// -------------------------------------------
// Threads
// Separate threads for background tasks.
// -------------------------------------------

/// Responsible for handling user input.
fn io_thread(tx: mpsc::Sender<AppEvent>) -> Result<()> {
    loop {
        let event = match event::read() {
            Ok(event) => event,
            Err(e) => {
                eprintln!("Error reading event: {}", e);
                break;
            }
        };
        if let Event::Key(key_event) = event {
            if key_event.kind == KeyEventKind::Press {
                if tx.send(AppEvent::KeyEvent(key_event)).is_err() {
                    break;
                }
            }
        }
    }
    Ok(())
}

/// Sends AppEvent::CounterChanged events at constant intervals
fn progress_task(tx: mpsc::Sender<AppEvent>) -> Result<()> {
    info!(target: "progress-task", "Starting progress task...");
    for progress in 0..100 {
        tx.send(AppEvent::CounterChanged(Some(progress)))?;
        thread::sleep(Duration::from_millis(3000));
    }
    info!(target: "progress-task", "Progress task finished!");
    tx.send(AppEvent::CounterChanged(None))?;
    Ok(())
}

/// Spams a bunch of logs every second
fn background_task() {
    loop {
        error!(target: "background-task", "an error");
        warn!(target: "background-task", "an warning");
        info!(target: "background-task", "an two line info\nsecond line");
        debug!(target: "background-task", "an debug");
        trace!(target: "background-task", "an trace");
        error!(target: "background-task", "an error");
        thread::sleep(Duration::from_millis(1000));
    }
}

/// Spams a long log message every second
fn background_task2() {
    loop {
        info!(target: "background-task2", "This is a very long message, blah di blah di blah, lets wrap this up with some screen size magic.");
        thread::sleep(Duration::from_millis(1000));
    }
}

/// Spams hearts <3 every 1.5 seconds
fn heart_task() {
    let mut line = "<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3<3".to_string();
    loop {
        info!(target: "heart-task", "{}", line);
        line = format!(".{}", line);
        thread::sleep(time::Duration::from_millis(1500));
    }
}
