
use core::cmp::min;

use crossterm::event::{
    read, Event,
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};
mod terminal;
mod view;
mod buffer;
use terminal::{Position, Size, Terminal};
use view::View;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Position,
    view: View,
}

impl Editor {

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }

        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn move_point(&mut self, keycode: KeyCode) -> Result<(), std::io::Error> {
        let Size { height, width } = Terminal::size()?;

        match keycode {
            KeyCode::Left => self.location.x = self.location.x.saturating_sub(1),
            KeyCode::Right => self.location.x = min(width.saturating_sub(1), self.location.x.saturating_add(1)),
            KeyCode::Up => self.location.y = self.location.y.saturating_sub(1),
            KeyCode::Down => self.location.y = min(height.saturating_sub(1), self.location.y.saturating_add(1)),

            KeyCode::Home => self.location.x = 0,
            KeyCode::End => self.location.x = width.saturating_sub(1),
            KeyCode::PageUp => self.location.y = 0,
            KeyCode::PageDown => self.location.y = height.saturating_sub(1),
            _ => (),
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            self.view.render()?;
            Terminal::move_cursor_to(self.location)?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    
}
