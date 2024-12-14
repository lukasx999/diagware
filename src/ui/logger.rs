use crate::ui::util;


#[derive(Debug, Clone, Copy, Default)]
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


#[derive(Debug, Clone)]
pub struct LogMessage {
    pub level:     LogLevel,
    pub message:   String,
    pub timestamp: String,
}


#[derive(Debug, Clone)]
pub struct Logger {
    pub log: Vec<LogMessage>,
}

impl Logger {

    pub fn new() -> Self {
        Self {
            log: Vec::new(),
        }
    }

    pub fn append(&mut self, level: LogLevel, message: impl std::borrow::Borrow<str>) {
        let msg = LogMessage {
            level,
            message: message.borrow().to_owned(),
            timestamp: util::get_time()
        };
        self.log.push(msg);
    }

    pub fn clear(&mut self) {
        self.log.clear();
    }

}
