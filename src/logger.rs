use serde::Serialize;

use crate::util;


const LOGDIRECTORY: &str = ".diagware_logs";


#[derive(Debug, Clone, Copy, Default, Serialize)]
pub enum LogLevel {
    #[default] Info,
    Warning,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LogLevel as L;
        let repr = match self {
            L::Info    => "Info",
            L::Warning => "Warn",
            L::Error   => "Error",
        };
        write!(f, "{}", repr)
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct LogMessage {
    pub level:     LogLevel,
    pub message:   String,
    pub timestamp: String,
}

impl LogMessage {
    pub fn new(level: LogLevel, message: impl std::borrow::Borrow<str>) -> Self {
        Self {
            level,
            message: message.borrow().to_owned(),
            timestamp: util::get_time()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Logger {
    pub log: Vec<LogMessage>,
}

impl Logger {

    pub fn new() -> Self {
        let mut s = Self {
            log: Vec::new(),
        };
        s.append(LogLevel::Info, "Logging initialized");
        s
    }

    pub fn append(&mut self, level: LogLevel, message: impl std::borrow::Borrow<str>) {
        let msg = LogMessage::new(level, message);
        self.log.push(msg);
    }

    pub fn clear(&mut self) {
        self.log.clear();
    }

    pub fn export(&mut self) {
        use std::fs::{File, DirBuilder};
        use std::io::Write;

        let log_json: String = serde_json::to_string_pretty(&self.log).unwrap();

        let filename = format!("{}_{}",
            chrono::Local::now()
                .date_naive()
                .format("%d_%m_%Y")
                .to_string(),
            chrono::Local::now()
                .time()
                .format("%H_%M_%S")
                .to_string()
        );

        let logpath = format!("{}/{LOGDIRECTORY}", env!("HOME"));
        let filepath = format!("{logpath}/{filename}.json");

        // Make sure log directory exists
        DirBuilder::new()
            .recursive(true)
            .create(logpath)
            .unwrap();

        let mut file = match File::create(&filepath) {
            Ok(f)  => f,
            Err(_) => {
                self.append(LogLevel::Error, "Saving log failed");
                return;
            }
        };

        if let Err(_) = file.write_all(log_json.as_bytes()) {
            self.append(LogLevel::Error, "Saving log failed");
            return;
        }

        self.append(LogLevel::Info, format!("Log saved to {filepath}"));

    }

}
