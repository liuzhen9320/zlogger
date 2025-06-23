use crate::config::Config;
use std::fs::{File, OpenOptions};
use std::io::{self, Write, Seek, SeekFrom};
use std::path::PathBuf;

pub struct FileRotator {
    file_path: PathBuf,
    max_size: u64,
    max_files: u32,
    current_file: Option<File>,
}

impl FileRotator {
    pub fn new(config: &Config) -> io::Result<Self> {
        let file_path = config.file_path.as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "No file path specified"))?
            .clone();

        // Create parent directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let current_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;

        Ok(FileRotator {
            file_path,
            max_size: config.max_file_size,
            max_files: config.max_files,
            current_file: Some(current_file),
        })
    }

    pub fn write_log(&mut self, message: &str) -> io::Result<()> {
        // Ensure we have a valid file handle
        if self.current_file.is_none() {
            self.current_file = Some(OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.file_path)?);
        }

        // Check if rotation is needed
        let current_size = {
            let file = self.current_file.as_mut().unwrap();
            file.seek(SeekFrom::End(0))?
        };
        
        if current_size >= self.max_size {
            self.rotate_files()?;
        }

        // Write the message
        let file = self.current_file.as_mut().unwrap();
        file.write_all(message.as_bytes())?;
        file.write_all(b"\n")?;
        file.flush()?;

        Ok(())
    }

    fn rotate_files(&mut self) -> io::Result<()> {
        // Close current file by taking ownership and dropping it
        self.current_file.take();

        // Rotate existing files (move .1 to .2, .2 to .3, etc.)
        for i in (1..self.max_files).rev() {
            let old_path = self.rotated_file_path(i);
            let new_path = self.rotated_file_path(i + 1);
            
            if old_path.exists() {
                if new_path.exists() {
                    std::fs::remove_file(&new_path)?;
                }
                std::fs::rename(&old_path, &new_path)?;
            }
        }

        // Move current file to .1
        let rotated_path = self.rotated_file_path(1);
        if rotated_path.exists() {
            std::fs::remove_file(&rotated_path)?;
        }
        std::fs::rename(&self.file_path, &rotated_path)?;

        // Create new current file
        let new_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path)?;
        
        self.current_file = Some(new_file);

        Ok(())
    }

    fn rotated_file_path(&self, number: u32) -> PathBuf {
        let mut path = self.file_path.clone();
        let file_name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("log");
        
        let new_name = format!("{}.{}", file_name, number);
        path.set_file_name(new_name);
        path
    }
}
