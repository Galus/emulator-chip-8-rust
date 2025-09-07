// use ratatui::layout::Position;
// use ratatui::text::Text;
use ratatui::widgets::{BorderType, Paragraph};
use ratatui::{layout::Alignment, style::Stylize};
mod cpu;
pub mod gpu;
mod input;
mod iset;
mod mem;
mod timer;

use crate::emojis::EMOJIS as E;
use cpu::Cpu;
use gpu::Gpu;
use mem::Memory;
use timer::Timer; // Avoid Emoji Nightmares

use color_eyre::{
    eyre::bail,
    // Report,
    Result,
};
use std::time::{self, Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use tui_logger::{
    LevelFilter, TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetEvent, TuiWidgetState,
};

use std::sync::mpsc;
use std::thread;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    // text::Line,
    widgets::{
        // block::Title,
        Block,
        Widget,
    },
    DefaultTerminal,
    Frame,
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
    pub show_help: bool,
    pub show_logs: bool,
    pub timers: Timer,
    pub progress_counter: Option<u16>,
    pub states: Vec<TuiWidgetState>,
    pub tab_names: Vec<&'static str>,
    pub selected_tab: usize,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            memory: Memory::new(),
            timers: Timer::new(1),
            should_quit: false,
            show_help: false,
            show_logs: true,
            progress_counter: None,
            states: vec![
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
            ],
            tab_names: vec!["State 1", "State 2", "State 3", "State 4"],
            selected_tab: 0,
        }
    }

    /// Renders the Gpu on the left
    /// Renders the Logs on the right
    fn draw(&self, frame: &mut Frame) {
        if !self.show_help {
            if self.show_logs {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(frame.area());

                self.gpu.render(chunks[0], frame.buffer_mut());

                //let log_block = Block::bordered().title("Log Output");
                //frame.render_widget(log_block, chunks[1]);

                //let log_content = log_block.inner(chunks[1]);

                let current_state = self.selected_state();
                TuiLoggerSmartWidget::default()
                    .style_error(Style::default().fg(Color::Red))
                    .style_debug(Style::default().fg(Color::Green))
                    .style_warn(Style::default().fg(Color::Yellow))
                    .style_trace(Style::default().fg(Color::Magenta))
                    .style_info(Style::default().fg(Color::Cyan))
                    .output_separator(':')
                    .output_timestamp(Some("%H:%M:%S".to_string()))
                    .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
                    .output_target(true)
                    .output_file(true)
                    .output_line(true)
                    .state(current_state)
                    .render(chunks[1], frame.buffer_mut());
            } else {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100)])
                    .split(frame.area());

                self.gpu.render(chunks[0], frame.buffer_mut());
            }
        } else {
            let title = vec![" Help".bold(), " ?".red().bold()];
            let instructions = vec![" Close Help ".into(), "Press any key.".blue().bold()];
            let help_text = "\
  Help

  This is an interactive terminal application.
  Use the following keybindings to control the emulator and interact with the interface.

  General Controls
  - ?: Toggle the help screen.
  - q or Ctrl-C: Quit the application.

  Chip-8 Keypad Mapping
  The emulator maps your keyboard to a standard Chip-8 keypad.
  - 1: 1
  - 2: 2
  - 3: 3
  - 4: C
  - q: 4
  - w: 5
  - e: 6
  - r: D
  - a: 7
  - s: 8
  - d: 9
  - f: E
  - z: A
  - x: 0
  - c: B
  - v: F

  Log Panel Controls
  These controls are active when the log panel is focused.
  - l: Toggle Log Panel on/off
  - Arrow Keys (↑, ↓, ←, →): Navigate through log messages.
  - Page Up / Page Down: Jump to the previous or next page of logs.
  - + / -: Increase or decrease the log verbosity level.
  - h: Hide the log target selector.
  - f: Focus on the log target selector.
  - Tab: Switch between the different log states.
  - Escape: Exit the log focus mode.
  ";

            let block = Block::bordered()
                .title_top(title)
                .title_bottom(instructions)
                .border_type(BorderType::Rounded);

            let paragraph = Paragraph::new(help_text)
                .alignment(Alignment::Left)
                .block(block);

            frame.render_widget(paragraph, frame.area());
        }
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) -> Result<(), String> {
        // TODO: Looks like the rom is loading fine, tracking repeating OpCode 00e0 ticks.
        // I think its because our program_counter is just not incrementing? or state
        // is being reset back to 0x200 on each loop iteration
        info!(target: "emu", "load_rom before mem: {:x?}", self.memory);
        self.memory.load_rom(rom_data);
        info!(target: "emu", "load_rom after mem: {:x?}", self.memory);
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<(), String> {
        let state = self.selected_state();
        // If help window is showing, pressing any key removes it.
        if self.show_help {
            self.show_help = false;
            return Ok(());
        }

        match key_event.code {
            KeyCode::Char('?') => {
                self.show_help = !self.show_help;
                Ok(())
            }
            // Chip8 valid 16 chars
            KeyCode::Char('0') => Ok(()),
            KeyCode::Char('1') => Ok(()),
            KeyCode::Char('2') => Ok(()),
            KeyCode::Char('3') => Ok(()),
            KeyCode::Char('4') => Ok(()),
            KeyCode::Char('q') => {
                self.should_quit = true;
                Ok(())
            }
            KeyCode::Char('w') => Ok(()),
            KeyCode::Char('e') => Ok(()),
            KeyCode::Char('r') => Ok(()),
            KeyCode::Char('a') => Ok(()),
            KeyCode::Char('s') => Ok(()),
            KeyCode::Char('d') => Ok(()),
            //KeyCode::Char('f') => Ok(()),
            KeyCode::Char('z') => Ok(()),
            KeyCode::Char('x') => Ok(()),
            KeyCode::Char('c') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.should_quit = true;
                }
                Ok(())
            }
            KeyCode::Char('v') => Ok(()),

            // Tui Logger Smart Widget Keys
            KeyCode::Char('l') => {
                self.show_logs = !self.show_logs;
                Ok(())
            }
            KeyCode::Char('\t') | KeyCode::Tab => {
                self.next_tab();
                Ok(())
            }
            KeyCode::Char(' ') => {
                state.transition(TuiWidgetEvent::SpaceKey);
                Ok(())
            }
            KeyCode::Esc => {
                state.transition(TuiWidgetEvent::EscapeKey);
                Ok(())
            }
            KeyCode::PageUp => {
                state.transition(TuiWidgetEvent::PrevPageKey);
                Ok(())
            }
            KeyCode::PageDown => {
                state.transition(TuiWidgetEvent::NextPageKey);
                Ok(())
            }
            KeyCode::Up => {
                state.transition(TuiWidgetEvent::UpKey);
                Ok(())
            }
            KeyCode::Down => {
                state.transition(TuiWidgetEvent::DownKey);
                Ok(())
            }
            KeyCode::Left => {
                state.transition(TuiWidgetEvent::LeftKey);
                Ok(())
            }
            KeyCode::Right => {
                state.transition(TuiWidgetEvent::RightKey);
                Ok(())
            }
            KeyCode::Char('+') => {
                state.transition(TuiWidgetEvent::PlusKey);
                Ok(())
            }
            KeyCode::Char('-') => {
                state.transition(TuiWidgetEvent::MinusKey);
                Ok(())
            }
            KeyCode::Char('h') => {
                state.transition(TuiWidgetEvent::HideKey);
                Ok(())
            }
            KeyCode::Char('f') => {
                state.transition(TuiWidgetEvent::FocusKey);
                Ok(())
            }

            // Catch the combination of Ctrl and any key.
            _ if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
                Ok(())
            }

            _ => Ok(()),
        }
    }

    fn selected_state(&self) -> &TuiWidgetState {
        &self.states[self.selected_tab]
    }

    fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % self.tab_names.len();
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
}

impl Default for Emulator {
    fn default() -> Self {
        Self::new()
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
