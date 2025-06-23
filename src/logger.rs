use crate::config::{Config, LogLevel, OutputTarget};
use crate::formatter::Formatter;
use crate::rotation::FileRotator;
use std::fmt::Arguments;
use std::io::{self, Write};
use std::sync::{Arc, Mutex, OnceLock};

static GLOBAL_LOGGER: OnceLock<Arc<Mutex<Logger>>> = OnceLock::new();

pub struct Logger {
    config: Config,
    formatter: Formatter,
    file_rotator: Option<FileRotator>,
}

impl Logger {
    pub fn new(config: Config) -> io::Result<Self> {
        let formatter = Formatter::new(&config);
        let file_rotator = if config.file_path.is_some() {
            Some(FileRotator::new(&config)?)
        } else {
            None
        };

        Ok(Logger {
            config,
            formatter,
            file_rotator,
        })
    }

    pub fn init(config: Config) -> io::Result<()> {
        let logger = Arc::new(Mutex::new(Logger::new(config)?));
        GLOBAL_LOGGER.set(logger).map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Logger already initialized")
        })?;
        Ok(())
    }

    pub fn log(&mut self, level: LogLevel, args: Arguments) -> io::Result<()> {
        if level < self.config.level {
            return Ok(());
        }

        let formatted_message = self.formatter.format(level, args);

        match &self.config.output {
            OutputTarget::Console => {
                self.write_to_console(&formatted_message)?;
            }
            OutputTarget::File => {
                self.write_to_file(&formatted_message)?;
            }
            OutputTarget::Both => {
                self.write_to_console(&formatted_message)?;
                self.write_to_file(&formatted_message)?;
            }
        }

        Ok(())
    }

    fn write_to_console(&self, message: &str) -> io::Result<()> {
        let mut stdout = io::stdout();
        stdout.write_all(message.as_bytes())?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
        Ok(())
    }

    fn write_to_file(&mut self, message: &str) -> io::Result<()> {
        if let Some(ref mut rotator) = self.file_rotator {
            rotator.write_log(message)?;
        }
        Ok(())
    }
}

pub fn log(level: LogLevel, args: Arguments) {
    if let Some(logger) = GLOBAL_LOGGER.get() {
        if let Ok(mut logger) = logger.lock() {
            let _ = logger.log(level, args);
        }
    }
}
