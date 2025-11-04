use crate::ui;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::process::Command;

#[derive(Debug)]
pub struct Pair {
    pub name: &'static str,
    pub url: &'static str,
}

#[derive(Debug)]
pub struct App {
    pub pairs: Vec<Pair>,
    pub selected: usize,
    pub search_content: String,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        let entries = vec![
            ("DuckDuckGo", "https://duckduckgo.com/?q="),
            ("URL Search", ""),
            ("Wikipedia", "https://en.wikipedia.org/wiki/"),
            ("YouTube", "https://www.youtube.com/results?search_query="),
            ("Rust Documentation", "https://doc.rust-lang.org/stable/"),
            ("Rust Crates", "https://crates.io/crates/"),
        ];

        let mut pairs: Vec<Pair> = Vec::new();
        for (name, url) in entries.iter() {
            let pair = Pair { name, url };
            pairs.push(pair);
        }

        Self {
            pairs,
            selected: 0,
            search_content: String::new(),
            exit: false,
        }
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
            true => self.pairs.len() - 1,
            false => self.selected - 1,
        }
    }

    fn select_next(&mut self) {
        self.selected = match self.selected == self.pairs.len() - 1 {
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
        let browser = Self::parse_web_browser();
        if let Some(pair) = self.pairs.get(self.selected) {
            let full_url = format!("{}{}", pair.url, self.search_content);
            std::process::Command::new(browser)
                .arg(full_url)
                .spawn()
                .expect("Failed to open URL");
            self.exit = true;
        }
    }
}
