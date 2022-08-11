use crate::ln::open_text::*;
use crate::ln::scraper::*;
use std::fs::File;
use std::io::Write;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

enum InputMode {
    Normal,
    Editing,
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: StatefulList<String>,
    ln_titles: Vec<String>,
    ln_links: Vec<String>,
    title: String,
    ln_id: String,
    ln_chapters: Vec<String>,
    ln_chapters_links: Vec<String>,
    last_page: String,
    current_page: String,
    current_page_number: u32,
}

impl<'a> App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: StatefulList::with_items(Vec::new()),
            ln_titles: Vec::new(),
            ln_links: Vec::new(),
            title: String::new(),
            ln_id: String::new(),
            ln_chapters: Vec::new(),
            ln_chapters_links: Vec::new(),
            last_page: String::new(),
            current_page: String::new(),
            current_page_number: 0,
        }
    }
}

pub fn ln_ui(chapter: u32) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::default();
    let chapter = chapter as f64;
    app.current_page_number = 1;
    if chapter != 0.0 {
        app.current_page_number = (chapter / 48.0).ceil() as u32;
    }

    let res = run_app(&mut terminal, app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut chapter_select = false;

    loop {
        terminal.clear()?;
        terminal.draw(|f| ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('i') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        terminal.clear()?;
                        return Ok(());
                    }
                    KeyCode::Left => app.messages.unselect(),
                    KeyCode::Char('h') => {
                        if app.current_page_number > 0 {
                            app.current_page_number -= 1;
                        }
                        app.current_page =
                            get_ln_next_page(&app.ln_id, &(app.current_page_number.to_string()));
                        app.ln_chapters = get_ln_chapters(&app.current_page);
                        app.ln_chapters_links = get_ln_chapters_urls(&app.current_page);
                        app.messages.items.clear();
                        for chapter in app.ln_chapters.iter() {
                            app.messages.push(chapter.to_string());
                        }
                    }
                    KeyCode::Down => app.messages.next(),
                    KeyCode::Char('j') => app.messages.next(),
                    KeyCode::Up => app.messages.previous(),
                    KeyCode::Char('k') => app.messages.previous(),
                    KeyCode::Char('l') => {
                        if app.current_page_number < app.last_page.parse::<u32>().unwrap() {
                            app.current_page_number += 1;
                        }
                        app.current_page =
                            get_ln_next_page(&app.ln_id, &(app.current_page_number.to_string()));
                        app.ln_chapters = get_ln_chapters(&app.current_page);
                        app.ln_chapters_links = get_ln_chapters_urls(&app.current_page);
                        app.messages.items.clear();
                        for chapter in app.ln_chapters.iter() {
                            app.messages.push(chapter.to_string());
                        }
                    }
                    //if KeyCode::Enter => {
                    KeyCode::Enter => {
                        if chapter_select == false {
                            let selected = app.messages.state.selected();
                            app.title = app
                                .messages
                                .iter()
                                .nth(selected.unwrap())
                                .unwrap()
                                .to_string();
                            let link = app.ln_links[selected.unwrap()].to_string();
                            let html = get_html(&link);
                            app.ln_id = get_ln_id(&html).to_string();
                            app.last_page = get_ln_last_page(&html);
                            app.current_page = get_ln_next_page(
                                &app.ln_id.to_string(),
                                &app.current_page_number.to_string(),
                            );
                            app.ln_chapters = get_ln_chapters(&app.current_page);
                            app.ln_chapters_links = get_ln_chapters_urls(&app.current_page);
                            app.messages.items.clear();
                            for chapter in app.ln_chapters.iter() {
                                app.messages.push(chapter.to_string());
                            }
                            chapter_select = true;
                        } else {
                            let selected = app.messages.state.selected();
                            let chapter_url = app.ln_chapters_links[selected.unwrap()].to_string();
                            let full_text = get_full_text(&chapter_url);
                            if cfg!(target_os = "windows") {
                                use dirs::home_dir;
                                let mut home = format!("{:?}", home_dir()).replace("\\\\", "/");
                                home.drain(0..6);
                                home.drain(home.len() - 2..home.len());
                                let mut file =
                                    File::create(format!("{}/AppData/Roaming/log_e", home))
                                        .expect("Unable to create file");
                                file.write_all(full_text.as_bytes())
                                    .expect("Unable to write to file");
                                file.sync_all().expect("Unable to sync file");
                            } else {
                                let mut file =
                                    File::create("/tmp/log_e").expect("Unable to create file");
                                file.write_all(full_text.as_bytes())
                                    .expect("Unable to write to file");
                                file.sync_all().expect("Unable to sync file");
                            };
                            terminal.clear()?;
                            let _ = open_bat();
                            terminal.clear()?;
                        }
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        //push app.input into app.messages with '1
                        let search: String = app.input.drain(..).collect();
                        let search = search.replace(" ", "+");
                        let url = "https://readlightnovels.net/?s=".to_string();
                        let url = format!("{}{}", url, search.trim()).trim().to_string();
                        let html = get_html(&url);
                        let ln_list = get_ln_list(html.as_str());
                        app.ln_titles = get_ln_titles(&ln_list);
                        app.ln_links = get_ln_urls(&ln_list);
                        app.messages.items.clear();
                        //remove index 0 of app.ln_titles and app.ln_links
                        app.ln_titles.remove(0);
                        app.ln_links.remove(0);
                        for ln in &app.ln_titles {
                            app.messages.push(ln.to_string());
                        }
                        chapter_select = false;
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());
    let block = Block::default()
        .borders(Borders::ALL)
        .title("kami")
        .border_type(BorderType::Rounded);
    f.render_widget(block, f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to search, "),
                Span::styled("h", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to go to the previous page, "),
                Span::styled("l", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to go to the next page."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to select."),
            ],
            Style::default(),
        ),
    };

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("list"))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(183, 142, 241))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");
    f.render_stateful_widget(messages, chunks[0], &mut app.messages.state);

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[1]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Rgb(183, 142, 241)),
        })
        .block(Block::default().borders(Borders::all()).title("Input"));
    f.render_widget(input, chunks[2]);
    match app.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[2].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + 1,
            )
        }
    }
}
