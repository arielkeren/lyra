use crate::types::Keyword::*;
use crate::types::SpecialCharacter::*;
use crate::types::Token::*;

pub fn get_tokens(line: &str) -> (Vec<crate::types::Token>, u8) {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();
    let mut spaces = 0;

    while let Some(&ch) = chars.peek() {
        if ch == '\t' {
            if tokens.len() == 0 {
                spaces += 4;
            }
            chars.next();
        } else if ch == ' ' {
            if tokens.len() == 0 {
                spaces += 1;
            }
            chars.next();
        } else if ch.is_whitespace() {
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

    (tokens, spaces / 4)
}

fn is_special_character(ch: char) -> bool {
    get_special_character(ch).is_some()
}

fn get_special_character(ch: char) -> Option<crate::types::SpecialCharacter> {
    match ch {
        '=' => Some(Assignment),
        '.' => Some(Dot),
        ',' => Some(Comma),
        '!' => Some(ExclamationMark),
        '+' => Some(Plus),
        '-' => Some(Minus),
        '*' => Some(Multiply),
        '/' => Some(Divide),
        '%' => Some(Modulo),
        '>' => Some(LargerThan),
        '<' => Some(SmallerThan),
        '(' => Some(OpenParenthesis),
        ')' => Some(CloseParenthesis),
        '[' => Some(OpenBracket),
        ']' => Some(CloseBracket),
        _ => None,
    }
}

fn get_keyword(word: &str) -> Option<crate::types::Keyword> {
    match word {
        "print" => Some(Print),
        "return" => Some(Return),
        "import" => Some(Import),
        "export" => Some(Export),
        "const" => Some(Const),
        "if" => Some(If),
        "else" => Some(Else),
        "while" => Some(While),
        "for" => Some(For),
        "in" => Some(In),
        "true" => Some(True),
        "false" => Some(False),
        "int" => Some(Int),
        "float" => Some(Float),
        "bool" => Some(Bool),
        "char" => Some(Char),
        "break" => Some(Break),
        "continue" => Some(Continue),
        "and" => Some(And),
        "or" => Some(Or),
        "not" => Some(Not),
        _ => None,
    }
}
