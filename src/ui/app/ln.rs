use crate::ui::{app::app::KamiApp, input::InputMode, list::StatefulList};

use crate::ln::open_text::{open_bat, open_glow};
use crate::ln::scraper::*;
use crate::ln::tracker::*;
use crossterm::event::{self, Event, KeyCode};
use std::fs::File;
use std::io;
use std::io::Write;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use unicode_width::UnicodeWidthStr;

pub struct App {
    input: String, // Input box's value
    input_mode: InputMode,
    messages: StatefulList<String>, // History of recorded messages
    ln_titles: Vec<String>,
    ln_links: Vec<String>,
    title: String,
    ln_id: String,
    ln_chapters: Vec<String>,
    ln_chapters_links: Vec<String>,
    last_page: String,
    current_page: String,
    pub current_page_number: u32,
    pub reader: String,
}

impl<'a> KamiApp for App {
    fn new() -> Self {
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
            reader: "bat".to_string(),
        }
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut chapter_select = false;

        loop {
            terminal.draw(|f| self.ui(f))?;
            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => {
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            terminal.clear()?;
                            return Ok(());
                        }
                        KeyCode::Left => self.messages.unselect(),
                        KeyCode::Char('g') => self.messages.begin(),
                        KeyCode::Char('G') => self.messages.end(),
                        KeyCode::Down => self.messages.next(),
                        KeyCode::Char('j') => self.messages.next(),
                        KeyCode::Up => self.messages.previous(),
                        KeyCode::Char('k') => self.messages.previous(),

                        KeyCode::Char('h') => {
                            if self.current_page_number > 0 {
                                self.current_page_number -= 1;
                            }

                            self.current_page =
                                get_ln_next_page(&self.ln_id, &self.current_page_number);
                            self.ln_chapters = get_ln_chapters(&self.current_page);
                            self.ln_chapters_links = get_ln_chapters_urls(&self.current_page);
                            self.messages.items.clear();
                            for chapter in self.ln_chapters.iter() {
                                self.messages.push(chapter.to_string());
                            }
                        }

                        KeyCode::Char('l') => {
                            if self.current_page_number < self.last_page.parse::<u32>().unwrap() {
                                self.current_page_number += 1;
                            }
                            self.current_page =
                                get_ln_next_page(&self.ln_id, &self.current_page_number);
                            self.ln_chapters = get_ln_chapters(&self.current_page);
                            self.ln_chapters_links = get_ln_chapters_urls(&self.current_page);
                            self.messages.items.clear();
                            for chapter in self.ln_chapters.iter() {
                                self.messages.push(chapter.to_string());
                            }
                        }
                        //if KeyCode::Enter => {
                        KeyCode::Enter => {
                            if chapter_select == false {
                                let selected = self.messages.state.selected();
                                self.title = self
                                    .messages
                                    .iter()
                                    .nth(selected.unwrap())
                                    .unwrap()
                                    .to_string();
                                if self.current_page_number == 1 {
                                    let progress = get_ln_progress(&self.title);
                                    self.current_page_number = progress.0;
                                    self.messages.state.select(Some(progress.1));
                                }
                                let link = self.ln_links[selected.unwrap()].to_string();
                                let html = get_html(&link);
                                self.ln_id = get_ln_id(&html).to_string();
                                self.last_page = get_ln_last_page(&html);
                                self.current_page = get_ln_next_page(
                                    &self.ln_id.to_string(),
                                    &self.current_page_number,
                                );
                                self.ln_chapters = get_ln_chapters(&self.current_page);
                                self.ln_chapters_links = get_ln_chapters_urls(&self.current_page);
                                self.messages.items.clear();
                                for chapter in self.ln_chapters.iter() {
                                    self.messages.push(chapter.to_string());
                                }
                                chapter_select = true;
                            } else {
                                let selected = self.messages.state.selected();
                                let chapter_url =
                                    self.ln_chapters_links[selected.unwrap()].to_string();
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
                                let _ = match &*self.reader {
                                    "bat" => open_bat(),
                                    "glow" => open_glow(),
                                    &_ => todo!(),
                                };
                                write_ln_progress(
                                    &self.title,
                                    &self.current_page_number,
                                    &self.messages.state.selected().unwrap(),
                                );
                                terminal.clear()?;
                            }
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Enter => {
                            //push self.input into self.messages with '1
                            let search: String = self.input.drain(..).collect();
                            let search = search.replace(" ", "+");
                            let url = "https://readlightnovels.net/?s=".to_string();
                            let url = format!("{}{}", url, search.trim()).trim().to_string();
                            let html = get_html(&url);
                            let ln_list = get_ln_list(html.as_str());
                            self.ln_titles = get_ln_titles(&ln_list);
                            self.ln_links = get_ln_urls(&ln_list);
                            self.messages.items.clear();
                            //remove index 0 of self.ln_titles and self.ln_links
                            self.ln_titles.remove(0);
                            self.ln_links.remove(0);
                            for ln in &self.ln_titles {
                                self.messages.push(ln.to_string());
                            }
                            chapter_select = false;
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

        let (msg, style) = match self.input_mode {
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
            .block(Block::default().borders(Borders::ALL).title("list"))
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .bg(Color::Rgb(183, 142, 241))
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">>");
        f.render_stateful_widget(messages, chunks[0], &mut self.messages.state);

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
