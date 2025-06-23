use zlogger::{init, init_with_config, Config, LogLevel, OutputTarget};
use zlogger::{trace, debug, info, warn, error};

fn main() {
    println!("=== Basic Usage Example ===");
    
    // Method 1: Initialize with default settings (reads from environment)
    init();
    
    info!("Logger initialized with default settings");
    debug!("This is a debug message");
    warn!("This is a warning message");
    error!("This is an error message");
    
    println!("\n=== Custom Configuration Example ===");
    
    // Method 2: Initialize with custom configuration
    let config = Config::default()
        .level(LogLevel::Debug)
        .output(OutputTarget::Both)
        .file_path("example.log")
        .max_file_size(1024 * 1024) // 1MB
        .max_files(3)
        .use_colors(true);
    
    // Note: This won't work if logger is already initialized
    // In a real app, you'd do this at the start
    println!("Custom config created: {:?}", config);
    
    // Demonstrate different log levels
    trace!("This trace message might not appear depending on log level");
    debug!("Debug: Application state: {}", "running");
    info!("Info: Processing {} items", 42);
    warn!("Warning: Low disk space: {}%", 85);
    error!("Error: Failed to connect to database: {}", "connection timeout");
    
    // Using format arguments
    let user = "Alice";
    let count = 10;
    info!("User {} has {} items in cart", user, count);
    
    println!("\nCheck 'example.log' if file output was configured!");
}
