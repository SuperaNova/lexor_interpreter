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

    // TODO: MAIN NEXT TOKEN FUNCTION
    pub fn next_token(&mut self) -> Option<Token> {
        // skip whitespace
        self.skip_whitespace();

        // read the next character. if None, we are at End of File.
        let ch = self.input.next()?;

        match ch {
            // TODO: Make all the match cases for character handling
            // Unknown Character
            _ => Some(Token::Illegal(ch.to_string())),
        }
    }

    // TODO: add helper functions like reading the whatever it is
}
