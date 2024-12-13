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

impl LogMessage {
    pub fn new(level: LogLevel, message: impl std::borrow::Borrow<str>) -> Self {
        let time = util::get_time();

        Self {
            level,
            message:   message.borrow().to_owned(),
            timestamp: time,
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

        for _ in 0..25 {

            let msg = LogMessage::new(LogLevel::Info, "foofjldksjfkldsjflkdsjfkldsjfklsdjflksdjflsdkjfsdkljfksdljfsdkljfdslkjfdlksjfsdkljfsdkljfsdlkflkjsfkldsjlksdjfldksjfldksjlkfsdjlfkdsjflksdjfldskj");
            s.append(msg);
            let msg = LogMessage::new(LogLevel::Warning, "bar");
            s.append(msg);
            let msg = LogMessage::new(LogLevel::Error, "baz");
            s.append(msg);
        }


        s
    }

    pub fn append(&mut self, message: LogMessage) {
        self.log.push(message);
    }

    pub fn clear(&mut self) {
        self.log.clear();
    }

}
