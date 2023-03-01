pub mod lexer;

#[derive(Debug)]
pub enum TokenType {
  Number,
  Identifier,
  OpenParen,
  CloseParen,
  BinaryOperator,
  EOF,
}

#[derive(Debug)]
pub struct Token {
  pub kind: TokenType,
  pub value: String,
}
