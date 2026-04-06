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
        for current_row in self.current_x..height {
            print!("~");
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        Terminal::initialize()?;
        Self::draw_rows(self)?;
        Terminal::move_cursor_to(0, 0)?;
        self.current_x = 0;
        self.current_y = 0;
        Self::repl(self)?;
        Terminal::terminate()?;
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
        } else {
            self.draw_rows()?;
            Terminal::move_cursor_to(0,0)?;
        }
        Ok(())
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            // self.refresh_screen()?;
            match read() {
                Ok(Key(event)) => {
                    if let Char(c) = event.code
                    {
                        Terminal::print(&c )?;
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