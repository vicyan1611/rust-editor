use super::terminal::Terminal;

pub struct View;

impl View {
    pub fn render(current_row: &u16) -> Result<(), std::io::Error> {
        let height = Terminal::size()?.1;
        for row in current_row+1..height {
            Terminal::move_cursor_to(0, row)?;
            Terminal::print(&'~')?;
            if row + 1 < height {
                Terminal::print(&'\r')?;
                Terminal::print(&'\n')?;
            }
        }
        Ok(())
    }
}