use std::{process::exit, fmt};

use colored::Colorize;

use crate::position::Position;

struct Error<'a> {
  kind: ErrorType,
  message: String,
  position: &'a Position,
  line_content: &'a str
}

impl Error<'_> {
  pub fn new<'a>(kind: ErrorType, message: String, position: &'a Position, line_content: &'a str) -> Error<'a> {
    Error {
      kind,
      message,
      position,
      line_content
    }
  }

  pub fn to_string(&self) -> String {

    format!(
      "{}: {} in {}:{}:{}.\n\n{}\n{}",
      self.kind.to_string().bold().red(),
      self.message,
      self.position.get_filename(),
      self.position.get_line(),
      self.position.get_idx(),
      self.line_content,
      " ".repeat((self.position.get_idx() - 1).try_into().unwrap()) + "^"
    )
  }
}


pub enum ErrorType {
  SyntaxError,
}

impl fmt::Display for ErrorType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ErrorType::SyntaxError => write!(f, "SyntaxError"),
    }
  }
}

pub fn throw_syntax_error(position: &Position, line_content: &str) {
  let e = Error::new(
    ErrorType::SyntaxError,
    "Unexpected character found".to_string(),
    position,
    line_content
  );
  println!("{}", e.to_string());
  exit(1);
}
