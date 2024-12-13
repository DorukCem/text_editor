use std::io::stdout;

use crossterm::terminal::{self, Clear, ClearType};

pub struct Terminal {

}

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        terminal::disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        crossterm::execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        crossterm::execute!(stdout(), crossterm::cursor::MoveTo(x, y))?;
        Ok(())
    }
    pub fn size() -> Result<(u16, u16), std::io::Error> {
        terminal::size()
    }
}