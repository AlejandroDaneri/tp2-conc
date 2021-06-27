use std::cmp::Ordering;
use std::time::SystemTime;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::borrow::Borrow;

#[derive(Copy, Clone)]
pub enum Level {
    Error = 4,
    Warn = 3,
    Info = 2,
    Debug = 1,
}

impl PartialEq<Level> for Level {
    fn eq(&self, other: &Level) -> bool {
        (*self as i32).eq(&(*other as i32))
    }
}

impl PartialOrd<Level> for Level {
    fn partial_cmp(&self, other: &Level) -> Option<Ordering> {
        (*self as i32).partial_cmp(&(*other as i32))
    }
}

pub struct Logger {
    level: Level,
    file: File,
}

impl Logger {
    fn log(&self, level: &str, msg: String) {
        let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_micros(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        let msg = format!("{:?} {} - {} \n", time, level, msg);
        //print!("{}", msg);
        self.file.borrow().write(msg.as_bytes()).expect("Couldn’t write to log file");
    }

    pub fn new(level: Level) -> Self {

        let path = Path::new("app.log");
        let display = path.display();

        let file = match OpenOptions::new()
            .create(true)
            .append(true)
            .open(path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        Self { level, file }
    }

    pub fn error(&self, msg: String) {
        if self.level <= Level::Error {
            self.log("ERROR", msg);
        }
    }

    pub fn warn(&self, msg: String) {
        if self.level <= Level::Warn {
            self.log("WARNING", msg);
        }
    }

    pub fn info(&self, msg: String) {
        if self.level <= Level::Info {
            self.log("INFO", msg);
        }
    }

    pub fn debug(&self, msg: String) {
        if self.level <= Level::Debug {
            self.log("DEBUG", msg);
        }
    }
}
