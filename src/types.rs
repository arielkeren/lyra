pub type Reader = std::io::BufReader<std::fs::File>;
pub type Writer = std::io::BufWriter<std::fs::File>;

#[derive(Debug, PartialEq)]
pub enum SpecialCharacter {
    Assignment,
    Colon,
    Dot,
    Dash,
    ParanthesisOpen,
    ParanthesisClose,
    SquareBracketOpen,
    SquareBracketClose,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Print,
    Call,
    Import,
    Export,
    Alloc,
    Binary,
    Octal,
    Hex,
    Signed,
    Unsigned,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    SpecialCharacter(SpecialCharacter),
    Literal(String),
}

pub struct Args {
    pub command: String,
    pub executable_name: String,
    pub release: bool,
}
