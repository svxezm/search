use crate::ui;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize, Debug)]
struct Config {
    pages: Vec<Page>,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct App {
    pub pages: Vec<Page>,
    pub selected: usize,
    pub search_content: String,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let entries = Self::parse_config();

        let pages: Vec<Page> = entries
            .pages
            .iter()
            .map(|entry| Page {
                name: entry.name.to_owned(),
                url: entry.url.to_owned(),
            })
            .collect();

        Self {
            pages,
            selected: 0,
            search_content: String::new(),
            exit: false,
        }
    }

    fn parse_config() -> Config {
        let json_data = std::fs::read_to_string("config.json")
            .expect("Failed to read configuration content");

        let pages: Config = serde_json::from_str(&json_data)
            .expect("Failed to parse configuration content");

        pages
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| ui::render(&self, frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            KeyCode::Esc => self.exit = true,
            KeyCode::Enter => self.web_search(),
            KeyCode::Char(ch) => self.search_content.push(ch),
            KeyCode::Backspace => {
                self.search_content.pop();
            }
            _ => {}
        }
    }

    fn select_previous(&mut self) {
        self.selected = match self.selected == 0 {
            true => self.pages.len() - 1,
            false => self.selected - 1,
        }
    }

    fn select_next(&mut self) {
        self.selected = match self.selected == self.pages.len() - 1 {
            true => 0,
            false => self.selected + 1,
        }
    }

    fn parse_web_browser() -> String {
        let output = Command::new("xdg-settings")
            .args(["get", "default-web-browser"])
            .output()
            .expect("Failed to find default browser");

        let browser = std::str::from_utf8(&output.stdout)
            .expect("Failed to get UTF-8 data from output")
            .trim();

        browser.split(".").next().unwrap().to_string()
    }

    fn web_search(&mut self) {
        let Some(page) = self.pages.get(self.selected) else {
            eprintln!("URL not found");
            return;
        };

        let browser = Self::parse_web_browser();
        let full_url = format!("{}{}", page.url, self.search_content);

        let status =
            std::process::Command::new(browser).arg(&full_url).status();
        if let Err(error) = status {
            eprintln!("Failed to open URL: {}", error);
            return;
        }

        self.exit = true;
    }
}
