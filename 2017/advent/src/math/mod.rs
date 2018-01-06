pub mod parser;
pub mod executor;

#[derive(Debug, Clone)]
pub struct Name(String);

#[derive(Debug, Clone)]
pub enum Statement {
  VarAssignment(Name, Expression),
  FnDefinition(Name, Vec<Name>, Expression)
}

#[derive(Debug, Clone)]
pub struct Expression(Operand, Vec<(Operator, Operand)>);

#[derive(Debug, Clone, Copy)]
pub enum Operator {
  Add,
  Subtract,
  Multiply,
  Divide
}

#[derive(Debug, Clone)]
pub enum Operand {
  I64(i64),
  VarSubstitution(Name),
  FnApplication(Name, Vec<Expression>)
}
