use crossterm::{
    event::{read, Event, KeyCode::Char, KeyEvent, KeyModifiers},
    terminal,
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        terminal::enable_raw_mode()?;
        loop {
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r"
                );

                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }

            if self.should_quit {
                break;
            }
        }

        terminal::disable_raw_mode()?;
        Ok(())
    }
}
