use crossterm::event::KeyModifiers;
use crossterm::event::{read, Event::Key, KeyCode::Char};

mod terminal;
use terminal::Terminal;

mod view;
use view::View;

pub struct Editor {
    should_quit: bool,
    current_x: u16,
    current_y: u16
}

impl Editor {

    pub fn default() -> Editor {
        Editor { should_quit: false, current_x: 0, current_y: 0 }
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
            View::render(&self.current_y)?;
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
                Err(err) => eprintln!("Error: {}", err),
                Ok(_) => {}

            }
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

}