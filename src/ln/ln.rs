use crate::ln::tracker::*;
use crate::ui::app::{app::KamiApp, ln::App};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

pub fn ln_ui(chapter: u32, reader: String) -> Result<(), Box<dyn Error>> {
    // setup terminal
    let _ = get_ln_json();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    app.reader = reader;
    let chapter = chapter as f64;
    app.current_page_number = 1;
    if chapter != 0.0 {
        app.current_page_number = (chapter / 48.0).ceil() as u32;
    }

    let res = app.run(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
