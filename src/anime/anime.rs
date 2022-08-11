use crate::open_video;
use crate::{anime_ep_range, anime_link, anime_names};
use crate::{get_anime_id, get_token, get_user_anime_progress, update_anime_progress};

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
    title: String,
    ep: u64,
    progress: i32,
    anime_id: i32,
    token: String,
}

impl<'a> App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: StatefulList::with_items(Vec::new()),
            title: String::new(),
            ep: 0,
            progress: 0,
            anime_id: 0,
            token: String::new(),
        }
    }
}

pub fn anime_ui() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
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
    let mut ep_select = false;
    app.token = get_token();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('o') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Left => app.messages.unselect(),
                    KeyCode::Char('h') => app.messages.unselect(),
                    KeyCode::Down => app.messages.next(),
                    KeyCode::Char('j') => app.messages.next(),
                    KeyCode::Up => app.messages.previous(),
                    KeyCode::Char('k') => app.messages.previous(),
                    //if KeyCode::Enter => {
                    KeyCode::Enter => {
                        if ep_select == false {
                            let selected = app.messages.state.selected();
                            app.title = app
                                .messages
                                .iter()
                                .nth(selected.unwrap())
                                .unwrap()
                                .to_string();
                            let ep_range = anime_ep_range(&app.title);
                            app.title = app.title.replace("tv-", "");
                            app.title = app.title.replace("dub", "");
                            app.anime_id = get_anime_id(&app.title);
                            app.messages.items.clear();
                            app.progress =
                                get_user_anime_progress(app.anime_id, app.token.as_str());
                            //set app.messages.state.selected to app.progress
                            app.messages.state.select(Some(app.progress as usize));
                            for ep in 1..ep_range {
                                app.messages.push(format!("Episode {}", ep));
                            }
                            ep_select = true;
                        } else {
                            let selected = app.messages.state.selected();
                            app.ep = app
                                .messages
                                .iter()
                                .nth(selected.unwrap())
                                .unwrap()
                                .replace("Episode ", "")
                                .parse::<u64>()
                                .unwrap();
                            let link = anime_link(&app.title, app.ep);
                            open_video(link);
                            update_anime_progress(
                                app.anime_id,
                                app.ep as usize,
                                app.token.as_str(),
                            );
                        }
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        //push app.input into app.messages with '1
                        let anime_list = anime_names(app.input.drain(..).collect());
                        app.messages.items.clear();
                        for anime in anime_list {
                            app.messages.push(anime);
                        }
                        ep_select = false;
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
                Span::styled("o", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to search."),
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
