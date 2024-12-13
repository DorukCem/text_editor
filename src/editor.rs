use crossterm::{
    event::{Event, KeyCode::Char},
    terminal,
};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        terminal::enable_raw_mode().unwrap();
        loop {
            match crossterm::event::read() {
                Ok(Event::Key(event)) => {
                    println!("{:?} \r", event);
                    match event.code {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                Err(err) => println!("{err}"),
                _ => (),
            }
        }

        terminal::disable_raw_mode().unwrap();
    }
}
