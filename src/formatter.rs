use crate::colors::ColorCode;
use crate::config::{Config, LogLevel, OutputTarget};
use chrono::{DateTime, Local};
use std::fmt::Arguments;

pub struct Formatter {
    use_colors: bool,
    include_timestamp: bool,
    include_level: bool,
    include_target: bool,
}

impl Formatter {
    pub fn new(config: &Config) -> Self {
        let use_colors = config.use_colors && matches!(config.output, OutputTarget::Console | OutputTarget::Both);
        
        Self {
            use_colors,
            include_timestamp: config.include_timestamp,
            include_level: config.include_level,
            include_target: config.include_target,
        }
    }

    pub fn format(&self, level: LogLevel, args: Arguments) -> String {
        let mut parts = Vec::new();

        // Timestamp
        if self.include_timestamp {
            let now: DateTime<Local> = Local::now();
            parts.push(format!("[{}]", now.format("%Y-%m-%d %H:%M:%S%.3f")));
        }

        // Log level (only the level text gets colored, not the brackets)
        if self.include_level {
            let level_str = if self.use_colors {
                self.colorize_level(&level)
            } else {
                format!("[{}]", level.as_str())
            };
            parts.push(level_str);
        }

        // Target (module path) - optional
        if self.include_target {
            // This would require more complex macro expansion to capture the module path
            // For now, we'll skip this feature to keep it simple
        }

        // Message (no coloring applied to the message itself)
        parts.push(format!("{}", args));

        parts.join(" ")
    }

    fn colorize_level(&self, level: &LogLevel) -> String {
        format!("[{}{}{}]",
            level.color_code().as_str(),
            level.as_str(),
            ColorCode::Reset.as_str()
        )
    }
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }

    pub fn color_code(&self) -> ColorCode {
        match self {
            LogLevel::Trace => ColorCode::Dim,
            LogLevel::Debug => ColorCode::Blue,
            LogLevel::Info => ColorCode::Green,
            LogLevel::Warn => ColorCode::Yellow,
            LogLevel::Error => ColorCode::Red,
        }
    }
}
