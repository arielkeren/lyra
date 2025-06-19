pub type Reader = std::io::BufReader<std::fs::File>;
pub type Writer = std::io::BufWriter<std::fs::File>;

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
    Alloc,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    SpecialCharacter(SpecialCharacter),
    Literal(String),
}
