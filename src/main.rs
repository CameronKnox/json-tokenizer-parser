// Simple Json Parser
// By: Cameron Knox
// 7/2/2025

use std::fs;

#[derive(Debug)]
pub enum Token {
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    String(String),
    Number(f64),
    Comma,
    Colon,
    Boolean(bool),
    Null,
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable(); // Allows peeking at the next character

    while let Some(c) = chars.next() {
        match c {
            '{' => tokens.push(Token::BraceOpen),
            '}' => tokens.push(Token::BraceClose),
            '[' => tokens.push(Token::BracketOpen),
            ']' => tokens.push(Token::BracketClose),
            ',' => tokens.push(Token::Comma),
            ':' => tokens.push(Token::Colon),
            '"' => {
                let mut string_content: String = String::new();
                while let Some(&next_character) = chars.peek() {
                    if next_character == '"' {
                        chars.next();
                        break;
                    } else {
                        string_content.push(next_character);
                        chars.next();
                    }
                }
                tokens.push(Token::String(string_content));
            }
            't' | 'f' => {
                let mut bool_value: String = String::new();
                bool_value.push(c);
                for _ in 0..3 {
                    if let Some(next_character) = chars.next() {
                        bool_value.push(next_character);
                    }
                }
                match bool_value.as_str() {
                    "true" => tokens.push(Token::Boolean(true)),
                    "false" => tokens.push(Token::Boolean(false)),
                    _ => panic!("Unexpected token: {}", bool_value),
                }
            }
            'n' => {
                let mut null_value: String = String::new();
                null_value.push(c);
                for _ in 0..3 {
                    if let Some(next_character) = chars.next() {
                        null_value.push(next_character);
                    }
                }
                if null_value == "null" {
                    tokens.push(Token::Null);
                } else {
                    panic!("Unexpected token: {}", null_value);
                }
            }
            c if c.is_digit(10) || c == '-' => {
                let mut number_content: String = String::new();
                number_content.push(c);
                while let Some(&next_character) = chars.peek() {
                    if next_character.is_digit(10) || next_character == '.' {
                        number_content.push(next_character);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let number: f64 = number_content.parse().expect("Invalid number");
                tokens.push(Token::Number(number));
            }
            c if c.is_whitespace() => {
                continue;
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    tokens
}

#[warn(dead_code)]
fn parse() {
    todo!();
}

fn main() {
    let a: String = fs::read_to_string("./foo.json").unwrap();
    let token_output: Vec<Token> = tokenize(a);

    for token in token_output {
        println!("{:?}", token);
    }
}
