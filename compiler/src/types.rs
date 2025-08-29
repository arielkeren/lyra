pub type Reader = std::io::BufReader<std::fs::File>;
pub type Writer = std::io::BufWriter<std::fs::File>;

pub struct Method {
    pub method: String,
    pub args_str: String,
    pub num_params: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SpecialCharacter {
    Equals,
    ExclamationMark,
    Dot,
    Colon,
    Comma,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    LargerThan,
    SmallerThan,
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Let,
    Const,
    Method,
    Return,
    Import,
    If,
    Else,
    Loop,
    In,
    True,
    False,
    Null,
    Break,
    Continue,
    And,
    Or,
    Not,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Str(String),
    Character(String),
    Number(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    SpecialCharacter(SpecialCharacter),
    Literal(Literal),
}
