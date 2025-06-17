#[derive(Debug, PartialEq)]
pub enum Keyword {
    Assignment,
    Print,
    Call,
    Import,
    Colon,
    Export,
    EndFunction,
    Dot,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    Literal(String),
}
