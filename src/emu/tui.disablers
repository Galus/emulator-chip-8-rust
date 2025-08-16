use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    // What is raw_mode?
    //   Starts taking input immediately w/o waiting for newline
    //   and prevents typed keys being echo'd back
    enable_raw_mode()?;
    set_panic_hook();
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore();
        hook(panic_info);
    }))
}
//
//use ratatui::{
//    style::Color,
//    widgets::{canvas::*, *},
//};
//
//Canvas::default()
//    .block(Block::bordered().title("Canvas"))
//    .x_bounds([-180.0,180.0])
//    .y_bounds([-90.0,90.0])
//    .paint(|ctx| {
//        ctx.draw(&Map {
//            resolution: MapResolution::High,
//            color: Color::White,
//        });
//        ctx.layer();
//        ctx.draw(&Line {
//            x1: 0.0,
//            y1: 10.0,
//            x2: 10.0,
//            y2: 10.0,
//            color: Color::White,
//        });
//        ctx.draw(&Rectangle {
//            x: 10.0,
//            y: 20.0,
//            width: 10.0,
//            height: 10.0,
//            color: Color::Red,
//        });
//    });
