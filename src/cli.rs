use std::io::{self, stdout, Write};
use termion::raw::IntoRawMode;

pub struct CliController {
    stdout: Box<dyn io::Write>,
}

impl CliController {
    pub fn new() -> Self {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();
        Self {
            stdout: Box::from(stdout),
        }
    }

    pub fn move_up(&mut self, lines: usize) {
        if lines <= 0 {
            return;
        }
        write!(
            self.stdout.as_mut(),
            "{}",
            termion::cursor::Up(lines as u16)
        )
        .unwrap();
    }

    pub fn move_down(&mut self, lines: usize) {
        if lines <= 0 {
            return;
        }
        write!(
            self.stdout.as_mut(),
            "{}",
            termion::cursor::Down(lines as u16)
        )
        .unwrap();
    }

    pub fn clear_line(&mut self) {
        write!(self.stdout.as_mut(), "{}", termion::clear::CurrentLine).unwrap();
    }

    pub fn print(&mut self, msg: String) {
        write!(self.stdout.as_mut(), "{}", msg).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.as_mut().flush().unwrap();
    }
}

impl Drop for CliController {
    fn drop(&mut self) {
        write!(self.stdout.as_mut(), "{}", termion::cursor::Show).unwrap();
    }
}
