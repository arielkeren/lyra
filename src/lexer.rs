use crate::types::Keyword::*;
use crate::types::SpecialCharacter::*;
use crate::types::Token::*;

pub fn get_tokens(line: &str) -> Vec<crate::types::Token> {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
        } else if ch == '#' {
            break;
        } else if ch == '\'' {
            chars.next();
            let mut literal = String::new();
            while let Some(&c) = chars.peek() {
                if c == '\'' {
                    chars.next();
                    break;
                } else {
                    literal.push(c);
                    chars.next();
                }
            }
            tokens.push(Literal(format!("{}", literal)));
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
                tokens.push(Literal(word));
            } else if word.chars().all(|c| c.is_ascii_alphanumeric())
                && !word.chars().next().unwrap_or('0').is_ascii_digit()
            {
                tokens.push(Identifier(word));
            }
        }
    }

    tokens
}

fn is_special_character(ch: char) -> bool {
    get_special_character(ch).is_some()
}

fn get_special_character(ch: char) -> Option<crate::types::SpecialCharacter> {
    match ch {
        '=' => Some(Assignment),
        ':' => Some(Colon),
        '.' => Some(Dot),
        '+' => Some(Plus),
        '-' => Some(Minus),
        '*' => Some(Multiply),
        '/' => Some(Divide),
        '%' => Some(Modulo),
        '&' => Some(BitwiseAnd),
        '|' => Some(BitwiseOr),
        '^' => Some(BitwiseXor),
        '(' => Some(OpenParenthesis),
        ')' => Some(CloseParenthesis),
        _ => None,
    }
}

fn get_keyword(word: &str) -> Option<crate::types::Keyword> {
    match word {
        "print" => Some(Print),
        "println" => Some(Println),
        "call" => Some(Call),
        "import" => Some(Import),
        "export" => Some(Export),
        "true" => Some(True),
        "false" => Some(False),
        "list" => Some(List),
        "i8" => Some(I8),
        "i16" => Some(I16),
        "i32" => Some(I32),
        "i64" => Some(I64),
        "u8" => Some(U8),
        "u16" => Some(U16),
        "u32" => Some(U32),
        "u64" => Some(U64),
        "f32" => Some(F32),
        "f64" => Some(F64),
        "bool" => Some(Bool),
        "char" => Some(Char),
        _ => None,
    }
}
