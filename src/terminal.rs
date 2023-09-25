use std::io::Stdin;

use crate::data::{Field, Space};
use crate::game::Event;
use crate::game::Frontend;
use termion::{
    clear, cursor,
    event::Key,
    input::{Keys, TermRead},
    terminal_size,
};

pub struct Terminal {
    size: Space,
    keys: Keys<Stdin>,
}

impl Terminal {
    pub fn new() -> Self {
        let cols_rows = terminal_size().unwrap();
        Self {
            size: Space {
                ver: cols_rows.1,
                hor: cols_rows.0,
            },
            keys: std::io::stdin().keys(),
        }
    }
}

impl Frontend for Terminal {
    fn show(&self, field: &Field, player: &Space) -> Result<(), std::io::Error> {
        println!(
            "{}{}{}{}{}{}{}{}",
            cursor::Hide,
            cursor::Goto(1, 1),
            clear::AfterCursor,
            cursor::Goto(1, 1),
            field,
            cursor::Goto(1 + player.hor, player.ver),
            cursor::Show,
            cursor::BlinkingBlock,
        );
        Ok(())
    }

    fn input(&mut self) -> Option<Event> {
        match self.keys.next().unwrap() {
            Ok(key) => match key {
                Key::Down => Some(Event::MoveDown),
                Key::Up => Some(Event::MoveUp),
                Key::Right => Some(Event::MoveRight),
                Key::Left => Some(Event::MoveLeft),
                Key::Char('_') => Some(Event::PutDominoHor),
                Key::Char('|') => Some(Event::PutDominoVer),
                Key::Char('/') => Some(Event::PutDominoRight),
                Key::Char('\\') => Some(Event::PutDominoLeft),
                Key::Ctrl('c') => Some(Event::Quit),
                _ => None,
            },
            Err(_) => None,
        }
    }

    fn size(&self) -> crate::data::Space {
        self.size
    }
}
