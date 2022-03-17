use std::io::{stdout, Write};

use termion::{input::TermRead, raw::IntoRawMode, event::Key};
///# Panics
/// 
/// will panic if the terminal is not set up for raw mode
pub fn run(){
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = std::io::stdin();
    for c in stdin.keys(){
        match c{
            Ok(b)=>{
                match b {
                    Key::Ctrl('q')=>{
                        println!("\r");
                        break;
                    },
                    Key::Char(x)=>{
                        print!("{x}");
                        if x == '\n'{
                            print!("\r");
                        }
                    },
                    _=>(),
                }
                stdout.flush().unwrap();
            },
            Err(err)=>{
                panic!("{}", err);
            },
        }
    }
}