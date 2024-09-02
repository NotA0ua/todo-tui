use std::error::Error;

use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    prelude::*,
};

use crate::{
    key_events::{key_backspace, key_char, key_down, key_enter, key_esc, key_up},
    todo::Todo,
    ui::ui,
};

pub struct App {
    pub is_running: bool,
    pub current_todo: usize,
    pub todos: Vec<Todo>,
    pub editing: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            is_running: true,
            current_todo: 0,
            todos: vec![Todo::default()],
            editing: false,
        }
    }
}

impl App {
    // pub fn from_file() -> Self {
    //     App {
    //         is_running: true,
    //         current_todo: 0,
    //         todos:
    //     }
    // }
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>> {
        while self.is_running {
            terminal.draw(|f| ui(f, self))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }

                match key.code {
                    KeyCode::Down => key_down(self),
                    KeyCode::Up => key_up(self),
                    KeyCode::Enter => key_enter(self),
                    KeyCode::Esc => key_esc(self),
                    KeyCode::Backspace => key_backspace(self),
                    KeyCode::Char(c) => key_char(self, c),
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
