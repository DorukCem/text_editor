use std::io::{self, Read};

use crossterm::terminal;

pub struct Editor{

}

impl Editor {
    pub fn default() -> Self {
        Editor {  }
    }

    pub fn run(&self){
        terminal::enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            let b = b.unwrap();
            let c = b as char;
    
            if c.is_control() {
                println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
            } else {
                println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
            }
    
            if c == 'q' {
                break;
            }
        }
        terminal::disable_raw_mode().unwrap();
    }
}