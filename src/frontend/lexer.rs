use crate::{error::throw_syntax_error, position::Position};

use super::{Token, TokenType};
use regex::Regex;

pub struct Lexer {
  pos: Position,
  source: Vec<String>,
  idx: usize,
}

impl Lexer {
  pub fn new(source: Vec<String>, filename: String) -> Lexer {
    Lexer {
      pos: Position::new(filename),
      source,
      idx: 0,
    }
  }

  pub fn make_tokens(&mut self) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    while self.is_not_eof() {
      match self.at() {
        "(" => tokens.push(make_token(TokenType::OpenParen, self.eat())),
        ")" => tokens.push(make_token(TokenType::CloseParen, self.eat())),
        "+" | "-" | "/" | "*" | "%" => {
          tokens.push(make_token(TokenType::BinaryOperator, self.eat()))
        },
        _ => {
          // proceed multichar or unrecognized char

          if is_int(&self.at()) {
            let mut number: String = String::new();
            while self.is_not_eof() && is_int(&self.at()) {
              number += &self.eat();
            }
    
            tokens.push(make_token(TokenType::Number, number));
          } else if is_alpha(&self.at()) {
            let mut ident: String = String::new();
            while self.is_not_eof() && is_alpha(&self.at()) {
              ident += &self.eat();
            }
    
            tokens.push(make_token(TokenType::Identifier, ident));
          } else {
            if is_skippable(&self.at()) {
              self.eat();
            } else {
              throw_syntax_error(&self.pos, &self.source.join(""));
            }
          }
        }
      }
    }

    /* tokens.push(
      make_token(TokenType::EOF, "EndOfFile".to_owned())
    ); */
    return tokens;
  }

  fn eat(&mut self) -> String {
    let char = &self.source[self.idx];

    match char.as_str() {
      "\n" | "\r" => {
        self.pos.incr_line();
        self.pos.reset_idx();
      }
      _ => {}
    }

    self.idx += 1;
    self.pos.incr_idx();

    return char.to_owned();
  }

  fn at(&self) -> &str {
    &self.source[self.idx]
  }

  fn is_not_eof(&self) -> bool {
    self.source.len() > self.idx
  }
}

fn is_alpha(text: &str) -> bool {
  let alpha_rgx = Regex::new(r"[a-zA-Z]+").unwrap();
  alpha_rgx.is_match(&text)
}

fn is_int(text: &str) -> bool {
  let int_rgx = Regex::new(r"[0-9]+").unwrap();
  int_rgx.is_match(&text)
}

fn make_token(kind: TokenType, value: String) -> Token {
  Token { kind, value }
}

fn is_skippable(char: &str) -> bool {
  match char {
    " " => true,
    "\n" => true,
    "\t" => true,
    "\r" => true,
    _ => false,
  }
}
