use crate::ui::{app::app::KamiApp, input::InputMode, list::StatefulList};

use crate::anime::player::{open_cast, open_video};
use crate::anime::scraper::{get_episode_link, get_episodes, get_image, search_anime};
use crate::anime::trackers::{
    get_an_history, get_an_progress, get_user_anime_progress, update_anime_progress,
    write_an_progress,
};

use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
use viuer::{print_from_file, terminal_size, Config};

pub struct App {
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
    link: String,
    ep: u64,
    progress: i32,
    anime_id: i32,
    pub token: String,
    pub provider: String,
    pub cast: (bool, String),
}

impl<'a> App {
    fn change_image(&mut self) {
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
        get_image(&self.image, &image_path.to_str().unwrap());
        print_from_file(image_path, &config).expect("Image printing failed.");
    }
}

impl<'a> KamiApp for App {
    fn new() -> Self {
        App {
            input: String::new(),
            animes: get_an_history(),
            image: String::new(),
            input_mode: InputMode::Normal,
            messages: StatefulList::with_items(Vec::new()),
            episodes: (Vec::new(), Vec::new()),
            title: String::new(),
            link: String::new(),
            ep: 0,
            progress: 0,
            anime_id: 0,
            token: String::new(),
            provider: String::new(),
            cast: (false, "0".to_string()),
        }
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut ep_select = false;
        self.messages.items.clear();
        for anime in &self.animes.1 {
            self.messages.push(anime.to_string());
        }
        self.input_mode = InputMode::Normal;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => {
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Left => self.messages.unselect(),
                        KeyCode::Char('h') => self.messages.unselect(),
                        KeyCode::Down => match ep_select {
                            true => {
                                self.messages.next();
                            }
                            false => {
                                self.messages.next();
                                let selected = self.messages.state.selected();
                                self.image = self.animes.2[selected.unwrap()].clone();
                                self.change_image();
                            }
                        },
                        KeyCode::Char('j') => match ep_select {
                            true => {
                                self.messages.next();
                            }
                            false => {
                                self.messages.next();
                                let selected = self.messages.state.selected();
                                self.image = self.animes.2[selected.unwrap()].clone();
                                self.change_image();
                            }
                        },
                        KeyCode::Up => match ep_select {
                            true => {
                                self.messages.previous();
                            }
                            false => {
                                self.messages.previous();
                                let selected = self.messages.state.selected();
                                self.image = self.animes.2[selected.unwrap()].clone();
                                self.change_image();
                            }
                        },
                        KeyCode::Char('k') => match ep_select {
                            true => {
                                self.messages.previous();
                            }
                            false => {
                                self.messages.previous();
                                let selected = self.messages.state.selected();
                                self.image = self.animes.2[selected.unwrap()].clone();
                                self.change_image();
                            }
                        },
                        //if KeyCode::Enter => {
                        KeyCode::Enter => {
                            if ep_select == false {
                                self.progress = 0;
                                let selected = self.messages.state.selected();
                                self.title = self.messages.items[selected.unwrap()].clone();
                                self.anime_id = self.animes.0[selected.unwrap()]
                                    .clone()
                                    .parse::<i32>()
                                    .unwrap();
                                self.episodes = get_episodes(
                                    &self.animes.0[selected.unwrap()].parse::<i32>().unwrap(),
                                    &self.provider,
                                );
                                self.messages.items.clear();
                                if self.token == "local" || self.anime_id == 0 {
                                    self.progress = get_an_progress(&self.title) as i32;
                                    self.messages.state.select(Some(self.progress as usize));
                                } else {
                                    self.progress =
                                        get_user_anime_progress(self.anime_id, self.token.as_str());
                                    self.messages.state.select(Some(self.progress as usize));
                                }
                                if self.episodes.0.len() == 1 {
                                    let link =
                                        get_episode_link(&self.episodes.1[0], &self.provider);
                                    if !self.cast.0 {
                                        open_video((
                                            link.0,
                                            format!("{} Episode 1", &self.title),
                                            link.1,
                                        ));
                                    } else {
                                        open_cast(
                                            (link.1, format!("{} Episode 1", &self.title)),
                                            &self.cast.1,
                                        )
                                    }
                                    let selected = self.messages.state.selected();
                                    let image_url = self.animes.2[selected.unwrap()].clone();
                                    if self.token == "local" || self.anime_id == 0 {
                                        write_an_progress(
                                            (&self.title, &self.anime_id.to_string(), &image_url),
                                            &1,
                                        );
                                    } else {
                                        update_anime_progress(
                                            self.anime_id,
                                            1,
                                            self.token.as_str(),
                                        );
                                        write_an_progress(
                                            (&self.title, &self.anime_id.to_string(), &image_url),
                                            &1,
                                        );
                                    }
                                } else {
                                    for ep in 1..self.episodes.1.len() + 1 {
                                        self.messages.push(format!(
                                            "Episode {}: {}",
                                            ep,
                                            self.episodes.0[ep - 1]
                                        ));
                                    }
                                    ep_select = true;
                                }
                            } else {
                                let selected = self.messages.state.selected();
                                self.ep = self
                                    .messages
                                    .iter()
                                    .nth(selected.unwrap())
                                    .unwrap()
                                    .replace("Episode ", "")
                                    .split(":")
                                    .collect::<Vec<&str>>()[0]
                                    .parse::<u64>()
                                    .unwrap();
                                let link = get_episode_link(
                                    &self.episodes.1[self.ep as usize - 1],
                                    &self.provider,
                                );
                                if !self.cast.0 {
                                    open_video((
                                        link.0,
                                        format!("{} Episode {}", &self.title, self.ep),
                                        link.1,
                                    ));
                                } else {
                                    open_cast(
                                        (link.0, format!("{} Episode {}", &self.title, self.ep)),
                                        &self.cast.1,
                                    )
                                }
                                let image_url = &self.image;
                                if self.ep > self.progress as u64 {
                                    if self.token == "local" || self.anime_id == 0 {
                                        write_an_progress(
                                            (&self.title, &self.anime_id.to_string(), &image_url),
                                            &self.ep,
                                        );
                                    } else {
                                        update_anime_progress(
                                            self.anime_id,
                                            self.ep as usize,
                                            self.token.as_str(),
                                        );
                                        write_an_progress(
                                            (&self.title, &self.anime_id.to_string(), &image_url),
                                            &self.ep,
                                        );
                                    }
                                    self.progress = self.ep as i32;
                                }
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            //push self.input into self.messages with '
                            self.animes = search_anime(self.input.drain(..).collect());
                            self.messages.items.clear();
                            for anime in &self.animes.1 {
                                self.messages.push(anime.to_string());
                            }
                            ep_select = false;
                            self.input_mode = InputMode::Normal;
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    fn ui<B: Backend>(&mut self, f: &mut Frame<B>) {
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

        let (msg, style) = match self.input_mode {
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

        let messages: Vec<ListItem> = self
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
        f.render_stateful_widget(messages, top_chunks[0], &mut self.messages.state);
        let block = Block::default()
            .borders(Borders::ALL)
            .title("info")
            .border_type(BorderType::Rounded);
        f.render_widget(block, top_chunks[1]);

        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let help_message = Paragraph::new(text);
        f.render_widget(help_message, chunks[1]);

        let input = Paragraph::new(self.input.as_ref())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Rgb(183, 142, 241)),
            })
            .block(Block::default().borders(Borders::all()).title("Input"));
        f.render_widget(input, chunks[2]);
        match self.input_mode {
            InputMode::Normal =>
                // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
                {}

            InputMode::Editing => {
                // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[2].x + self.input.width() as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[2].y + 1,
                )
            }
        }
    }
}
