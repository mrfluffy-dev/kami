use crate::{
    get_an_history, get_an_progress, get_user_anime_progress, update_anime_progress,
    write_an_progress,
};
use crate::{get_episode_link, get_episodes, get_image, search_anime};
use crate::{open_cast, open_video};

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
use viuer::{print_from_file, terminal_size, Config};

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
    animes: (Vec<String>, Vec<String>, Vec<String>),
    image: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: StatefulList<String>,
    episodes: (Vec<String>, Vec<String>),
    title: String,
    ep: u64,
    progress: i32,
    anime_id: i32,
    token: String,
    provider: String,
    cast: (bool, String),
}

impl<'a> App {
    fn default() -> App {
        App {
            input: String::new(),
            animes: get_an_history(),
            image: String::new(),
            input_mode: InputMode::Normal,
            messages: StatefulList::with_items(Vec::new()),
            episodes: (Vec::new(), Vec::new()),
            title: String::new(),
            ep: 0,
            progress: 0,
            anime_id: 0,
            token: String::new(),
            provider: String::new(),
            cast: (false, "0".to_string()),
        }
    }
}

pub fn anime_ui(
    token: String,
    provider: String,
    cast: (bool, String),
) -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::default();
    app.token = token;
    app.provider = provider;
    app.cast = cast;
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
    fn change_image(app: &App) {
        //save as f32
        let (width, height) = terminal_size().to_owned();
        let width = width as f32;
        let height = height as f32;
        let sixel_support = viuer::is_sixel_supported();
        let config = match sixel_support {
            true => Config {
                x: ((width / 2.0) + 1.0).round() as u16,
                y: 2,
                width: Some((width / 1.3).round() as u32),
                height: Some((height * 1.5) as u32),
                restore_cursor: true,
                ..Default::default()
            },
            false => Config {
                x: ((width / 2.0) + 1.0).round() as u16,
                y: 2,
                width: Some(((width / 2.0) - 4.0).round() as u32),
                height: Some((height / 1.3).round() as u32),
                restore_cursor: true,
                ..Default::default()
            },
        };

        let config_path = dirs::config_dir().unwrap().join("kami");
        let image_path = config_path.join("tmp.jpg");
        get_image(&app.image, &image_path.to_str().unwrap());
        print_from_file(image_path, &config).expect("Image printing failed.");
    }
    app.messages.items.clear();
    for anime in &app.animes.1 {
        app.messages.push(anime.to_string());
    }
    app.input_mode = InputMode::Normal;

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('i') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    KeyCode::Left => app.messages.unselect(),
                    KeyCode::Char('h') => app.messages.unselect(),
                    KeyCode::Down => match ep_select {
                        true => {
                            app.messages.next();
                        }
                        false => {
                            app.messages.next();
                            let selected = app.messages.state.selected();
                            app.image = app.animes.2[selected.unwrap()].clone();
                            change_image(&app);
                        }
                    },
                    KeyCode::Char('j') => match ep_select {
                        true => {
                            app.messages.next();
                        }
                        false => {
                            app.messages.next();
                            let selected = app.messages.state.selected();
                            app.image = app.animes.2[selected.unwrap()].clone();
                            change_image(&app);
                        }
                    },
                    KeyCode::Up => match ep_select {
                        true => {
                            app.messages.previous();
                        }
                        false => {
                            app.messages.previous();
                            let selected = app.messages.state.selected();
                            app.image = app.animes.2[selected.unwrap()].clone();
                            change_image(&app);
                        }
                    },
                    KeyCode::Char('k') => match ep_select {
                        true => {
                            app.messages.previous();
                        }
                        false => {
                            app.messages.previous();
                            let selected = app.messages.state.selected();
                            app.image = app.animes.2[selected.unwrap()].clone();
                            change_image(&app);
                        }
                    },
                    //if KeyCode::Enter => {
                    KeyCode::Enter => {
                        if ep_select == false {
                            app.progress = 0;
                            let selected = app.messages.state.selected();
                            app.title = app.messages.items[selected.unwrap()].clone();
                            app.anime_id = app.animes.0[selected.unwrap()]
                                .clone()
                                .parse::<i32>()
                                .unwrap();
                            app.episodes = get_episodes(
                                &app.animes.0[selected.unwrap()].parse::<i32>().unwrap(),
                            );
                            app.messages.items.clear();
                            if app.token == "local" || app.anime_id == 0 {
                                app.progress = get_an_progress(&app.title) as i32;
                                app.messages.state.select(Some(app.progress as usize));
                            } else {
                                app.progress =
                                    get_user_anime_progress(app.anime_id, app.token.as_str());
                                app.messages.state.select(Some(app.progress as usize));
                            }
                            if app.episodes.0.len() == 1 {
                                let link = get_episode_link(&app.episodes.1[0]);
                                if !app.cast.0 {
                                    open_video((link, format!("{} Episode 1", &app.title)));
                                } else {
                                    open_cast(
                                        (link, format!("{} Episode 1", &app.title)),
                                        &app.cast.1,
                                    )
                                }
                                let selected = app.messages.state.selected();
                                let image_url = app.animes.2[selected.unwrap()].clone();
                                if app.token == "local" || app.anime_id == 0 {
                                    write_an_progress(
                                        (&app.title, &app.anime_id.to_string(), &image_url),
                                        &1,
                                    );
                                } else {
                                    update_anime_progress(app.anime_id, 1, app.token.as_str());
                                    write_an_progress(
                                        (&app.title, &app.anime_id.to_string(), &image_url),
                                        &1,
                                    );
                                }
                            } else {
                                for ep in 1..app.episodes.1.len() + 1 {
                                    app.messages.push(format!(
                                        "Episode {}: {}",
                                        ep,
                                        app.episodes.0[ep - 1]
                                    ));
                                }
                                ep_select = true;
                            }
                        } else {
                            let selected = app.messages.state.selected();
                            app.ep = app
                                .messages
                                .iter()
                                .nth(selected.unwrap())
                                .unwrap()
                                .replace("Episode ", "")
                                .split(":")
                                .collect::<Vec<&str>>()[0]
                                .parse::<u64>()
                                .unwrap();
                            let link = get_episode_link(&app.episodes.1[app.ep as usize - 1]);
                            if !app.cast.0 {
                                open_video((link, format!("{} Episode {}", &app.title, app.ep)));
                            } else {
                                open_cast(
                                    (link, format!("{} Episode {}", &app.title, app.ep)),
                                    &app.cast.1,
                                )
                            }
                            let image_url = &app.image;
                            if app.ep > app.progress as u64 {
                                if app.token == "local" || app.anime_id == 0 {
                                    write_an_progress(
                                        (&app.title, &app.anime_id.to_string(), &image_url),
                                        &app.ep,
                                    );
                                } else {
                                    update_anime_progress(
                                        app.anime_id,
                                        app.ep as usize,
                                        app.token.as_str(),
                                    );
                                    write_an_progress(
                                        (&app.title, &app.anime_id.to_string(), &image_url),
                                        &app.ep,
                                    );
                                }
                                app.progress = app.ep as i32;
                            }
                        }
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        //push app.input into app.messages with '
                        app.animes = search_anime(app.input.drain(..).collect());
                        app.messages.items.clear();
                        for anime in &app.animes.1 {
                            app.messages.push(anime.to_string());
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

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
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
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("list")
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(183, 142, 241))
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>");
    f.render_stateful_widget(messages, top_chunks[0], &mut app.messages.state);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("info")
        .border_type(BorderType::Rounded);
    f.render_widget(block, top_chunks[1]);

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
