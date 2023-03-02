use super::*;

pub struct Parser {
  tokens: Vec<Token>,
  index: usize
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser { tokens, index: 0 }
  }

  pub fn produce_ast(&mut self) -> Program  {
    let mut program: Program = Program {
      kind: NodeType::Program,
      body: vec![]
    };

    while self.not_at_eof() {
      program.body.push(self.parse_statement());
    }
        
    return program;
  }

  fn at(&self) -> &Token {
    let tok = &self.tokens[self.index];
    return tok;
  }

  fn eat(&mut self) -> &Token {
    let token = &self.tokens[self.index];
    self.index += 1;

    return &token;
  }

  fn not_at_eof(&self) -> bool {
    self.tokens.len() > self.index
  }

  fn parse_statement(&mut self) -> BodyType {
    match self.at().kind {
      TokenType::Number => {
        let num = NumericLiteral { kind: NodeType::NumericLiteral, value: self.eat().value.parse().unwrap() };
        BodyType::Expression(Expression::NumericLiteral(num))
      },
      TokenType::Identifier => {
        let id = Identifier { kind: NodeType::Identifier, name: self.eat().value.to_owned() };
        BodyType::Expression(Expression::Identifier(id))
      },
      _ => {
        panic!("Unexpected token")
      }
    }
  }
}