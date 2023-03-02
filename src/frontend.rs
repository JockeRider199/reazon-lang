pub mod lexer;
pub mod parser;

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

#[derive(Debug)]
pub enum NodeType {
  Program,

  // Statements

  //Expressions
  BinaryExr,

  // Literals
  NumericLiteral,
  Identifier
}

/* pub struct Statement {
  pub kind: NodeType
} */

#[derive(Debug)]
pub enum BodyType {
  Statement(Statement),
  Expression(Expression)
}
#[derive(Debug)]
pub enum Statement {}

#[derive(Debug)]
pub enum Expression {
  BinaryExpression(BinaryExpression),

  NumericLiteral(NumericLiteral),
  Identifier(Identifier)
}

#[derive(Debug)]
pub struct Program {
  pub kind: NodeType,
  pub body: Vec<BodyType>
}

#[derive(Debug)]
pub struct BinaryExpression {
  pub kind: NodeType,
  pub left: Box<Expression>,
  pub right: Box<Expression>,
  pub operator: String
}

#[derive(Debug)]
pub struct  Identifier {
  pub kind: NodeType,
  pub name: String
}

#[derive(Debug)]
pub struct NumericLiteral {
  pub kind: NodeType,
  pub value: usize
}