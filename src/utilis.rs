use std::fs::{OpenOptions, File};
use std::io::{BufWriter, Write};

pub struct HackerLogger {
    writer: BufWriter<File>,
}

impl HackerLogger {
    pub fn new(log_path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_path)?;
        Ok(HackerLogger {
            writer: BufWriter::new(file),
        })
    }

    pub fn log(&mut self, message: &str) -> std::io::Result<()> {
        writeln!(self.writer, "[{}] {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), message)?;
        self.writer.flush()?;
        Ok(())
    }
  }
