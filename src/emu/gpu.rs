// I am thinking of making this a layer between the App and the Ratatui
// Contains the graphics processing.
use color_eyre::{
    eyre::{bail, WrapErr},
    Report, Result,
};
use std::time::Duration;

use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
};

use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use crossterm::execute;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::{border, Marker},
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        canvas::{Canvas, Rectangle},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

use tui_logger::{ExtLogRecord, LevelFilter, LogFormatter, TuiWidgetEvent, TuiWidgetState};

use std::io::{self, stdout};
use std::sync::mpsc;
use std::thread;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Gpu {
    pub counter: u8,
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

//--------------------------------------------------------------
// Gpu
// Part of the App.
// Struggling with if this name and purpose;
// is this the monitor, the GPU, or something else?
//--------------------------------------------------------------
impl Gpu {
    pub fn new() -> Self {
        Self {
            counter: 0,
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    //pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
    //    while !self.exit {
    //        // Render
    //        terminal.draw(|frame| self.render_frame(frame))?;
    //
    //        // Handle Input
    //        self.handle_events().wrap_err("handle events failed")?;
    //    }
    //    Ok(())
    //}

    //fn render_frame(&self, frame: &mut Frame) {
    //    frame.render_widget(self, frame.area());
    //}

    //pub fn handle_events(&mut self) -> Result<u8> {
    //    //color_eyre::install()?; // error hooks
    //    match event::read()? {
    //        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => self
    //            .handle_key_event(key_event)
    //            .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
    //        _ => Ok(255),
    //    }
    //}

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<u8> {
        match key_event.code {
            KeyCode::Char('0') => {
                self.exit();
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

    //fn exit(&mut self) {
    //    self.exit = true;
    //}

    // galus: There is an overflow bug here left for educational porpoises 🎓 🐬
    fn increment_counter(&mut self) -> Result<()> {
        self.counter += 1;
        if self.counter > 2 {
            bail!("counter overflow");
        }
        Ok(())
    }

    fn decrement_counter(&mut self) -> Result<()> {
        self.counter -= 1;
        Ok(())
    }

    fn content(&self) -> impl Widget + '_ {
        let mut screen = self.screen.clone();
        screen[1000..1099].copy_from_slice(&[true; 99]);

        let canvas = Canvas::default()
            .marker(Marker::Block)
            .block(Block::bordered().title("Canvas"))
            .x_bounds([0.0, SCREEN_WIDTH as f64])
            .y_bounds([0.0, SCREEN_HEIGHT as f64])
            .paint(move |ctx| {
                for y in 0..SCREEN_HEIGHT {
                    for x in 0..SCREEN_WIDTH {
                        let index = y * SCREEN_WIDTH + x;
                        if screen[index] {
                            ctx.draw(&Rectangle {
                                x: x as f64,
                                y: y as f64,
                                width: 1.0,
                                height: 1.0,
                                color: Color::Cyan,
                            })
                        }
                    }
                }
            });
        canvas
    }
} // End impl Gpu

/// I like to say Ratatui
impl Widget for &Gpu {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(Line::from(vec![
            " Canvas ".bold(),
            "<3".red().bold(),
            " Galus ".bold(),
        ]));

        let instructions = Title::from(Line::from(vec![
            " Left ".into(),
            "<H> ".blue().bold(),
            " Right ".into(),
            "<L> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        let block = Block::bordered()
            .title(title.alignment(Alignment::Right))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            " Value: ".into(),
            self.counter.to_string().yellow(),
            " ".into(),
        ])]);

        let paragraph = Paragraph::new(counter_text).alignment(Alignment::Center);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(block.inner(area));

        block.render(area, buf);
        paragraph.render(chunks[0], buf);
        self.content().render(chunks[1], buf);
    }
}

//--------------------------------------------------------------
// App
// Core App Logic
//--------------------------------------------------------------

//#[derive(Debug)]
struct App {
    mode: AppMode,
    states: Vec<TuiWidgetState>,
    tab_names: Vec<&'static str>,
    selected_tab: usize,
    progress_counter: Option<u16>,
    gpu: Gpu,
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

impl App {
    pub fn new() -> App {
        let states = vec![
            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
            TuiWidgetState::new().set_default_display_level(LevelFilter::Info),
        ];

        let tab_names = vec!["State 1", "State 2", "State 3", "State 4"];
        App {
            mode: AppMode::Run,
            states,
            tab_names,
            selected_tab: 0,
            progress_counter: None,
            gpu: Gpu::new(),
        }
    }

    fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        rx: mpsc::Receiver<AppEvent>,
    ) -> color_eyre::Result<()> {
        for event in rx {
            match event {
                AppEvent::UiEvent(event) => self.handle_ui_event(event),
                AppEvent::CounterChanged(value) => self.update_progress_bar(event, value),
            }
            if self.mode == AppMode::Quit {
                break;
            }
            self.draw(terminal)?;
        }
        Ok(())
    }

    fn update_progress_bar(&mut self, event: AppEvent, value: Option<u16>) {
        trace!(target: "App", "Updating progress bar {:?}", event);
        self.progress_counter = value;
        if value.is_none() {
            info!(target: "App", "Background task finished");
        }
    }

    fn handle_ui_event(&mut self, event: Event) {
        debug!(target: "App", "Handling UI event {:?}", event);
        let state = self.selected_state();

        if let Event::Key(key) = event {
            let code = key.code;

            match code.into() {
                KeyCode::Char('q') => self.mode = AppMode::Quit,
                KeyCode::Char('\t') => self.next_tab(),
                KeyCode::Tab => self.next_tab(),
                KeyCode::Char(' ') => state.transition(TuiWidgetEvent::SpaceKey),
                KeyCode::Esc => state.transition(TuiWidgetEvent::EscapeKey),
                KeyCode::PageUp => state.transition(TuiWidgetEvent::PrevPageKey),
                KeyCode::PageDown => state.transition(TuiWidgetEvent::NextPageKey),
                KeyCode::Up => state.transition(TuiWidgetEvent::UpKey),
                KeyCode::Down => state.transition(TuiWidgetEvent::DownKey),
                KeyCode::Left => state.transition(TuiWidgetEvent::LeftKey),
                KeyCode::Right => state.transition(TuiWidgetEvent::RightKey),
                KeyCode::Char('+') => state.transition(TuiWidgetEvent::PlusKey),
                KeyCode::Char('-') => state.transition(TuiWidgetEvent::MinusKey),
                KeyCode::Char('h') => state.transition(TuiWidgetEvent::HideKey),
                KeyCode::Char('f') => state.transition(TuiWidgetEvent::FocusKey),
                _ => (),
            }
        }
    }

    fn selected_state(&mut self) -> &mut TuiWidgetState {
        &mut self.states[self.selected_tab]
    }

    fn new_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) & self.tab_names.len();
    }

    fn draw(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        terminal.draw(|frame| {
            // Render the entire App widget.
            frame.render_widget(self, frame.area());
        })?;
        Ok(())
    }
} // End impl App

//--------------------------------------------------------------
// App
// The V in MVC
//--------------------------------------------------------------
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // manage layout here
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // renders the GPU widget in the left chunk
        self.gpu.render(chunks[0], buf);

        // renders the logger on the right
        let log_block = Block::bordered().title("Log Output");
        let log_content = Paragraph::new("Placeholder for log output");
        log_content.block(log_block).render(chunks[1], buf);
    }
}

// TODO: Figure out if I want this still.
pub fn start(mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
    let (tx, rx) = mpsc::channel();
    let event_tx = tx.clone();
    let progress_tx = tx.clone();

    thread::spawn(move || input_thread(event_tx));
    thread::spawn(move || progress_task(progress_tx).unwrap());
    thread::spawn(move || background_task());
    thread::spawn(move || background_task2());
    thread::spawn(move || heart_task());

    self.run(terminal, rx);

    Ok(())
}

// -------------------------------------------
// Utility
// Terminal Managment Functions
// -------------------------------------------

/// Initialize the terminal
fn init() -> io::Result<DefaultTerminal> {
    trace!(target:"tui", "Initializing terminal");
    enable_raw_mode()?; // takes input w/o w8n 4 newline, prevents keys being echo'd back
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    set_panic_hook();

    let mut terminal = ratatui::init(); // ratatui.rs has 'let mut terminal'
    Ok(terminal)
}

/// Restore the terminal to its original state
fn restore() -> io::Result<()> {
    trace!(target:"tui", "Restoring terminal");
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore();
        hook(panic_info);
    }))
}

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
                let st = Style::new().red().bold();
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
                lines.push(Lines::from(format!("{}: {}", evt.level, evt.msg())));
            }
        };

        match evt.level {
            log::Level::Error => {
                let st = Style::new().blue().bold();
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
