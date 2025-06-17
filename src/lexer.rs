use crate::types::Keyword::*;
use crate::types::Token::*;

pub fn get_tokens(line: &str) -> Vec<crate::types::Token> {
    let mut tokens = Vec::new();

    for word in line.split_whitespace() {
        if let Some(keyword) = get_keyword(word) {
            tokens.push(Keyword(keyword));
        } else if word.starts_with('"') && word.ends_with('"') {
            tokens.push(Literal(word.to_string()));
        } else {
            tokens.push(Identifier(word.to_string()));
        }
    }

    tokens
}

fn get_keyword(word: &str) -> Option<crate::types::Keyword> {
    match word {
        "=" => Some(Assignment),
        ":" => Some(Colon),
        "." => Some(Dot),
        "print" => Some(Print),
        "call" => Some(Call),
        "import" => Some(Import),
        "export" => Some(Export),
        "end" => Some(EndFunction),
        _ => None,
    }
}
