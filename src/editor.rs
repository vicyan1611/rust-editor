use crossterm::event::KeyModifiers;
use crossterm::event::{read, Event::Key, KeyCode::Char};

mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
    current_x: u16,
    current_y: u16
}

impl Editor {

    pub fn default() -> Editor {
        Editor { should_quit: false, current_x: 0, current_y: 0 }
    }

    fn draw_rows(&mut self) -> Result<(), std::io::Error> {
        let height = Terminal::size()?.1;
        for current_row in self.current_y+1..height {
            Terminal::move_cursor_to(0, current_row)?;
            Terminal::print(&'~')?;
            if current_row + 1 < height {
                Terminal::print(&'\r')?;
                Terminal::print(&'\n')?;
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        Terminal::initialize()?;
        self.current_x = 0;
        self.current_y = 0;
        self.refresh_screen()?;
        Self::repl(self)?;
        Terminal::terminate()?;
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
        } else {
            self.draw_rows()?;
            Terminal::move_cursor_to(self.current_x,self.current_y)?;
            Terminal::execute()?;
        }
        Ok(())
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            match read() {
                Ok(Key(event)) => {
                    if let Char(c) = event.code
                    {
                        Terminal::print(&c)?;
                        self.current_x += 1;
                        if KeyModifiers::CONTROL == event.modifiers
                        && c == 'q' {
                            break;
                        }
                    } 
                },
                Err(err) => println!("Error: {}", err),
                Ok(_) => {}

            }
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

}