use chrono::{Local, DateTime};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct GameResult {
  date_time: DateTime<Local>,
  result: String,
  pub attempts: u8,
}

impl GameResult {
    pub fn new() -> Self {
        GameResult {
            date_time: Local::now(),
            result: String::new(),
            attempts: 0,
        }
    }

    pub fn store(&self) -> Result<(), Box<dyn std::error::Error>> {
      let home_dir = std::env::var("HOME")?;
      let file_path = PathBuf::from(home_dir).join(".code-breaker/results.txt");

      let mut file = File::open(file_path)?;
      file.write(format!("date:{} - result:{} - attempts:{}", self.date_time, self.result, self.attempts).as_bytes())?;

      Ok(())
    }
}