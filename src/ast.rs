//! The LEXOR Abstract Syntax Tree (AST).
//!
//! This module contains the strictly typed Enums and Structs that securely represent the 
//! nested hierarchical structure of parsed code. The Parser functionally constructs these nodes, 
//! and the Evaluator naturally consumes them.

use crate::tokens::Token;

/// The overall program containing a sequence of statements.
#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// A node that executes an action but does not produce a value.
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// e.g., DECLARE INT x, y, z = 5
    /// Stores the base type token (INT, FLOAT, etc) and a list of variables with optional initialization expressions.
    Declare(Token, Vec<(String, Option<Expression>)>),

    /// Any standalone expression, notably including Assignments (e.g. `x = y = 4`) since they are Right-Associative expressions.
    Expression(Expression),

    /// e.g., PRINT: expr1 & expr2 & ...
    Print(Expression),

    /// e.g., SCAN: var1, var2
    Scan(Vec<String>),

    /// IF (<condition>) START IF ... END IF [ELSE IF ...] [ELSE...]
    If {
        condition: Expression,
        consequence: Vec<Statement>,
        /// If there's an ELSE IF, it can simply be a Vec containing a single `Statement::If` wrapped inside this alternative.
        alternative: Option<Vec<Statement>>,
    },

    /// FOR (init, condition, update) START FOR ... END FOR
    For {
        initialization: Box<Statement>,
        condition: Expression,
        update: Box<Statement>,
        body: Vec<Statement>,
    },

    /// REPEAT WHEN (<condition>) START REPEAT ... END REPEAT
    RepeatWhen {
        condition: Expression,
        body: Vec<Statement>,
    },
}

/// A node that evaluates and produces a value.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    IntLiteral(i32),
    FloatLiteral(f32),
    BoolLiteral(bool),
    CharLiteral(char),
    StringLiteral(String),

    /// Prefix expressions like -5, +5, NOT true, [#]
    Prefix {
        operator: Token,
        right: Box<Expression>,
    },

    /// Infix expressions like 5 + 5, x == y, a AND b, a = b, string & string
    Infix {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
}
