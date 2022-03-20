use std::io::{stdout, Stdout, Write};

use termion::raw::{IntoRawMode, RawTerminal};

use crate::Position;

pub struct Terminal {
    pub width: u16,
    pub height: u16,
    _stdout: RawTerminal<Stdout>,
}
impl Terminal {
    ///# Panics
    ///
    /// Will panic if the terminal cannot enter raw mode
    #[must_use]
    pub fn new(tuple: (u16, u16)) -> Terminal {
        Terminal {
            width: tuple.0,
            height: tuple.1.saturating_sub(2),
            _stdout: stdout().into_raw_mode().unwrap(),
        }
    }
    pub fn clear_screen() {
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    }
    pub fn move_cursor(x: u16, y: u16) {
        print!(
            "{}",
            termion::cursor::Goto(x.saturating_add(1), y.saturating_add(1))
        );
    }
    #[allow(clippy::cast_possible_truncation)]
    pub fn position_cursor(pos: &Position) {
        print!(
            "{}",
            termion::cursor::Goto(
                pos.x.saturating_add(1) as u16,
                pos.y.saturating_add(1) as u16
            )
        );
    }
    pub fn clear_row() {
        print!("{}", termion::clear::CurrentLine);
    }
    ///# Panics
    ///
    /// Will panic if flush fails
    pub fn flush() {
        stdout().flush().unwrap();
    }
}
