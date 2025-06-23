# zlogger.rs

A lightweight, configurable Rust logging library with color support and file rotation.

## Features

- 🌈 **Color-coded log levels** for terminal output
- 🔧 **Environment variable configuration**
- 📁 **File logging** with automatic rotation
- 🎯 **Multiple output targets** (console, file, or both)
- ⚡ **Thread-safe logging**
- 🪶 **Minimal dependencies**

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
zlogger = "0.0.1"
```

Basic usage:

```rust
use zlogger::{init, info, warn, error};

fn main() {
    // Initialize with default settings
    init();
    
    info!("Application started");
    warn!("This is a warning");
    error!("Something went wrong");
}
```

## Configuration

### Environment Variables

| Variable | Description | Default | Example |
|----------|-------------|---------|---------|
| `ZLOG_LEVEL` | Minimum log level | `info` | `debug` |
| `ZLOG_OUTPUT` | Output target | `console` | `file`, `both` |
| `ZLOG_FILE` | Log file path | `app.log` | `/var/log/myapp.log` |
| `ZLOG_MAX_SIZE` | Max file size (bytes) | `10485760` (10MB) | `1048576` (1MB) |
| `ZLOG_MAX_FILES` | Max rotated files | `5` | `10` |
| `ZLOG_COLOR` | Enable colors | `true` | `false` |

### Programmatic Configuration

```rust
use zlogger::{init_with_config, Config, LogLevel, OutputTarget};

let config = Config::default()
    .level(LogLevel::Debug)
    .output(OutputTarget::Both)
    .file_path("myapp.log")
    .max_file_size(5 * 1024 * 1024) // 5MB
    .max_files(10)
    .use_colors(true);

init_with_config(config);
```

## Log Levels

- `trace` - Very detailed debug information
- `debug` - Debug information
- `info` - General information
- `warn` - Warning messages
- `error` - Error messages

## File Rotation

When a log file reaches the maximum size, it's automatically rotated:
- `app.log` → `app.log.1`
- `app.log.1` → `app.log.2`
- etc.

Old files beyond the maximum count are automatically deleted.

## Examples

Run the basic example:

```bash
cargo run --example basic_usage
```

## License

Licensed under MIT license.

## Credits

- [@liuzhen9320](https://github.com/liuzhen9320) - Project author
- All contributors to the `rust-lang` ecosystem who have made this project possible

---

Built with ❤️ in Rust for the community