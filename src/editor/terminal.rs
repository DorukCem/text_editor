use std::io::{stdout, Write};

use crossterm::terminal::{self, Clear, ClearType};

#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        terminal::disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        terminal::enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), std::io::Error> {
        let Position { x, y } = pos;
        Self::queue_command(crossterm::cursor::MoveTo(x, y))?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        terminal::size().map(|(height, width)| Size { height, width })
    }

    pub fn hide_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::cursor::Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::cursor::Show)?;
        Ok(())
    }

    pub fn print(value: &str) -> Result<(), std::io::Error> {
        Self::queue_command(crossterm::style::Print(value))?;
        Ok(())
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

    fn queue_command<T:crossterm::Command>(command: T) -> Result<(), std::io::Error> {
        crossterm::queue!(stdout(), command)?;
        Ok(())
    }
}
