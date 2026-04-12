use std::fs::File;
use std::io::StdinLock;
use std::io::{BufRead, BufReader};
pub enum InputSource {
    File(BufReader<File>),
    Stdin(StdinLock<'static>),
    String(String),
}

impl InputSource {
    pub fn read_line(&mut self) -> Option<String> {
        let mut buf = String::new();

        match self {
            InputSource::File(reader) => {
                match reader.read_line(&mut buf) {
                    Ok(0) | Err(_) => None,
                    Ok(_) => Some(buf),
                }
            }

            InputSource::String(s) => {
                if s.is_empty() {
                    None
                } else if let Some(pos) = s.find('\n') {
                    let line = s[..=pos].to_string();
                    *s = s[pos + 1..].to_string();
                    Some(line)
                } else {
                    let line = s.clone();
                    s.clear();
                    Some(line)
                }
            }

            InputSource::Stdin(stdin) => {
                match stdin.read_line(&mut buf) {
                    Ok(0) | Err(_) => None,
                    Ok(_) => Some(buf),
                }
            }
        }
    }
}