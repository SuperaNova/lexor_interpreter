//! The LEXOR Execution Engine (Evaluator).
//!
//! Walks recursively through the Abstract Syntax Tree executing logic securely without panic.
//!
//! # Core Responsibilities
//! 1. **Tree Traversal:** Structurally unwraps nested AST Enums recursively passing universal scalable runtime `Object` values securely.
//! 2. **State Delegation:** Retrieves explicitly and persists user-defined memory values tightly natively into the integrated RAM `Environment`.
//! 3. **Real-World Side Effects:** Computes math equations natively securely triggering CLI output rendering sequentially.
//!
//! # Special LEXOR Execution Rules:
//! - `IF` conditions uniquely interpret generic Objects identically targeting strictly non-zero integers, or explicitly flagged `TRUE` booleans safely.
//! - Instantly structures and safely recursively bubbles exact `Object::Error` variants explicitly terminating executions purely if a math/type violation universally occurs.
//! - Identifiers natively requested lacking a strict preceding `DECLARE` block universally fail securely returning cache misses identically natively compiled C implementations would.

use crate::ast::{Expression, Program, Statement};
use crate::environment::Environment;
use crate::object::Object;
use crate::tokens::Token;
use std::io::{self, Write};

pub fn eval_program(program: &Program, env: &mut Environment) -> Option<Object> {
    let mut result = None;

    for statement in &program.statements {
        result = eval_statement(statement, env);

        if let Some(Object::Error(_)) = result {
            return result;
        }
    }

    result
}

fn eval_block_statement(statements: &Vec<Statement>, env: &mut Environment) -> Option<Object> {
    let mut result = None;

    for statement in statements {
        result = eval_statement(statement, env);

        if let Some(Object::Error(_)) = result {
            return result;
        }
    }

    result.or(Some(Object::Null))
}

fn eval_statement(statement: &Statement, env: &mut Environment) -> Option<Object> {
    match statement {
        Statement::Expression(expr) => eval_expression(expr, env),

        Statement::Declare(_var_type, variables) => {
            for (name, init_expr) in variables {
                let init_val = match init_expr {
                    Some(expr) => eval_expression(expr, env)?,
                    None => Object::Null,
                };
                env.set(name.clone(), init_val);
            }
            Some(Object::Null)
        }

        Statement::Print(expr) => {
            let val = eval_expression(expr, env)?;
            print!("{}", val);
            io::stdout().flush().unwrap(); // Flush strictly ensures immediate render
            Some(Object::Null)
        }

        Statement::Scan(variables) => {
            for var_name in variables {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to cleanly read terminal line input");
                // Smart parsing of user input logic:
                let trimmed = input.trim();
                let obj = if let Ok(i) = trimmed.parse::<i32>() {
                    Object::Integer(i)
                } else if let Ok(f) = trimmed.parse::<f32>() {
                    Object::Float(f)
                } else if trimmed == "TRUE" {
                    Object::Boolean(true)
                } else if trimmed == "FALSE" {
                    Object::Boolean(false)
                } else {
                    Object::String(trimmed.to_string())
                };

                env.set(var_name.clone(), obj);
            }
            Some(Object::Null)
        }

        Statement::If {
            condition,
            consequence,
            alternative,
        } => {
            let cond_val = eval_expression(condition, env)?;
            if is_truthy(cond_val) {
                eval_block_statement(consequence, env)
            } else if let Some(alt) = alternative {
                eval_block_statement(alt, env)
            } else {
                Some(Object::Null)
            }
        }

        Statement::RepeatWhen { condition, body } => {
            loop {
                let cond_val = eval_expression(condition, env)?;
                if !is_truthy(cond_val) {
                    break;
                }

                let result = eval_block_statement(body, env);
                if let Some(Object::Error(_)) = result {
                    return result;
                }
            }
            Some(Object::Null)
        }

        Statement::For {
            initialization,
            condition,
            update,
            body,
        } => {
            let init_result = eval_statement(initialization, env);
            if let Some(Object::Error(_)) = init_result {
                return init_result;
            }

            loop {
                let cond_val = eval_expression(condition, env)?;
                if !is_truthy(cond_val) {
                    break;
                }

                let result = eval_block_statement(body, env);
                if let Some(Object::Error(_)) = result {
                    return result;
                }

                let update_result = eval_statement(update, env);
                if let Some(Object::Error(_)) = update_result {
                    return update_result;
                }
            }
            Some(Object::Null)
        }
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(b) => b,
        Object::Integer(i) => i != 0,
        Object::Float(f) => f != 0.0,
        _ => true,
    }
}

fn eval_expression(expression: &Expression, env: &mut Environment) -> Option<Object> {
    match expression {
        Expression::IntLiteral(val) => Some(Object::Integer(*val)),
        Expression::FloatLiteral(val) => Some(Object::Float(*val)),
        Expression::BoolLiteral(val) => Some(Object::Boolean(*val)),
        Expression::CharLiteral(val) => Some(Object::Character(*val)),
        Expression::StringLiteral(val) => Some(Object::String(val.clone())),

        Expression::Identifier(name) => match env.get(name) {
            Some(val) => Some(val.clone()),
            None => Some(Object::Error(format!(
                "Identifier memory cache lookup failed (not declared): {}",
                name
            ))),
        },

        Expression::Prefix { operator, right } => {
            let right_val = eval_expression(right, env)?;
            eval_prefix_expression(operator, right_val)
        }

        Expression::Infix {
            left,
            operator,
            right,
        } => {
            if *operator == Token::Assign {
                if let Expression::Identifier(name) = &**left {
                    let val = eval_expression(right, env)?;
                    if env.get(name).is_none() {
                        return Some(Object::Error(format!(
                            "Cannot assign to strictly undeclared variable '{}'",
                            name
                        )));
                    }
                    env.set(name.clone(), val.clone());
                    return Some(val);
                } else {
                    return Some(Object::Error(format!(
                        "Invalid assignment structural target: {:?}",
                        left
                    )));
                }
            }

            let left_val = eval_expression(left, env)?;
            let right_val = eval_expression(right, env)?;
            eval_infix_expression(operator, left_val, right_val, env)
        }
    }
}

fn eval_prefix_expression(operator: &Token, right: Object) -> Option<Object> {
    match operator {
        Token::Minus => eval_minus_prefix_operator_expression(right),
        Token::Not => eval_not_operator_expression(right),
        _ => Some(Object::Error(format!(
            "Unknown mathematical abstract prefix operator: {:?}",
            operator
        ))),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Option<Object> {
    match right {
        Object::Integer(value) => Some(Object::Integer(-value)),
        Object::Float(value) => Some(Object::Float(-value)),
        _ => Some(Object::Error(format!(
            "Unsupported minus operator negation target: -{}",
            right
        ))),
    }
}

fn eval_not_operator_expression(right: Object) -> Option<Object> {
    match right {
        Object::Boolean(value) => Some(Object::Boolean(!value)),
        _ => Some(Object::Error(format!(
            "Unsupported structural NOT operator target: NOT {}",
            right
        ))),
    }
}

fn eval_infix_expression(
    operator: &Token,
    left: Object,
    right: Object,
    _env: &mut Environment,
) -> Option<Object> {
    // Generic concatenations natively bubble formatting through everything
    if let Token::Concat = operator {
        return Some(Object::String(format!("{}{}", left, right)));
    }

    if *operator == Token::And
        || *operator == Token::Or
        || *operator == Token::Eq
        || *operator == Token::Neq
    {
        if let (Object::Boolean(l), Object::Boolean(r)) = (&left, &right) {
            match operator {
                Token::And => return Some(Object::Boolean(*l && *r)),
                Token::Or => return Some(Object::Boolean(*l || *r)),
                Token::Eq => return Some(Object::Boolean(l == r)),
                Token::Neq => return Some(Object::Boolean(l != r)),
                _ => {}
            }
        }
    }

    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix_expression(operator, l, r),
        (Object::Float(l), Object::Float(r)) => eval_float_infix_expression(operator, l, r),
        (l, r) => {
            if *operator == Token::Eq {
                Some(Object::Boolean(l == r))
            } else if *operator == Token::Neq {
                Some(Object::Boolean(l != r))
            } else {
                Some(Object::Error(format!(
                    "Type mismatch securely trapped: {} {:?} {}",
                    l, operator, r
                )))
            }
        }
    }
}

fn eval_integer_infix_expression(operator: &Token, left: i32, right: i32) -> Option<Object> {
    match operator {
        Token::Plus => Some(Object::Integer(left + right)),
        Token::Minus => Some(Object::Integer(left - right)),
        Token::Star => Some(Object::Integer(left * right)),
        Token::Slash => {
            if right == 0 {
                return Some(Object::Error(String::from(
                    "Attempted to divide exclusively by zero. Halt!",
                )));
            }
            Some(Object::Integer(left / right))
        }
        Token::Modulo => {
            if right == 0 {
                return Some(Object::Error(String::from(
                    "Attempted modulo completely by zero. Halt!",
                )));
            }
            Some(Object::Integer(left % right))
        }
        Token::Lt => Some(Object::Boolean(left < right)),
        Token::Gt => Some(Object::Boolean(left > right)),
        Token::Lte => Some(Object::Boolean(left <= right)),
        Token::Gte => Some(Object::Boolean(left >= right)),
        Token::Eq => Some(Object::Boolean(left == right)),
        Token::Neq => Some(Object::Boolean(left != right)),
        _ => Some(Object::Error(format!(
            "Unknown exact integer logic operator: {:?}",
            operator
        ))),
    }
}

fn eval_float_infix_expression(operator: &Token, left: f32, right: f32) -> Option<Object> {
    match operator {
        Token::Plus => Some(Object::Float(left + right)),
        Token::Minus => Some(Object::Float(left - right)),
        Token::Star => Some(Object::Float(left * right)),
        Token::Slash => {
            if right == 0.0 {
                return Some(Object::Error(String::from(
                    "Attempted precision float division firmly by zero. Halt.",
                )));
            }
            Some(Object::Float(left / right))
        }
        Token::Lt => Some(Object::Boolean(left < right)),
        Token::Gt => Some(Object::Boolean(left > right)),
        Token::Lte => Some(Object::Boolean(left <= right)),
        Token::Gte => Some(Object::Boolean(left >= right)),
        Token::Eq => Some(Object::Boolean(left == right)),
        Token::Neq => Some(Object::Boolean(left != right)),
        _ => Some(Object::Error(format!(
            "Unknown precision float operator: {:?}",
            operator
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn eval(input: &str) -> Option<Object> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let mut env = Environment::new();
        eval_program(&program, &mut env)
    }

    #[test]
    fn test_eval_integer_expression() {
        assert_eq!(
            eval("SCRIPT AREA \n START SCRIPT \n 5 \n END SCRIPT").unwrap(),
            Object::Integer(5)
        );
        assert_eq!(
            eval("SCRIPT AREA \n START SCRIPT \n 10 + 5 \n END SCRIPT").unwrap(),
            Object::Integer(15)
        );
    }

    #[test]
    fn test_eval_declare_and_assign() {
        let input = "
SCRIPT AREA
START SCRIPT
    DECLARE INT x = 5
    x = x + 10
    x
END SCRIPT
";
        assert_eq!(eval(input).unwrap(), Object::Integer(15));
    }

    #[test]
    fn test_concat() {
        let input = "
SCRIPT AREA
START SCRIPT
    DECLARE CHAR x = 'a'
    x & \"bc\"
END SCRIPT
";
        assert_eq!(eval(input).unwrap(), Object::String("abc".to_string()));
    }
}
