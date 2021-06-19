use std::cmp::Ordering;
use std::time::SystemTime;

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
}

impl Logger {
    fn log(&self, level: &str, msg: String) {
        let time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_micros(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        let msg = format!("{:?} {} - {}", time, level, msg);
        println!("{}", msg);
    }

    pub fn new(level: Level) -> Self {
        Self { level }
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
