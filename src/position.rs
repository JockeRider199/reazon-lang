pub struct Position {
  filename: String,
  index: usize,
  line: usize,
}

impl Position {
  pub fn new(filename: String) -> Position {
    Position {
      filename,
      index: 1,
      line: 1,
    }
  }

  pub fn advance(&mut self) {
    self.index += 1;
  }

  pub fn get_filename(&self) -> &str {
    return &self.filename;
  }

  pub fn get_idx(&self) -> &usize {
    return &self.index;
  }

  pub fn get_line(&self) -> &usize {
    return &self.line;
  }

  pub fn incr_idx(&mut self) {
    self.index += 1;
  }

  pub fn reset_idx(&mut self) {
    self.index = 1;
  }

  pub fn incr_line(&mut self) {
    self.line += 1;
  }
}
