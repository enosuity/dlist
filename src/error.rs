use thiserror::Error;



#[derive(Error, Debug)]
pub enum AppError {
  #[error("-n must be a number but was {0}")]
  InvalidNumberOfFiles(String),
}


impl From<&str> for AppError {
  fn from(message: &str) -> Self {
      Self::InvalidNumberOfFiles(message.to_string())
  }
}
