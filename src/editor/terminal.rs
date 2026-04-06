pub struct Terminal;
use std::io::{Write, stdout};
use crossterm::{execute, queue};
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};

impl Terminal {
    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(),
            Clear(ClearType::Purge),
            Clear(ClearType::All),
        )?;
        Self::move_cursor_to(0, 0)?;
        Ok(())
    }

    pub fn print(c: &char) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(c))?;
        Ok(())
    }

    pub fn initialize() -> Result<(), std::io::Error> {
        Self::clear_screen()?;
        enable_raw_mode()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }

    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }

    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }

}