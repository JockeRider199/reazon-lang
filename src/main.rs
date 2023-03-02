#![allow(dead_code)]

extern crate regex;

pub mod error;
pub mod position;

mod frontend;

use frontend::{
  lexer::Lexer,
  parser::Parser
};
use std::io::{self, Write};

use crate::frontend::Program;

fn main() {
  loop {

    let input = repl();
    let res = split_input(input);
    let tok = Lexer::new(res, "main".to_string()).make_tokens();

    println!("{:?}", &tok);

    let program: Program = Parser::new(tok).produce_ast();

    println!("{:#?}", program);
  }
}

fn repl() -> String {
  print!("\n> ");
  io::stdout().flush().unwrap();

  let mut input = String::new();
  let stdin = io::stdin();

  stdin.read_line(&mut input).unwrap();

  return input;
}

//fn read_file() -> Vec<String> {}
fn split_input(input: String) -> Vec<String> {
  let mut chars = input
    .trim()
    .split("")
    .map(str::to_string)
    .collect::<Vec<String>>();
  chars = trimmed_vec(chars);

  return chars;
}

fn trimmed_vec(mut src: Vec<String>) -> Vec<String> {
  src.pop();
  src.remove(0);

  return src;
}
