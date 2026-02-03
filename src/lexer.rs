/*
 * * The Lexer (Tokenizer) is the first step of the interpreter.
 * It takes the raw source code text (e.g., "DECLARE INT x") and breaks it into tokens.
 * * Its main jobs are:
 * 1. Read the text character by character.
 * 2. Group characters into words (Keywords or Identifiers).
 * 3. Recognize numbers (Ints vs Floats).
 * 4. Handle special LEXOR rules:
 * - Ignore comments starting with %%
 * - Treat $ as the "End of Line" token (not just whitespace).
 * - Handle escape codes inside [ ] brackets.
 */

use crate::tokens::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    // the input is our iterator over the characters of the source code.
    // Peekable so we can look ahead at the next character without losing information.
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    // basically breaks down a string and converts it to a struct of a char list iterator that is peekable yes
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    // peek next char, if at the end return null char
    fn peek_char(&mut self) -> char {
        *self.input.peek().unwrap_or(&'\0')
    }

    // skip space and tabs only
    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch == ' ' || ch == '\t' {
                self.input.next();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        // skip whitespace
        self.skip_whitespace();

        // read the next character. if None, we are at End of File.
        let ch = self.input.next()?;

        match ch {
            // Terminations & Print Symbols
            '\n' => Some(Token::Newline),
            '\r' => {
                // windows \r\n
                if self.peek_char() == '\n' {
                    self.input.next();
                }
                Some(Token::Newline)
            }
            '$' => Some(Token::Dollar), // Used in PRINT

            // Simple Operators
            ',' => Some(Token::Comma),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            '&' => Some(Token::Concat),

            // Operators that need lookahead

            // % -> modulo or comment %%
            '%' => {
                if self.peek_char() == '%' {
                    self.consume_comment();
                    self.next_token() // Recursively get the next real token
                } else {
                    Some(Token::Modulo)
                }
            }

            // = -> assign or equal ==
            '=' => {
                if self.peek_char() == '=' {
                    self.input.next();
                    Some(Token::Eq)
                } else {
                    Some(Token::Assign)
                }
            }

            // < -> less than, less than or equal to <=, or not equal to <>
            '<' => match self.peek_char() {
                '=' => {
                    self.input.next();
                    Some(Token::Lte)
                }
                '>' => {
                    self.input.next();
                    Some(Token::Neq)
                }
                _ => Some(Token::Lt),
            },

            // > -> greater than or greater than or equal to >=
            '>' => {
                if self.peek_char() == '=' {
                    self.input.next();
                    Some(Token::Gte)
                } else {
                    Some(Token::Gt)
                }
            }

            // Complex Tokens (Handled by Helpers)
            '[' => self.read_escape_sequence(),
            '"' => self.read_string(),
            '\'' => self.read_char_literal(),

            // Words (Keywords or Variables)
            c if c.is_alphabetic() || c == '_' => Some(self.read_identifier(c)),

            // Numbers (Int or Float)
            c if c.is_ascii_digit() => Some(self.read_number(c)),

            // Unknown Character
            _ => Some(Token::Illegal(ch.to_string())),
        }
    }

    // Helper functions
    // Eat comments starting with %% until the end of line
    fn consume_comment(&mut self) {
        self.input.next(); // Eat second %
        while let Some(&ch) = self.input.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            self.input.next();
        }
    }

    // Handle [] escape codes
    fn read_escape_sequence(&mut self) -> Option<Token> {
        let mut content = String::new();
        while let Some(ch) = self.input.next() {
            if ch == ']' {
                break;
            }
            content.push(ch);
        }
        match content.as_str() {
            "#" => Some(Token::CharLit('#')), // [#]
            "[" => Some(Token::CharLit('[')), // [[]
            "]" => Some(Token::CharLit(']')), // []]
            _ => Some(Token::Illegal(format!("[{}]", content))),
        }
    }

    // Handle Words: Check against ALL CAPS keywords
    fn read_identifier(&mut self, first: char) -> Token {
        let mut ident = String::new();
        ident.push(first);
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        match ident.as_str() {
            "DECLARE" => Token::Declare,
            "PRINT" => Token::Print,
            "SCAN" => Token::Scan,
            "IF" => Token::If,
            "ELSE" => Token::Else,
            "FOR" => Token::For,
            "REPEAT" => Token::Repeat,
            "WHEN" => Token::When,
            "START" => Token::Start,
            "END" => Token::End,
            "SCRIPT" => Token::Script,
            "AREA" => Token::Area,
            "INT" => Token::TypeInt,
            "CHAR" => Token::TypeChar,
            "BOOL" => Token::TypeBool,
            "FLOAT" => Token::TypeFloat,
            "AND" => Token::And,
            "OR" => Token::Or,
            "NOT" => Token::Not,
            "TRUE" => Token::BoolLit(true),
            "FALSE" => Token::BoolLit(false),
            _ => Token::Identifier(ident),
        }
    }

    // check if int or float
    fn read_number(&mut self, first: char) -> Token {
        let mut num_str = String::new();
        num_str.push(first);
        let mut has_decimal = false;

        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_digit() {
                num_str.push(self.input.next().unwrap());
            } else if ch == '.' {
                if has_decimal {
                    break;
                }
                has_decimal = true;
                num_str.push(self.input.next().unwrap());
            } else {
                break;
            }
        }

        if has_decimal {
            Token::FloatLit(num_str.parse::<f32>().unwrap_or(0.0))
        } else {
            Token::IntLit(num_str.parse::<i32>().unwrap_or(0))
        }
    }

    // Handle "strings"
    fn read_string(&mut self) -> Option<Token> {
        let mut s = String::new();
        while let Some(ch) = self.input.next() {
            if ch == '"' {
                break;
            }
            s.push(ch);
        }
        Some(Token::StringLit(s))
    }

    // Handle 'char'
    fn read_char_literal(&mut self) -> Option<Token> {
        let ch = self.input.next()?;
        if self.peek_char() == '\'' {
            self.input.next();
        }
        Some(Token::CharLit(ch))
    }
}
