pub type Reader = std::io::BufReader<std::fs::File>;
pub type Writer = std::io::BufWriter<std::fs::File>;

#[derive(Debug, PartialEq, Clone)]
pub enum SpecialCharacter {
    Assignment,
    Colon,
    Dot,
    Comma,
    ExclamationMark,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LargerThan,
    SmallerThan,
    OpenParenthesis,
    CloseParenthesis,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Print,
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
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
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
