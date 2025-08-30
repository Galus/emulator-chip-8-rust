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

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use tui_logger::{LevelFilter, TuiWidgetState};

use std::sync::mpsc;
use std::thread;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

//#[derive(Debug)]
pub struct Emulator {
    pub cpu: Cpu,
    pub gpu: Gpu,
    pub keypad: Keypad,
    pub memory: Memory,
    pub should_quit: bool,
    pub timers: Timer,
    // fields for tui state management
    pub states: Vec<TuiWidgetState>,
    pub tab_names: Vec<&'static str>,
    pub selected_tab: usize,
    pub progress_counter: Option<u16>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppMode {
    #[default]
    Run,
    Quit,
}

#[derive(Debug)]
enum AppEvent {
    UiEvent(Event),
    CounterChanged(Option<u16>),
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            memory: Memory::new(),
            keypad: Keypad::new(),
            timers: Timer::new(99999),
            should_quit: false,
            // init tui state
            states: vec![
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
                TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
            ],
            tab_names: vec!["State 1", "State 2", "State 3", "State 4"],
            selected_tab: 0,
            progress_counter: None,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        info!("\t{} Running Emulator...", E["computer"]);
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        let progress_tx = tx.clone();
        thread::spawn(move || input_thread(event_tx));

        // for testing right now...
        thread::spawn(move || progress_task(progress_tx));

        loop {
            self.timers.tick();
            self.cpu
                .tick(&mut self.memory, &mut self.gpu, &mut self.timers);

            match rx.recv() {
                Ok(AppEvent::UiEvent(event)) => {
                    // if its q, quit
                    info!("rx.recv got {:?}", event);
                    break;
                }
                Ok(AppEvent::CounterChanged(x)) => {
                    info!("counter changed {:?}", x);
                }
                Err(_) => {
                    error!("Core thread Sender disconnected. Exitting.");
                    break;
                }
            }

            //if let Ok(event) = rx.try_recv() {
            //    match event {
            //        AppEvent::UiEvent(e) => {
            //            self.handle_ui_event(e);
            //            if self.should_quit {
            //                return Ok(());
            //            }
            //        }
            //        AppEvent::CounterChanged(value) => {
            //            self.update_progress_bar(value);
            //        }
            //    }
            //}
            //Err(err) = self.restoreTerminal() {
            //    warn!(
            //        "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            //        err
            //    );
            //}
            terminal.draw(|frame| {
                self.draw(frame);
            });
        }
        Ok(())
    }

    // TODO: What is a Frame?
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
            KeyCode::Char('q') => Ok(4),
            KeyCode::Char('w') => Ok(5),
            KeyCode::Char('e') => Ok(6),
            KeyCode::Char('r') => Ok(7),
            KeyCode::Char('a') => Ok(8),
            KeyCode::Char('s') => Ok(9),
            KeyCode::Char('d') => Ok(10),
            KeyCode::Char('f') => Ok(11),
            KeyCode::Char('z') => Ok(12),
            KeyCode::Char('x') => Ok(13),
            KeyCode::Char('c') => Ok(14),
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
} // end Impl Emulator

// -------------------------------------------
// Threads
// Separate threads for background tasks.
// -------------------------------------------

/// Responsible for handling user input
fn input_thread(tx: mpsc::Sender<AppEvent>) -> Result<()> {
    loop {
        match event::read() {
            Ok(event) => {
                if let Event::Key(key) = event {
                    if key.kind == KeyEventKind::Press {
                        if tx.send(AppEvent::UiEvent(event)).is_err() {
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading event: {}", e);
                break;
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

//--------------------------------------------------------------
// App
// Core App Logic
//--------------------------------------------------------------

//#[derive(Debug)]
//struct App {
//    mode: AppMode,
//    states: Vec<TuiWidgetState>,
//    tab_names: Vec<&'static str>,
//    selected_tab: usize,
//    progress_counter: Option<u16>,
//    gpu: Gpu,
//}

//impl App {
//    pub fn new() -> App {
//        let states = vec![
//            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
//            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
//            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
//            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
//        ];
//
//        let tab_names = vec!["State 1", "State 2", "State 3", "State 4"];
//        App {
//            mode: AppMode::Run,
//            states,
//            tab_names,
//            selected_tab: 0,
//            progress_counter: None,
//            gpu: Gpu::new(),
//        }
//    }
//
//    //fn run(
//    //    &mut self,
//    //    terminal: &mut DefaultTerminal,
//    //    rx: mpsc::Receiver<AppEvent>,
//    //) -> color_eyre::Result<()> {
//    //    for event in rx {
//    //        match event {
//    //            AppEvent::UiEvent(event) => self.handle_ui_event(event),
//    //            AppEvent::CounterChanged(value) => self.update_progress_bar(event, value),
//    //        }
//    //        if self.mode == AppMode::Quit {
//    //            break;
//    //        }
//    //        self.draw(terminal)?;
//    //    }
//    //    Ok(())
//    //}
//
//    fn update_progress_bar(&mut self, event: AppEvent, value: Option<u16>) {
//        trace!(target: "App", "Updating progress bar {:?}", event);
//        self.progress_counter = value;
//        if value.is_none() {
//            info!(target: "App", "Background task finished");
//        }
//    }
//
//    fn next_tab(&mut self) {
//        self.selected_tab = (self.selected_tab + 1) % self.tab_names.len();
//    }
//
//    fn handle_ui_event(&mut self, event: Event) {
//        debug!(target: "App", "Handling UI event {:?}", event);
//        let state = self.selected_state();
//
//        if let Event::Key(key) = event {
//            let code = key.code;
//
//            match code.into() {
//                KeyCode::Char('q') => self.mode = AppMode::Quit,
//                KeyCode::Char('\t') => self.next_tab(),
//                KeyCode::Tab => self.next_tab(),
//                KeyCode::Char(' ') => state.transition(TuiWidgetEvent::SpaceKey),
//                KeyCode::Esc => state.transition(TuiWidgetEvent::EscapeKey),
//                KeyCode::PageUp => state.transition(TuiWidgetEvent::PrevPageKey),
//                KeyCode::PageDown => state.transition(TuiWidgetEvent::NextPageKey),
//                KeyCode::Up => state.transition(TuiWidgetEvent::UpKey),
//                KeyCode::Down => state.transition(TuiWidgetEvent::DownKey),
//                KeyCode::Left => state.transition(TuiWidgetEvent::LeftKey),
//                KeyCode::Right => state.transition(TuiWidgetEvent::RightKey),
//                KeyCode::Char('+') => state.transition(TuiWidgetEvent::PlusKey),
//                KeyCode::Char('-') => state.transition(TuiWidgetEvent::MinusKey),
//                KeyCode::Char('h') => state.transition(TuiWidgetEvent::HideKey),
//                KeyCode::Char('f') => state.transition(TuiWidgetEvent::FocusKey),
//                _ => (),
//            }
//        }
//    }
//
//    fn selected_state(&mut self) -> &mut TuiWidgetState {
//        &mut self.states[self.selected_tab]
//    }
//
//    fn new_tab(&mut self) {
//        self.selected_tab = (self.selected_tab + 1) & self.tab_names.len();
//    }
//} // End impl App

//--------------------------------------------------------------
// App
// The V in MVC
//--------------------------------------------------------------
//impl Widget for &App {
//    fn render(self, area: Rect, buf: &mut Buffer) {
//        // manage layout here
//        let chunks = Layout::default()
//            .direction(Direction::Horizontal)
//            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
//            .split(area);
//
//        // renders the GPU widget in the left chunk
//        self.gpu.render(chunks[0], buf);
//
//        // renders the logger on the right
//        let log_block = Block::bordered().title("Log Output");
//        let log_content = Paragraph::new("Placeholder for log output");
//        log_content.block(log_block).render(chunks[1], buf);
//    }
//}
