use super::*;
use std::collections::HashMap;

pub struct Executor {
  variables: HashMap<Name, i64>,
  functions: HashMap<Name, (Vec<Name>, Expression)>,
}

impl Executor {
  pub fn run(&mut self, statements: Vec<Statement>) {
    for statement in statements {
      self.execute(statement)
    }
  }

  pub fn execute(&mut self, statement: Statement) {
    match statement {
      Statement::VarAssignment(name, expr) => {
        self.variables.insert(name, self.evaluate_expression(expr));
      }
      Statement::FnDefinition(name, params, expr) => {
        self.functions.insert(name, (params, expr));
      }
    }
  }

  // @TODO: Evaluation.
  // 1. Replace all operands by their values.
  // @TODO: 2. Devise queue of evaluations from highest to lowest priority operations.
  // 3. Perform series of evaluations.
  // 4. Return the expr's evaluated value.
  fn evaluate_expression(&self, expr: Expression) -> i64 {
    let substituted_expr = (
      self.evaluate_operand(expr.0),
      expr.1.map(|(operator, operand)| (operator, self.evaluate_operand(operand))).collect()
    );
    let accumulator = substituted_expr.0;
    for (operator, operand) in substituted_expr.1 {
      accumulator = self.evaluate_operation(operator, accumulator, operand);
    }
    return accumulator;
  }

  fn evaluate_operation(&self, operator: Operator, operand1: i64, operand2: i64) -> i64 {
    match operator {
      Operator::Add => operand1 + operand2,
      Operator::Subtract => operand1 - operand2,
      Operator::Multiply => operand1 * operand2,
      Operator::Divide => operand1 / operand2
    }
  }

  fn evaluate_operand(&self, operand: Operand) -> i64 {
    match operand {
      Operand::I64(value) => value,
      Operand::VarSubstitution(name) => self.variables[name],
      Operand::FnApplication(name, args) => self.evaluate_function(name, args)
    }
  }

  // 1. Verify number of args matches the expected number of params.
  // 2. Replace all args by their values.
  // 3. Make a backup copy of the current variables.
  // 4. Insert all evaluated args as variables.
  // 5. Evaluate the function's expression as normal.
  // 6. Restore the variables to the backup.
  // 7. Return the function's evaluated value.
  // @TODO: This allows functions to access global variables. Hmmm.
  fn evaluate_function(&self, name: Name, args: Vec<Expression>) {
    assert_eq!(args.len(), self.functions[name].0.len());
    let evaluated_args = args.into_iter().map(self::evaluate_expression).collect();
    let backup_of_global_variables = self.variables.clone();
    for (name, value) in self.functions[name].0.iter().zip(evaluated_args.into_iter()) {
      self.variables.insert(name.clone(), value);
    }
    let result = self.evaluate_expression(self.functions[name].1);
    self.variables = backup_of_global_variables;
    return result;
  }
}
