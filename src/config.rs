use std::env;
use std::path::PathBuf;

/// Logger configuration
#[derive(Clone, Debug)]
pub struct Config {
    pub level: LogLevel,
    pub output: OutputTarget,
    pub file_path: Option<PathBuf>,
    pub max_file_size: u64,
    pub max_files: u32,
    pub use_colors: bool,
    pub include_timestamp: bool,
    pub include_level: bool,
    pub include_target: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

#[derive(Clone, Debug)]
pub enum OutputTarget {
    Console,
    File,
    Both,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            output: OutputTarget::Console,
            file_path: None,
            max_file_size: 10 * 1024 * 1024, // 10MB
            max_files: 5,
            use_colors: true,
            include_timestamp: true,
            include_level: true,
            include_target: false,
        }
    }
}

impl Config {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Config::default();

        // Parse log level
        if let Ok(level_str) = env::var("ZLOG_LEVEL") {
            config.level = match level_str.to_lowercase().as_str() {
                "trace" => LogLevel::Trace,
                "debug" => LogLevel::Debug,
                "info" => LogLevel::Info,
                "warn" | "warning" => LogLevel::Warn,
                "error" => LogLevel::Error,
                _ => LogLevel::Info,
            };
        }

        // Parse output target
        if let Ok(output_str) = env::var("ZLOG_OUTPUT") {
            config.output = match output_str.to_lowercase().as_str() {
                "console" => OutputTarget::Console,
                "file" => OutputTarget::File,
                "both" => OutputTarget::Both,
                _ => OutputTarget::Console,
            };
        }

        // Set file path
        if let Ok(file_path) = env::var("ZLOG_FILE") {
            config.file_path = Some(PathBuf::from(file_path));
        } else if matches!(config.output, OutputTarget::File | OutputTarget::Both) {
            config.file_path = Some(PathBuf::from("app.log"));
        }

        // Parse max file size
        if let Ok(size_str) = env::var("ZLOG_MAX_SIZE") {
            if let Ok(size) = size_str.parse::<u64>() {
                config.max_file_size = size;
            }
        }

        // Parse max files
        if let Ok(files_str) = env::var("ZLOG_MAX_FILES") {
            if let Ok(files) = files_str.parse::<u32>() {
                config.max_files = files;
            }
        }

        // Parse color setting
        if let Ok(color_str) = env::var("ZLOG_COLOR") {
            config.use_colors = match color_str.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => true,
                "false" | "0" | "no" | "off" => false,
                _ => true,
            };
        }

        config
    }

    /// Builder pattern methods
    pub fn level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    pub fn output(mut self, output: OutputTarget) -> Self {
        self.output = output;
        self
    }

    pub fn file_path<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.file_path = Some(path.into());
        self
    }

    pub fn max_file_size(mut self, size: u64) -> Self {
        self.max_file_size = size;
        self
    }

    pub fn max_files(mut self, count: u32) -> Self {
        self.max_files = count;
        self
    }

    pub fn use_colors(mut self, use_colors: bool) -> Self {
        self.use_colors = use_colors;
        self
    }
}
