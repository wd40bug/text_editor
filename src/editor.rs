use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

use termion::{
    color::{self, Bg, Black, Blue, Fg, Reset, White},
    event::Key,
    input::TermRead,
};

use crate::{
    document::Document, highlight::Type, row::Row, terminal::Terminal, Position, StatusMessage,
};

pub struct Editor {
    should_exit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
    offset: Position,
    message_buffer: Vec<String>,
    message: StatusMessage,
    unsaved_changes: bool,
}
impl Editor {
    //RUN
    ///# Panics
    ///
    /// will panic if something is wrong with the inputted key
    pub fn run(&mut self) {
        Terminal::clear_screen();
        self.document.highlight(&None);
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
    #[allow(clippy::cast_possible_truncation)]
    //RENDERING
    fn render(&mut self) {
        self.draw_rows();
        self.message_bar();
        self.stats_bar();
        Terminal::move_cursor(1, 1);
        self.render_cursor();
        Terminal::flush();
    }
    #[allow(clippy::cast_possible_truncation)]
    fn render_cursor(&self) {
        Terminal::move_cursor(
            self.cursor_position.x.saturating_sub(self.offset.x) as u16,
            self.cursor_position.y.saturating_sub(self.offset.y) as u16,
        );
    }
    pub fn draw_rows(&mut self) {
        print!("{}", termion::cursor::Hide);
        for row in 0..self.terminal.height as usize {
            Terminal::clear_row();
            if row == 0 {
                self.welcome();
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
                    "{}{}{}\r",
                    &self.document.rows[row + self.offset.y - 1].to_string(self.offset.x..=end),
                    Fg(Reset),
                    Bg(Reset),
                );
            } else {
                println!("~\r");
            }
        }
        print!("{}", termion::cursor::Show);
    }

    //DECODE KEYS
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let mut off = &mut self.offset;
        let width = self.terminal.width as usize;
        let height = self.terminal.height as usize;
        if y <= off.y {
            off.y = y - 1;
        } else if y >= off.y.saturating_add(height) {
            off.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < off.x {
            off.x = x;
        } else if x >= off.x.saturating_add(width) {
            off.x = x.saturating_sub(width).saturating_add(1);
        }
    }
    #[allow(clippy::if_same_then_else)]
    fn ctrl_decode(&mut self, key: char) {
        match key {
            'q' => {
                if !self.unsaved_changes {
                    println!("\r");
                    self.should_exit = true;
                } else if let Some('y') = self.prompt_char(
                    "This file has unsaved changes, are you sure you want to quit?(y,n)",
                ) {
                    println!("\r");
                    self.should_exit = true;
                }
            }
            's' => {
                self.document.highlight(&None);
                match self.document.path {
                    Some(_) => {
                        self.unsaved_changes = false;
                        self.document.save();
                    }
                    None => {
                        if let Some(path) = self.prompt("Save As") {
                            self.document.save_as(path);
                            self.unsaved_changes = false;
                        }
                    }
                }
            }
            'f' => {
                let query = self.prompt("Search");
                if let Some(string) = query {
                    self.document.highlight(&Some(string.clone()));
                    let finds = self.document.search(&string);
                    if finds.is_empty() {
                    } else if finds.len() == 1 {
                        self.cursor_position = finds[0].clone();
                    } else {
                        let mut current = 0;
                        loop {
                            self.cursor_position = finds[current].clone();
                            let key = Self::get_next_key().unwrap();
                            match key {
                                Key::Esc => {
                                    self.document.highlight(&None);
                                    break;
                                }
                                Key::Left => {
                                    if current > 0 {
                                        current -= 1;
                                    } else {
                                        current = finds.len() - 1;
                                    }
                                }
                                Key::Right => {
                                    if current < finds.len() - 1 {
                                        current += 1;
                                    } else {
                                        current = 0;
                                    }
                                }
                                _ => (),
                            }
                            self.scroll();
                            self.draw_rows();
                            self.message_bar();
                            Terminal::move_cursor(0, self.terminal.height + 1);
                            Terminal::clear_row();
                            print!("{}/{}", current, finds.len());
                            Terminal::move_cursor(1, 1);
                            self.render_cursor();
                            Terminal::flush();
                        }
                    }
                }
            }
            'd' => {
                let inner_words = self.document.rows[self.cursor_position.y - 1].get_inner_words();
                self.message_buffer.push(format!("{:?}", inner_words))
            }
            'n' => {
                self.message.time = Instant::now() - Duration::new(5, 0);
            }
            _ => (),
        }
    }
    fn delete(&mut self) {
        if self.cursor_position.x
            < self.document.rows[self.cursor_position.y - 1]
                .content
                .len()
                .saturating_sub(1)
        {
            self.unsaved_changes = true;
            self.document.rows[self.cursor_position.y - 1]
                .content
                .remove(self.cursor_position.x + 1);
        } else if self.cursor_position.y < self.document.rows.len() {
            self.unsaved_changes = true;
            let mut current_row = self.document.rows.remove(self.cursor_position.y).content;
            self.document.rows[self.cursor_position.y - 1]
                .content
                .append(&mut current_row);
        }
    }
    #[allow(
        clippy::match_same_arms,
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss
    )]
    fn decode_key(&mut self, key: Key) {
        match key {
            Key::Ctrl(x) => self.ctrl_decode(x),
            Key::Backspace => {
                if self.cursor_position.x > 0 {
                    self.unsaved_changes = true;
                    self.document.rows[self.cursor_position.y - 1]
                        .content
                        .remove(self.cursor_position.x - 1);
                    self.cursor_position.x -= 1;
                } else if self.cursor_position.y > 1 {
                    self.unsaved_changes = true;
                    let mut current_row = self
                        .document
                        .rows
                        .remove(self.cursor_position.y - 1)
                        .content;
                    self.document.rows[self.cursor_position.y - 2]
                        .content
                        .append(&mut current_row);
                    self.cursor_position.x =
                        self.document.rows[self.cursor_position.y - 2].content.len();
                    self.cursor_position.y -= 1;
                }
            }
            Key::Delete => self.delete(),
            Key::Char('\n') => {
                self.unsaved_changes = true;
                if self.cursor_position.x
                    < self.document.rows[self.cursor_position.y - 1].content.len()
                {
                    self.document.rows.insert(
                        self.cursor_position.y,
                        Row {
                            content: (&self.document.rows[self.cursor_position.y - 1].content
                                [self.cursor_position.x..])
                                .to_vec(),
                            highlighting: Vec::new(),
                        },
                    );
                    self.document.rows[self.cursor_position.y - 1].content = self.document.rows
                        [self.cursor_position.y - 1]
                        .content[..self.cursor_position.x]
                        .to_vec();
                } else {
                    self.document.rows.insert(
                        self.cursor_position.y,
                        Row {
                            content: Vec::new(),
                            highlighting: Vec::new(),
                        },
                    );
                }
                self.cursor_position.y += 1;
                self.cursor_position.x = 0;
            }
            Key::Char('\t') => {
                self.unsaved_changes = true;
                for _ in 0..4 {
                    self.document.rows[self.cursor_position.y - 1]
                        .content
                        .insert(self.cursor_position.x, " ".to_string());
                }
                self.cursor_position.x += 4;
            }
            Key::Char(x) => {
                self.unsaved_changes = true;
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
        let mut in_comment = if let Some(row) = self
            .document
            .rows
            .get(self.cursor_position.y.saturating_sub(2))
        {
            if row.highlighting.get(0) == Some(&Type::Comment) {
                true
            } else {
                false
            }
        } else {
            false
        };
        for i in -2..1 {
            if let Some(row) = self
                .document
                .rows
                .get_mut((self.cursor_position.y as isize).saturating_add(i) as usize)
            {
                row.highlight(
                    &None,
                    &self.document.file_type.highlight_ops.clone(),
                    &mut in_comment,
                );
            }
        }
        Terminal::flush();
        self.scroll();
    }
    #[allow(clippy::cast_possible_truncation)]
    fn prompt(&mut self, query: &str) -> Option<String> {
        Terminal::move_cursor(1, self.terminal.height + 1);
        Terminal::clear_row();
        let mut message = String::new();
        print!("{}: ", query);
        stdout().flush().unwrap();
        loop {
            let key = Self::get_next_key().unwrap();
            match key {
                Key::Esc => return None,
                Key::Char('\n') => {
                    break;
                }
                Key::Char(x) => {
                    print!("{}", x);
                    message.push(x);
                    stdout().flush().unwrap();
                }
                Key::Backspace => {
                    message.pop();
                    Terminal::move_cursor(message.len() as u16 + 2, self.terminal.height + 1);
                    print!(" ");
                    Terminal::move_cursor(message.len() as u16 + 2, self.terminal.height + 1);
                    stdout().flush().unwrap();
                }
                _ => (),
            }
        }
        Terminal::clear_row();
        Some(message)
    }
    fn prompt_char(&mut self, query: &str) -> Option<char> {
        Terminal::move_cursor(1, self.terminal.height + 1);
        Terminal::clear_row();
        print!("{}: ", query);
        stdout().flush().unwrap();
        loop {
            let key = Self::get_next_key().unwrap();
            match key {
                Key::Esc => {
                    Terminal::clear_row();
                    return None;
                }
                Key::Char(x) => {
                    Terminal::clear_row();
                    return Some(x);
                }
                _ => (),
            }
        }
    }
    //MOVE CURSOR
    #[allow(clippy::cast_possible_wrap)]
    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let width = self.document.rows[y - 1].content.len();
        let height = self.document.rows.len();
        match key {
            Key::Up => {
                if y > 1 {
                    y = y.saturating_sub(1);
                }
            }
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            Key::PageDown => {
                y = if y.saturating_add(self.terminal.height as usize) < height {
                    y + self.terminal.height as usize
                } else {
                    self.document.rows.len()
                }
            }
            Key::PageUp => {
                y = if y > self.terminal.height as usize {
                    y - self.terminal.height as usize
                } else {
                    1
                }
            }
            _ => (),
        }
        if x > self.document.rows[y - 1].content.len() {
            x = self.document.rows[y - 1].content.len();
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
        Terminal::clear_row();
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
        let content = format!(
            "{}{}{} lines: {} x: {} y: {} terminal width: {} terminal height: {} x offset: {} y offset: {} line length: {} highlighting: {:?}\r",
            color::Fg(Black),
            if let Some(path) = &self.document.path{
                path.clone().into_os_string().into_string().unwrap()
            }else{
                "None".to_string()
            },
            if self.unsaved_changes{
                "*"
            } else{
                ""
            },
            self.document.rows.len(),
            self.cursor_position.x,
            self.cursor_position.y,
            self.terminal.width,
            self.terminal.height,
            self.offset.x,
            self.offset.y,
            self.document.rows[self.cursor_position.y - 1].content.len(),
            self.document.rows[self.cursor_position.y - 1].highlighting.get(self.cursor_position.x),
        );
        println!(
            "{}{}{}\r",
            if content.len() > self.terminal.width as usize {
                &content[..self.terminal.width as usize]
            } else {
                &content
            },
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
            cursor_position: Position { x: 0, y: 1 },
            document: doc,
            offset: Position { x: 0, y: 0 },
            message_buffer: vec!["press ctrl+n to compose a status message".to_string()],
            message: StatusMessage::new("HELP: ctrl + q to quit".to_string()),
            unsaved_changes: false,
        }
    }
}
