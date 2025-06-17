#[derive(Debug, PartialEq)]
pub enum SpecialCharacter {
    Assignment,
    Colon,
    Dot,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Print,
    Call,
    Import,
    Export,
    EndFunction,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    SpecialCharacter(SpecialCharacter),
    Literal(String),
}
