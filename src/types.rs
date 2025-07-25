pub type Reader = std::io::BufReader<std::fs::File>;
pub type Writer = std::io::BufWriter<std::fs::File>;

#[derive(Debug, PartialEq)]
pub enum SpecialCharacter {
    Assignment,
    Colon,
    Dot,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Print,
    Println,
    Call,
    Import,
    Export,
    If,
    Else,
    While,
    True,
    False,
    List,
    Int,
    Float,
    Bool,
    Char,
    Break,
    Continue,
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
