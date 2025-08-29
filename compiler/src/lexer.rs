use core::panic;

use crate::types::Keyword::*;
use crate::types::Literal::*;
use crate::types::SpecialCharacter::*;
use crate::types::Token::*;

pub fn get_tokens(line: &str) -> (Vec<crate::types::Token>, u8) {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();
    let mut spaces = 0;

    while let Some(&ch) = chars.peek() {
        match ch {
            '\t' => {
                spaces += 4;
                chars.next();
            }
            ' ' => {
                spaces += 1;
                chars.next();
            }
            _ => break,
        }
    }

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
        } else if ch == '#' {
            break;
        } else if ch == '\'' {
            chars.next();

            match chars.next() {
                Some('\\') => match chars.next() {
                    Some(c) => match get_escape_sequence(c) {
                        Some(escaped) => tokens.push(Literal(Character(escaped))),
                        None => panic!("Invalid escape sequence: \\{}", c),
                    },
                    None => panic!("Unterminated character literal"),
                },
                Some(c) => {
                    if chars.next() == Some('\'') {
                        tokens.push(Literal(Character(c.to_string())));
                    } else {
                        panic!("Unterminated character literal");
                    }
                }
                None => panic!("Unterminated character literal"),
            }
        } else if ch == '"' {
            chars.next();
            let mut literal = String::new();
            let mut is_escaped = false;
            let mut is_terminated = false;

            while let Some(c) = chars.next() {
                match c {
                    '\\' if !is_escaped => is_escaped = true,
                    '"' if !is_escaped => {
                        is_terminated = true;
                        break;
                    }
                    _ => literal.push(c),
                }
            }

            if !is_terminated {
                panic!("Unterminated string literal");
            }

            tokens.push(Literal(Str(literal)));
        } else if let Some(token) = get_special_character(ch) {
            chars.next();
            tokens.push(SpecialCharacter(token));
        } else {
            let mut word = String::new();
            let mut seen_dot = false;

            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    word.push(c);
                    chars.next();
                } else if c == '.'
                    && !seen_dot
                    && !word.is_empty()
                    && word.chars().last().unwrap().is_ascii_digit()
                {
                    seen_dot = true;
                    word.push(c);
                    chars.next();
                } else if c.is_whitespace() || is_special_character(c) || c == '\'' {
                    break;
                } else {
                    word.push(c);
                    chars.next();
                }
            }

            if let Some(keyword) = get_keyword(&word) {
                tokens.push(Keyword(keyword));
            } else if word.parse::<f64>().is_ok() {
                tokens.push(Literal(Number(word)));
            } else if word.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !word.chars().next().unwrap_or('0').is_ascii_digit()
            {
                tokens.push(Identifier(word));
            } else {
                panic!("Unexpected token: {}", word);
            }
        }
    }

    (tokens, spaces / 4)
}

fn is_special_character(ch: char) -> bool {
    get_special_character(ch).is_some()
}

fn get_special_character(ch: char) -> Option<crate::types::SpecialCharacter> {
    match ch {
        '=' => Some(Equals),
        '!' => Some(ExclamationMark),
        '.' => Some(Dot),
        ':' => Some(Colon),
        ',' => Some(Comma),
        '+' => Some(Plus),
        '-' => Some(Minus),
        '*' => Some(Asterisk),
        '/' => Some(Slash),
        '%' => Some(Percent),
        '>' => Some(LargerThan),
        '<' => Some(SmallerThan),
        '(' => Some(OpenParenthesis),
        ')' => Some(CloseParenthesis),
        '[' => Some(OpenBracket),
        ']' => Some(CloseBracket),
        '{' => Some(OpenBrace),
        '}' => Some(CloseBrace),
        _ => None,
    }
}

fn get_keyword(word: &str) -> Option<crate::types::Keyword> {
    match word {
        "let" => Some(Let),
        "const" => Some(Const),
        "method" => Some(Method),
        "return" => Some(Return),
        "import" => Some(Import),
        "if" => Some(If),
        "else" => Some(Else),
        "loop" => Some(Loop),
        "in" => Some(In),
        "true" => Some(True),
        "false" => Some(False),
        "null" => Some(Null),
        "break" => Some(Break),
        "continue" => Some(Continue),
        "and" => Some(And),
        "or" => Some(Or),
        "not" => Some(Not),
        _ => None,
    }
}

fn get_escape_sequence(ch: char) -> Option<String> {
    match ch {
        'n' => Some("\n".to_string()),
        't' => Some("\t".to_string()),
        'r' => Some("\r".to_string()),
        '\\' => Some("\\".to_string()),
        '\'' => Some("'".to_string()),
        '\0' => Some("\0".to_string()),
        _ => None,
    }
}
