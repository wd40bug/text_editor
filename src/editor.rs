use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

use termion::{
    color::{self, Black, Blue, Reset, White},
    event::Key,
    input::TermRead,
};

use crate::{document::Document, terminal::Terminal, Position, StatusMessage};

pub struct Editor {
    should_exit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    message_buffer: Vec<String>,
    message: StatusMessage,
}
impl Editor {
    //RUN
    ///# Panics
    ///
    /// will panic if something is wrong with the inputted key
    pub fn run(&mut self) {
        Terminal::clear_screen();
        loop {
            self.render();
            let c = Self::get_next_key();
            if let Ok(key) = c {
                self.decode_key(key);
            } else {
                panic!()
            }
            if self.should_exit {
                Terminal::clear_screen();
                println!("goodbye! \r");
                Terminal::flush();
                break;
            }
        }
    }
    //STATIC
    fn get_next_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = std::io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    //RENDERING
    fn render(&mut self) {
        self.draw_rows();
        Terminal::position_cursor(&self.cursor_position);
        Terminal::flush();
    }
    pub fn draw_rows(&mut self) {
        print!("{}", termion::cursor::Hide);
        for row in 0..=self.terminal.height as usize {
            Terminal::clear_row();
            if row == 0 {
                self.welcome();
            } else if row == self.terminal.height as usize - 1 {
                self.message_bar();
            } else if row == self.terminal.height as usize {
                self.stats_bar();
            } else if (1..=self.document.rows.len()).contains(&(row + self.offset.y)) {
                if self.document.rows[row + self.offset.y - 1]
                    .content
                    .is_empty()
                {
                    println!();
                    continue;
                }
                let end = if self.document.rows[row + self.offset.y - 1].content.len()
                    > self.terminal.width as usize + self.offset.x - 2
                {
                    self.terminal.width as usize + self.offset.x - 2
                } else {
                    self.document.rows[row + self.offset.y - 1]
                        .content
                        .len()
                        .saturating_sub(1)
                };
                println!(
                    "{}\r",
                    &self.document.rows[row + self.offset.y - 1].to_string(self.offset.x..=end)
                );
            } else {
                println!("~\r");
            }
        }
        Terminal::move_cursor(1, 1);
        print!("{}", termion::cursor::Show);
    }

    //DECODE KEYS
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let mut off = &mut self.offset;
        let width = self.terminal.width;
        let height = self.terminal.height;
        if y < off.y {
            off.y = y;
        } else if y >= off.y.saturating_add(height) {
            off.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < off.x {
            off.x = x;
        } else if x >= off.x.saturating_add(height) {
            off.x = x.saturating_sub(height).saturating_add(1);
        }
    }
    #[allow(clippy::match_same_arms, clippy::cast_possible_truncation)]
    fn decode_key(&mut self, key: Key) {
        match key {
            Key::Ctrl('q') => {
                println!("\r");
                self.should_exit = true;
            }
            Key::Ctrl('n') => {
                Terminal::move_cursor(1, self.terminal.height);
                Terminal::clear_row();
                let mut message = String::new();
                print!("~");
                stdout().flush().unwrap();
                loop {
                    let key = Self::get_next_key().unwrap();
                    match key {
                        Key::Esc => break,
                        Key::Char('\n') => {
                            self.message_buffer.push(message.clone());
                            break;
                        }
                        Key::Char(x) => {
                            print!("{}", x);
                            message.push(x);
                            stdout().flush().unwrap();
                        }
                        Key::Backspace => {
                            message.pop();
                            Terminal::move_cursor(message.len() as u16 + 2, self.terminal.height);
                            print!(" ");
                            Terminal::move_cursor(message.len() as u16 + 2, self.terminal.height);
                            stdout().flush().unwrap();
                        }
                        _ => (),
                    }
                }
                Terminal::clear_row();
            }
            Key::Char(x) => {
                self.document.rows[self.cursor_position.y - 1]
                    .content
                    .insert(self.cursor_position.x, x.to_string());
                self.cursor_position.x += 1;
            }
            Key::Up
            | Key::Left
            | Key::Right
            | Key::Down
            | Key::PageUp
            | Key::PageDown
            | Key::End
            | Key::Home => self.move_cursor(key),
            _ => (),
        }
        Terminal::flush();
        self.scroll();
    }

    //MOVE CURSOR
    #[allow(clippy::cast_possible_wrap)]
    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => y = y.saturating_add(1),
            Key::Left => x = x.saturating_sub(1),
            Key::Right => x = x.saturating_add(1),
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }

    //BARS
    fn welcome(&self) {
        Terminal::move_cursor(self.terminal.width / 2 - 1, 0);
        println!(
            "{}Welcome to Saphire!{}\r",
            color::Fg(Blue),
            color::Fg(Reset)
        );
    }
    fn stats_bar(&mut self) {
        print!("{}", self.message.message);
        if Instant::now() - self.message.time > Duration::new(5, 0) {
            if self.message_buffer.is_empty() {
                self.message = StatusMessage::new("".to_string());
            } else {
                self.message = StatusMessage::new(self.message_buffer.remove(0));
            }
        }
    }
    fn message_bar(&self) {
        print!(
            "{}{}\r",
            color::Bg(White),
            " ".repeat(self.terminal.width as usize)
        );
        println!(
            "{}{} lines: {} bx: {} by: {} tx: {} ty: {} terminal width: {} terminal height: {} x offset: {} y offset: {} line length: {}{}{}\r",
            color::Fg(Black),
            self.document
                .path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
            self.document.rows.len(),
            self.cursor_position.x + self.offset.x,
            self.cursor_position.y + self.offset.y,
            self.cursor_position.x,
            self.cursor_position.y,
            self.terminal.width,
            self.terminal.height,
            self.offset.x,
            self.offset.y,
            self.document.rows[self.cursor_position.y+self.offset.y - 1].content.len(),
            color::Bg(Reset),
            color::Fg(Reset),
        );
    }

    //CONSTRUCTOR!
    ///# Panics
    ///
    /// panics if the terminal fails to initiate
    #[must_use]
    pub fn new(doc: Document) -> Editor {
        Editor {
            should_exit: false,
            terminal: Terminal::new(termion::terminal_size().unwrap()),
            cursor_position: Position { x: 1, y: 1 },
            document: doc,
            offset: Position { x: 0, y: 0 },
            message_buffer: vec!["press ctrl+n to compose a status message".to_string()],
            message: StatusMessage::new("HELP: ctrl + q to quit".to_string()),
        }
    }

    //BAD FUNCTIONS!
    fn up(&mut self, x: usize, y: usize, off: &Position) -> Position {
        let mut x = x;
        let mut y = y;
        if y > 1 {
            if y - self.offset.y == 1 {
                self.offset.y -= 1;
                if x > self.document.rows[y].content.len() {
                    x = self.document.rows[y].content.len();
                }
            } else {
                y = y.saturating_sub(1);
                if x > self.document.rows[y - 1].content.len() {
                    if self.document.rows[y - 1].content.len() > self.terminal.width as usize {
                        x = self.terminal.width as usize - 1 + off.x;
                        self.offset.x = self.document.rows[y - 1]
                            .content
                            .len()
                            .saturating_sub(self.terminal.width as usize - 1);
                    } else {
                        x = self.document.rows[y - 1].content.len() + off.x;
                        self.offset.x = 0;
                    }
                }
            }
        }
        Position { x, y }
    }
    fn left(&mut self, x: usize, y: usize, _off: &Position) -> Position {
        let mut x = x;
        if x > 0 {
            if x - self.offset.x == 0 {
                self.offset.x -= 1;
            } else {
                x = x.saturating_sub(1);
            }
        }
        Position { x, y }
    }
    fn right(&mut self, x: usize, y: usize, off: &Position) -> Position {
        let mut x = x;
        if x < self.document.rows[y - 1].content.len() as usize {
            if x > self.terminal.width as usize + off.x - 3 {
                self.offset.x += 1;
            } else {
                x = x.saturating_add(1);
            }
        }
        Position { x, y }
    }
    fn down(&mut self, x: usize, y: usize, off: &Position) -> Position {
        let mut x = x;
        let mut y = y;
        if y < self.document.rows.len() {
            if y >= self.terminal.height as usize + off.y - 2 {
                self.offset.y += 1;
                if x > self.document.rows[y].content.len() {
                    x = self.document.rows[y].content.len();
                }
            } else {
                y = y.saturating_add(1);
                if x > self.document.rows[y - 1].content.len() {
                    if self.document.rows[y - 1].content.len() > self.terminal.width as usize {
                        x = self.terminal.width as usize - 1 + off.x;
                        self.offset.x = self.document.rows[y - 1]
                            .content
                            .len()
                            .saturating_sub(self.terminal.width as usize - 1);
                    } else {
                        x = self.document.rows[y - 1].content.len() + off.x;
                        self.offset.x = 0;
                    }
                }
            }
        }
        Position { x, y }
    }
}
