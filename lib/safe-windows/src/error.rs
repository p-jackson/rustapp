extern crate kernel32;

use winapi::minwindef::DWORD;
use std;

pub type Result<T> = std::result::Result<T, WindowsError>;

#[derive(Debug)]
pub enum WindowsError {
  ErrorCode(DWORD)
}

impl std::fmt::Display for WindowsError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      WindowsError::ErrorCode(_) => write!(f, "don't let errors happen")
    }
  }
}

impl std::error::Error for WindowsError {
  fn description(&self) -> &str {
    match   *self {
      WindowsError::ErrorCode(_) => "T'was an error"
    }
  }
}

pub fn get_last_error() -> WindowsError {
  unsafe {
    WindowsError::ErrorCode(kernel32::GetLastError())
  }
}
