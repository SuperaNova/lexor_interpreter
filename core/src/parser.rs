//! The LEXOR Pratt-Parsing Engine & Recursive Descent Flow logic.
//!
//! Constructs the multi-dimensional Abstract Syntax Tree safely from the Lexer stream.
//!
//! # Core Responsibilities
//! 1. **Statement Routing:** Employs a Top-Down Recursive Descent technique sequentially structuring layout blocks exactly dynamically.
//! 2. **Expression Ordering:** Utillizes Top-Down Operator Precedence (the Pratt Algorithm) identically routing math correctly without horrific nesting.
//!
//! # Special LEXOR Parsing Rules:
//! - Enforces strict native right-to-left associativity safely for chain-assignments (`x = y = z`).
//! - Guarantees strict sequential consumption of `END IF` and `END REPEAT` delimiters universally prior to routing fallback `ELSE` alternatives.
//! - Resolves `$` sequential string newline concatenations effortlessly identically to standard structural binary math logic.

use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::tokens::Token;

pub const PREC_LOWEST: u8 = 1;
pub const PREC_ASSIGN: u8 = 2; // =
pub const PREC_LOGICAL_OR: u8 = 3; // OR
pub const PREC_LOGICAL_AND: u8 = 4; // AND
pub const PREC_EQUALS: u8 = 5; // ==, <>
pub const PREC_LESSGREATER: u8 = 6; // >, <, >=, <=
pub const PREC_SUM: u8 = 7; // +, -, &
pub const PREC_PRODUCT: u8 = 8; // *, /, %
pub const PREC_PREFIX: u8 = 9; // -X, NOT X, [#]
pub const PREC_CALL: u8 = 10; // () grouping

pub fn get_precedence(token: &Token) -> u8 {
    match token {
        Token::Assign => PREC_ASSIGN,
        Token::Or => PREC_LOGICAL_OR,
        Token::And => PREC_LOGICAL_AND,
        Token::Eq | Token::Neq => PREC_EQUALS,
        Token::Lt | Token::Gt | Token::Lte | Token::Gte => PREC_LESSGREATER,
        Token::Plus | Token::Minus | Token::Concat => PREC_SUM,
        Token::Star | Token::Slash | Token::Modulo => PREC_PRODUCT,
        _ => PREC_LOWEST,
    }
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    pub errors: Vec<String>,
    /// Source line where the current token begins (1-indexed).
    pub current_line: usize,
    /// Source column where the current token begins (1-indexed).
    pub current_col: usize,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next();
        let peek_token = lexer.next();
        let current_line = lexer.line;
        let current_col = lexer.col;

        Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
            current_line,
            current_col,
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.lexer.next();
        // Snapshot the position after the token was consumed
        self.current_line = self.lexer.line;
        self.current_col = self.lexer.col;
    }

    pub fn current_precedence(&self) -> u8 {
        if let Some(token) = &self.current_token {
            get_precedence(token)
        } else {
            PREC_LOWEST
        }
    }

    pub fn peek_precedence(&self) -> u8 {
        if let Some(token) = &self.peek_token {
            get_precedence(token)
        } else {
            PREC_LOWEST
        }
    }

    /// Push an error message prefixed with the current source position.
    fn push_error(&mut self, msg: String) {
        let tagged = format!("[{}:{}] {}", self.current_line, self.current_col, msg);
        self.errors.push(tagged);
    }

    pub fn expect_peek(&mut self, token: Token) -> bool {
        if let Some(t) = &self.peek_token
            && *t == token
        {
            self.next_token();
            return true;
        }
        self.push_error(format!(
            "Expected next token to be {:?}, got {:?}",
            token, self.peek_token
        ));
        false
    }

    // --- PRATT PARSER ENGINE FOR EXPRESSIONS ---

    pub fn parse_expression(&mut self, precedence: u8) -> Option<Expression> {
        let current = self.current_token.clone()?;
        let mut left_exp = self.parse_prefix(&current)?;

        while self.peek_token.is_some() && precedence < self.peek_precedence() {
            let next_token = self.peek_token.clone().unwrap();

            if !self.is_infix(&next_token) {
                return Some(left_exp);
            }

            self.next_token();
            let operator = self.current_token.clone().unwrap();
            left_exp = self.parse_infix(left_exp, operator)?;
        }

        Some(left_exp)
    }

    fn parse_prefix(&mut self, token: &Token) -> Option<Expression> {
        match token {
            Token::Identifier(name) => Some(Expression::Identifier(name.clone())),
            Token::IntLit(val) => Some(Expression::IntLiteral(*val)),
            Token::FloatLit(val) => Some(Expression::FloatLiteral(*val)),
            Token::BoolLit(val) => Some(Expression::BoolLiteral(*val)),
            Token::CharLit(val) => Some(Expression::CharLiteral(*val)),
            Token::StringLit(val) => Some(Expression::StringLiteral(val.clone())),
            Token::Dollar => Some(Expression::StringLiteral("\n".to_string())),

            Token::Minus | Token::Plus | Token::Not => {
                let operator = token.clone();
                self.next_token(); // consume operator
                let right = self.parse_expression(PREC_PREFIX)?;
                Some(Expression::Prefix {
                    operator,
                    right: Box::new(right),
                })
            }

            Token::LParen => {
                self.next_token(); // consume '('
                let exp = self.parse_expression(PREC_LOWEST)?;

                if self.expect_peek(Token::RParen) {
                    Some(exp)
                } else {
                    None
                }
            }

            _ => {
                self.errors
                    .push(format!("No prefix parse function for {:?}", token));
                None
            }
        }
    }

    fn is_infix(&self, token: &Token) -> bool {
        matches!(
            token,
            Token::Plus
                | Token::Minus
                | Token::Star
                | Token::Slash
                | Token::Modulo
                | Token::Eq
                | Token::Neq
                | Token::Lt
                | Token::Gt
                | Token::Lte
                | Token::Gte
                | Token::And
                | Token::Or
                | Token::Concat
                | Token::Assign
        )
    }

    fn parse_infix(&mut self, left: Expression, operator: Token) -> Option<Expression> {
        let precedence = self.current_precedence();
        self.next_token(); // move past operator

        let right = if operator == Token::Assign {
            // Right-associative: we parse the right side with slightly lower precedence
            // so we keep grabbing deeper assignments into the right arm.
            self.parse_expression(precedence - 1)?
        } else {
            // Left-associative: pass current precedence
            self.parse_expression(precedence)?
        };

        Some(Expression::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    // --- STATEMENT PARSING ---

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };

        self.skip_newlines();

        // We expect `SCRIPT AREA` initially
        if self.current_token != Some(Token::Script) {
            self.errors.push(format!(
                "Expected SCRIPT AREA, got {:?}",
                self.current_token
            ));
            return None;
        }
        if !self.expect_peek(Token::Area) {
            return None;
        }
        self.next_token(); // consume Area
        self.skip_newlines();

        // We expect `START SCRIPT`
        if self.current_token != Some(Token::Start) {
            self.errors.push(format!(
                "Expected START SCRIPT, got {:?}",
                self.current_token
            ));
            return None;
        }
        if !self.expect_peek(Token::Script) {
            return None;
        }
        self.next_token(); // consume Script
        self.skip_newlines();

        // Parse body until END SCRIPT
        loop {
            // Skip blank lines
            if let Some(Token::Newline) = self.current_token {
                self.next_token();
                continue;
            }

            // Reached END SCRIPT — consume both tokens and stop
            if let (Some(Token::End), Some(Token::Script)) = (&self.current_token, &self.peek_token)
            {
                self.next_token(); // consume END
                self.next_token(); // consume SCRIPT
                break;
            }

            // EOF — stop
            if self.current_token.is_none() {
                break;
            }

            // Detect a second SCRIPT AREA or START SCRIPT — hard error
            if let (Some(Token::Script), Some(Token::Area)) =
                (&self.current_token, &self.peek_token)
            {
                self.push_error("Only one SCRIPT AREA block is allowed per program.".to_string());
                return Some(program);
            }
            if let (Some(Token::Start), Some(Token::Script)) =
                (&self.current_token, &self.peek_token)
            {
                self.push_error(
                    "Unexpected START SCRIPT: only one script block is allowed per program."
                        .to_string(),
                );
                return Some(program);
            }

            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
                self.expect_statement_end();
            } else {
                self.next_token(); // skip token to avoid infinite loop on error
            }
        }

        // After END SCRIPT, nothing else is allowed in the file.
        self.skip_newlines();
        if self.current_token.is_some() {
            self.push_error(format!(
                "Unexpected content after END SCRIPT: {:?}. Only one SCRIPT AREA block is allowed.",
                self.current_token
            ));
        }

        Some(program)
    }

    fn skip_newlines(&mut self) {
        while let Some(Token::Newline) = self.current_token {
            self.next_token();
        }
    }

    /// Enforces the one-statement-per-line rule.
    /// After every parsed statement, the current token must be a `Newline`, `EOF`, or an
    /// upcoming block terminator (`END`). Anything else is a parse error; the parser then
    /// skips forward to the next newline or block terminator so the block structure is preserved.
    fn expect_statement_end(&mut self) {
        match &self.current_token {
            Some(Token::Newline) => {
                self.next_token(); // consume the newline
            }
            None | Some(Token::End) => {} // EOF or upcoming END — fine, don't consume
            _ => {
                // Extract the inner token for a clean error message (e.g. "Declare" not "Some(Declare)").
                let got = self
                    .current_token
                    .as_ref()
                    .map(|t| format!("{:?}", t))
                    .unwrap_or_else(|| "EOF".to_string());
                self.push_error(format!(
                    "Expected newline after statement, got {got}. Each statement must be on its own line."
                ));
                // Skip ahead, but stop at newlines, EOF, or block terminators (END) so
                // the parser never accidentally consumes `END IF` / `END FOR` / etc.
                while !matches!(
                    self.current_token,
                    Some(Token::Newline) | Some(Token::End) | None
                ) {
                    self.next_token();
                }
                if matches!(self.current_token, Some(Token::Newline)) {
                    self.next_token();
                }
            }
        }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Some(Token::Declare) => self.parse_declare_statement(),
            Some(Token::Print) => self.parse_print_statement(),
            Some(Token::Scan) => self.parse_scan_statement(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::For) => self.parse_for_statement(),
            Some(Token::Repeat) => self.parse_repeat_when_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_declare_statement(&mut self) -> Option<Statement> {
        self.next_token(); // consume DECLARE
        let type_token = self.current_token.clone()?;
        match type_token {
            Token::TypeInt | Token::TypeFloat | Token::TypeBool | Token::TypeChar => {
                self.next_token();
            }
            _ => {
                self.errors
                    .push(format!("Expected type after DECLARE, got {:?}", type_token));
                return None;
            }
        }

        let mut declarations = Vec::new();

        loop {
            let var_name = if let Some(Token::Identifier(name)) = &self.current_token {
                name.clone()
            } else {
                self.errors
                    .push("Expected identifier in DECLARE".to_string());
                return None;
            };

            self.next_token(); // consume identifier

            let mut init_expr = None;
            if let Some(Token::Assign) = self.current_token {
                self.next_token(); // consume '='
                init_expr = self.parse_expression(PREC_LOWEST);
                // step past the expression tokens
                self.next_token();
            }

            declarations.push((var_name, init_expr));

            if let Some(Token::Comma) = self.current_token {
                self.next_token(); // consume comma
            } else {
                break;
            }
        }

        Some(Statement::Declare(type_token, declarations))
    }

    fn parse_print_statement(&mut self) -> Option<Statement> {
        self.next_token(); // consume PRINT

        if let Some(Token::Colon) = self.current_token {
            self.next_token(); // consume Colon
        } else {
            self.errors.push("Expected ':' after PRINT".to_string());
            return None;
        }

        let expr = self.parse_expression(PREC_LOWEST)?;
        self.next_token(); // step past expression

        Some(Statement::Print(expr))
    }

    fn parse_scan_statement(&mut self) -> Option<Statement> {
        self.next_token(); // consume SCAN

        if let Some(Token::Colon) = self.current_token {
            self.next_token(); // consume Colon
        } else {
            self.errors.push("Expected ':' after SCAN".to_string());
            return None;
        }

        let mut vars = Vec::new();
        loop {
            if let Some(Token::Identifier(name)) = &self.current_token {
                vars.push(name.clone());
                self.next_token();
            } else {
                self.errors.push("Expected identifier in SCAN".to_string());
                return None;
            }

            if let Some(Token::Comma) = self.current_token {
                self.next_token(); // consume Comma
            } else {
                break;
            }
        }

        Some(Statement::Scan(vars))
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(Token::LParen) {
            return None;
        }

        let condition = self.parse_expression(PREC_LOWEST)?;
        self.next_token(); // step past `)`
        self.skip_newlines();

        if self.current_token != Some(Token::Start) {
            self.errors
                .push("Expected START after IF (...)".to_string());
            return None;
        }
        self.next_token(); // consume Start
        if self.current_token != Some(Token::If) {
            return None;
        }
        self.next_token(); // consume If
        self.skip_newlines();

        let mut consequence = Vec::new();
        while self.current_token.is_some() && self.current_token != Some(Token::End) {
            if let Some(Token::Newline) = self.current_token {
                self.next_token();
                continue;
            }
            if let Some(stmt) = self.parse_statement() {
                consequence.push(stmt);
                self.expect_statement_end();
            } else {
                self.next_token();
            }
        }

        if self.current_token != Some(Token::End) {
            return None;
        }
        self.next_token(); // consume END
        if self.current_token != Some(Token::If) {
            return None;
        }
        self.next_token(); // consume IF

        let mut alternative = None;
        self.skip_newlines();

        if let Some(Token::Else) = self.current_token {
            self.next_token(); // consume ELSE

            if let Some(Token::If) = self.current_token {
                if let Some(else_if_stmt) = self.parse_if_statement() {
                    alternative = Some(vec![else_if_stmt]);
                }
            } else {
                self.skip_newlines();
                if self.current_token != Some(Token::Start) {
                    self.errors.push("Expected START after ELSE".to_string());
                    return None;
                }
                self.next_token(); // consume Start
                if self.current_token != Some(Token::If) {
                    return None;
                }
                self.next_token(); // consume If
                self.skip_newlines();

                let mut alt_stmts = Vec::new();
                while self.current_token.is_some() && self.current_token != Some(Token::End) {
                    if let Some(Token::Newline) = self.current_token {
                        self.next_token();
                        continue;
                    }
                    if let Some(stmt) = self.parse_statement() {
                        alt_stmts.push(stmt);
                        self.expect_statement_end();
                    } else {
                        self.next_token();
                    }
                }
                alternative = Some(alt_stmts);

                if self.current_token != Some(Token::End) {
                    return None;
                }
                self.next_token(); // consume END
                if self.current_token != Some(Token::If) {
                    return None;
                }
                self.next_token(); // consume If
            }
        }

        Some(Statement::If {
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_for_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(Token::LParen) {
            return None;
        }
        self.next_token(); // step firmly cleanly past the `(` so expression inside handles it

        let initialization = Box::new(self.parse_statement()?);
        if self.current_token != Some(Token::Comma) {
            self.errors
                .push("Expected ',' after initialization in FOR".to_string());
            return None;
        }
        self.next_token(); // consume comma

        let condition = self.parse_expression(PREC_LOWEST)?;
        self.next_token(); // step identically past condition 
        if self.current_token != Some(Token::Comma) {
            self.errors.push(format!(
                "Expected ',' after condition in FOR, got {:?}",
                self.current_token
            ));
            return None;
        }
        self.next_token(); // consume comma

        let update = Box::new(self.parse_statement()?);
        if self.current_token != Some(Token::RParen) {
            self.errors.push(format!(
                "Expected ')' to close FOR, got {:?}",
                self.current_token
            ));
            return None;
        }
        self.next_token(); // consume RParen
        self.skip_newlines();

        if self.current_token != Some(Token::Start) {
            return None;
        }
        self.next_token(); // consume START
        if self.current_token != Some(Token::For) {
            return None;
        }
        self.next_token(); // consume FOR
        self.skip_newlines();

        let mut body = Vec::new();
        while self.current_token.is_some() && self.current_token != Some(Token::End) {
            if let Some(Token::Newline) = self.current_token {
                self.next_token();
                continue;
            }
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
                self.expect_statement_end();
            } else {
                self.next_token();
            }
        }

        if self.current_token != Some(Token::End) {
            return None;
        }
        self.next_token();
        if self.current_token != Some(Token::For) {
            return None;
        }
        self.next_token();

        Some(Statement::For {
            initialization,
            condition,
            update,
            body,
        })
    }

    fn parse_repeat_when_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(Token::When) {
            return None;
        }
        if !self.expect_peek(Token::LParen) {
            return None;
        }

        let condition = self.parse_expression(PREC_LOWEST)?;
        self.next_token(); // cleanly step past `)`

        self.skip_newlines();

        if self.current_token != Some(Token::Start) {
            return None;
        }
        self.next_token(); // consume START
        if self.current_token != Some(Token::Repeat) {
            return None;
        }
        self.next_token(); // consume REPEAT
        self.skip_newlines();

        let mut body = Vec::new();
        while self.current_token.is_some() && self.current_token != Some(Token::End) {
            if let Some(Token::Newline) = self.current_token {
                self.next_token();
                continue;
            }
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
                self.expect_statement_end();
            } else {
                self.next_token();
            }
        }

        if self.current_token != Some(Token::End) {
            return None;
        }
        self.next_token();
        if self.current_token != Some(Token::Repeat) {
            return None;
        }
        self.next_token();

        Some(Statement::RepeatWhen { condition, body })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(PREC_LOWEST)?;
        self.next_token(); // Step past expression
        Some(Statement::Expression(expr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(input: &str) -> Expression {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let expr = parser
            .parse_expression(PREC_LOWEST)
            .expect("Expected an expression");
        assert_eq!(parser.errors.len(), 0, "Parser errors: {:?}", parser.errors);
        expr
    }

    #[test]
    fn test_prefix_expressions() {
        let expr = parse("-15");
        assert_eq!(
            expr,
            Expression::Prefix {
                operator: Token::Minus,
                right: Box::new(Expression::IntLiteral(15))
            }
        );
    }

    #[test]
    fn test_infix_expressions() {
        // 5 + 5
        let expr = parse("5 + 5");
        assert_eq!(
            expr,
            Expression::Infix {
                left: Box::new(Expression::IntLiteral(5)),
                operator: Token::Plus,
                right: Box::new(Expression::IntLiteral(5))
            }
        );
    }

    #[test]
    fn test_operator_precedence_parsing() {
        // -a * b
        let expr = parse("-a * b");
        assert_eq!(
            expr,
            Expression::Infix {
                left: Box::new(Expression::Prefix {
                    operator: Token::Minus,
                    right: Box::new(Expression::Identifier("a".to_string()))
                }),
                operator: Token::Star,
                right: Box::new(Expression::Identifier("b".to_string()))
            }
        );

        // a + b * c evaluates to (a + (b * c))
        let expr = parse("a + b * c");
        assert_eq!(
            expr,
            Expression::Infix {
                left: Box::new(Expression::Identifier("a".to_string())),
                operator: Token::Plus,
                right: Box::new(Expression::Infix {
                    left: Box::new(Expression::Identifier("b".to_string())),
                    operator: Token::Star,
                    right: Box::new(Expression::Identifier("c".to_string()))
                })
            }
        );
    }

    #[test]
    fn test_right_associative_assignment() {
        // x = y = 4 evaluates to (x = (y = 4))
        let expr = parse("x = y = 4");
        assert_eq!(
            expr,
            Expression::Infix {
                left: Box::new(Expression::Identifier("x".to_string())),
                operator: Token::Assign,
                right: Box::new(Expression::Infix {
                    left: Box::new(Expression::Identifier("y".to_string())),
                    operator: Token::Assign,
                    right: Box::new(Expression::IntLiteral(4))
                })
            }
        );
    }

    // --- One-Statement-Per-Line Enforcement ---

    fn parse_program_errors(input: &str) -> Vec<String> {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program();
        parser.errors
    }

    #[test]
    fn test_valid_program_has_no_errors() {
        let input = "SCRIPT AREA\nSTART SCRIPT\n    DECLARE INT x = 5\n    DECLARE INT y = 10\n    x = x + y\nEND SCRIPT\n";
        let errors = parse_program_errors(input);
        assert!(errors.is_empty(), "Expected no errors, got: {:?}", errors);
    }

    #[test]
    fn test_two_statements_on_one_line_is_error() {
        let input =
            "SCRIPT AREA\nSTART SCRIPT\n    DECLARE INT x = 5 DECLARE INT y = 10\nEND SCRIPT\n";
        let errors = parse_program_errors(input);
        assert!(
            !errors.is_empty(),
            "Expected a parse error for two statements on one line"
        );
        assert!(
            errors.iter().any(|e| e.contains("Expected newline")),
            "Expected an 'Expected newline' error, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_two_statements_on_one_line_in_if_body_is_error() {
        let input = "SCRIPT AREA\nSTART SCRIPT\n    DECLARE INT x = 1\n    IF (x == 1)\n    START IF\n        x = 2 x = 3\n    END IF\nEND SCRIPT\n";
        let errors = parse_program_errors(input);
        assert!(
            !errors.is_empty(),
            "Expected a parse error for two statements on one line inside IF body"
        );
        assert!(
            errors.iter().any(|e| e.contains("Expected newline")),
            "Expected an 'Expected newline' error, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_two_statements_on_one_line_in_else_body_is_error() {
        let input = "SCRIPT AREA\nSTART SCRIPT\n    DECLARE INT x = 0\n    IF (x == 1)\n    START IF\n        x = 1\n    END IF\n    ELSE\n    START IF\n        x = 10 x = 20\n    END IF\nEND SCRIPT\n";
        let errors = parse_program_errors(input);
        assert!(
            !errors.is_empty(),
            "Expected a parse error for two statements on one line inside ELSE body"
        );
        assert!(
            errors.iter().any(|e| e.contains("Expected newline")),
            "Expected an 'Expected newline' error, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_two_statements_on_one_line_in_for_body_is_error() {
        let input = "SCRIPT AREA\nSTART SCRIPT\n    DECLARE INT i = 0\n    DECLARE INT x = 0\n    FOR (i = 1, i <= 3, i = i + 1)\n    START FOR\n        x = x + 1 x = x + 1\n    END FOR\nEND SCRIPT\n";
        let errors = parse_program_errors(input);
        assert!(
            !errors.is_empty(),
            "Expected a parse error for two statements on one line inside FOR body"
        );
        assert!(
            errors.iter().any(|e| e.contains("Expected newline")),
            "Expected an 'Expected newline' error, got: {:?}",
            errors
        );
    }

    #[test]
    fn test_two_statements_on_one_line_in_repeat_body_is_error() {
        let input = "SCRIPT AREA\nSTART SCRIPT\n    DECLARE INT count = 0\n    DECLARE INT x = 0\n    REPEAT WHEN (count < 3)\n    START REPEAT\n        count = count + 1 x = x + 1\n    END REPEAT\nEND SCRIPT\n";
        let errors = parse_program_errors(input);
        assert!(
            !errors.is_empty(),
            "Expected a parse error for two statements on one line inside REPEAT WHEN body"
        );
        assert!(
            errors.iter().any(|e| e.contains("Expected newline")),
            "Expected an 'Expected newline' error, got: {:?}",
            errors
        );
    }
}
