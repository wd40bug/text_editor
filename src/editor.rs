use std::io::{stdout, Stdout, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::terminal::Terminal;

pub struct Editor {
    stdout: RawTerminal<Stdout>,
    should_exit: bool,
    terminal: Terminal,
}
impl Editor {
    ///# Panics
    ///
    /// will panic if something is wrong with the inputted key
    pub fn run(&mut self) {
        loop {
            self.render();
            let c = Self::get_next_key();
            if let Ok(key) = c {
                self.decode_key(key);
            } else {
                panic!()
            }
            if self.should_exit {
                self.clear_screen();
                println!("goodbye! \r");
                break;
            }
        }
    }
    fn get_next_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = std::io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    fn render(&mut self) {
        self.clear_screen();
        self.draw_tildes();
    }
    fn clear_screen(&mut self) {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        self.stdout.flush().unwrap();
    }
    fn draw_tildes(&mut self) {
        for _ in 0..self.terminal.height {
            println!("~\r");
        }
        print!("{}", termion::cursor::Goto(1, 1));
        self.stdout.flush().unwrap();
    }
    fn decode_key(&mut self, key: Key) {
        match key {
            Key::Ctrl('q') => {
                println!("\r");
                self.should_exit = true;
            }
            Key::Char(x) => {
                print!("{x}");
                if x == '\n' {
                    print!("\r");
                }
            }
            _ => (),
        }
        self.stdout.flush().unwrap();
    }
}
impl Default for Editor {
    fn default() -> Self {
        Editor {
            stdout: stdout().into_raw_mode().unwrap(),
            should_exit: false,
            terminal: Terminal::new(termion::terminal_size().unwrap()),
        }
    }
}
