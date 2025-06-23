/// ANSI color codes for terminal output
#[derive(Clone, Copy, Debug)]
pub enum ColorCode {
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Dim,
    Bold,
}

impl ColorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColorCode::Reset => "\x1b[0m",
            ColorCode::Red => "\x1b[31m",
            ColorCode::Green => "\x1b[32m",
            ColorCode::Yellow => "\x1b[33m",
            ColorCode::Blue => "\x1b[34m",
            ColorCode::Magenta => "\x1b[35m",
            ColorCode::Cyan => "\x1b[36m",
            ColorCode::White => "\x1b[37m",
            ColorCode::Dim => "\x1b[2m",
            ColorCode::Bold => "\x1b[1m",
        }
    }

    /// Check if colors should be used based on environment
    pub fn should_use_colors() -> bool {
        // Check if we're in a terminal that supports colors
        std::env::var("TERM").map(|term| {
            !term.is_empty() && term != "dumb"
        }).unwrap_or(false) &&
        // Check if colors are not explicitly disabled
        std::env::var("NO_COLOR").is_err()
    }
}
