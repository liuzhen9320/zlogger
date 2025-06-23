//! # zlogger
//! 
//! ## Features
//! 
//! - Color-coded log levels for terminal output
//! - Environment variable configuration
//! - File logging with automatic rotation
//! - Multiple output targets (console, file, or both)
//! - Customizable log formats
//! - Thread-safe logging
//! 
//! ## Quick Start
//! 
//! ```rust
//! use colorful_logger::{init, info, warn, error};
//! 
//! // Initialize with default settings
//! init();
//! 
//! // Log messages
//! info!("Application started");
//! warn!("This is a warning");
//! error!("Something went wrong");
//! ```
//! 
//! ## Environment Variables
//! 
//! - `ZLOG_LEVEL`: Set log level (trace, debug, info, warn, error)
//! - `ZLOG_OUTPUT`: Set output target (console, file, both)
//! - `ZLOG_FILE`: Set log file path
//! - `ZLOG_MAX_SIZE`: Set max file size before rotation (in bytes)
//! - `ZLOG_MAX_FILES`: Set maximum number of rotated files to keep
//! - `ZLOG_COLOR`: Enable/disable colors (true/false)

pub mod logger;
pub mod config;
pub mod formatter;
pub mod rotation;
pub mod colors;

pub use logger::Logger;
pub use config::{Config, LogLevel, OutputTarget};

use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize the global logger with default configuration
pub fn init() {
    INIT.call_once(|| {
        let config = Config::from_env();
        Logger::init(config).expect("Failed to initialize logger");
    });
}

/// Initialize the global logger with custom configuration
pub fn init_with_config(config: Config) {
    INIT.call_once(|| {
        Logger::init(config).expect("Failed to initialize logger");
    });
}

/// Log a trace message
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::logger::log($crate::LogLevel::Trace, format_args!($($arg)*))
    };
}

/// Log a debug message
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logger::log($crate::LogLevel::Debug, format_args!($($arg)*))
    };
}

/// Log an info message
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::log($crate::LogLevel::Info, format_args!($($arg)*))
    };
}

/// Log a warning message
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::log($crate::LogLevel::Warn, format_args!($($arg)*))
    };
}

/// Log an error message
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::log($crate::LogLevel::Error, format_args!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macros() {
        init();
        trace!("This is a trace message");
        debug!("This is a debug message");
        info!("This is an info message");
        warn!("This is a warning message");
        error!("This is an error message");
    }
}
