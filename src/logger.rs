use log::{LogRecord, LogLevel, LogLevelFilter, LogMetadata, set_logger, Log};

pub struct Builder {
    lvl: LogLevelFilter
}

impl Builder {
    pub fn new(lvl: LogLevelFilter) -> Self {
        Builder { lvl: lvl }
    }

    pub fn enable(&self) {
        set_logger(|max_log_level| {
            max_log_level.set(self.lvl);
            Box::new(Logger)
        }).unwrap();
    }
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}
