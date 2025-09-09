#![allow(dead_code)]
#![allow(unused_variables)]
use std::io;
use std::io::Write;

// I am thinking of making this a layer between the App and the Ratatui
// Contains the graphics processing.
// use crossterm::event::Event;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{
        Color,
        // Style,
        Stylize,
    },
    symbols::{border, Marker},
    text::{
        Line,
        // Span,
        Text,
    },
    widgets::{
        // block::{Position, Title},
        canvas::{Canvas, Rectangle},
        Block,
        Paragraph,
        Widget,
    },
    // DefaultTerminal,
};
// use std::sync::mpsc;
// use tui_logger::LevelFilter;
// use tui_logger::TuiWidgetState;

// use tui_logger::{ExtLogRecord, LogFormatter, TuiWidgetEvent};
//
// use crate::emu::KeyCode;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[derive(Debug)]
pub struct Gpu {
    pub screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
}

//--------------------------------------------------------------
// Gpu
// Here be graphics processing
//--------------------------------------------------------------
impl Gpu {
    pub fn new() -> Self {
        Self {
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn debug_screen_stdout(&self) {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        for (i, &pixel) in self.screen.iter().enumerate() {
            let character = match pixel {
                true => '▊',
                false => ' ',
            };
            write!(handle, "{}", character).expect("Failed to write to stdout");
            if (i + 1) % SCREEN_WIDTH == 0 {
                writeln!(handle).expect("Failed to write newline");
            }
        }
        handle.flush().expect("Couldnt flush!");
    }

    pub fn debug_screen_print_string(&self) {
        let mut screen_string = String::with_capacity(SCREEN_WIDTH * SCREEN_HEIGHT + SCREEN_HEIGHT);

        // Iterate over the screen array with the index `i`
        for (i, &pixel) in self.screen.iter().enumerate() {
            // Push the appropriate character to the string
            let character = match pixel {
                true => '▊',  // A block character for an "on" pixel
                false => ' ', // A space for an "off" pixel
            };
            screen_string.push(character);

            // After every row, push a newline character
            if (i + 1) % SCREEN_WIDTH == 0 {
                screen_string.push('\n');
            }
        }
        // Print the final string to the console
        //println!("{}", screen_string);
        info!(target: "gpu", "{}", screen_string);
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

    fn content(&self) -> impl Widget + '_ {
        let screen_ref = &self.screen;
        //let mut screen = self.screen.clone();
        //screen[1000..1099].copy_from_slice(&[true; 99]);

        let canvas = Canvas::default()
            .marker(Marker::Block)
            .block(Block::bordered().title("Canvas"))
            .x_bounds([0.0, SCREEN_WIDTH as f64])
            .y_bounds([0.0, SCREEN_HEIGHT as f64])
            .paint(move |ctx| {
                for y in 0..SCREEN_HEIGHT {
                    for x in 0..SCREEN_WIDTH {
                        let index = y * SCREEN_WIDTH + x;
                        if screen_ref[index] {
                            let flipped_y = (SCREEN_HEIGHT - 1 - y) as f64;
                            ctx.draw(&Rectangle {
                                x: x as f64,
                                y: flipped_y,
                                width: 0.8,
                                height: 0.4,
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
        let title = vec![" GPU".bold(), "<3".red().bold(), " Galus ".bold()];

        let instructions = vec![
            " Left ".into(),
            "<H> ".blue().bold(),
            " Right ".into(),
            "<L> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ];

        let block = Block::bordered()
            .title_top(title)
            .title_bottom(instructions)
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            " Value: ".into(),
            " PROGRESS_COUNTER_PLACEHOLDER".to_string().yellow(),
            //self.progress_counter.to_string().yellow(),
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
