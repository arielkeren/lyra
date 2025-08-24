pub type Reader = std::io::BufReader<std::fs::File>;
pub type Writer = std::io::BufWriter<std::fs::File>;

#[derive(Debug, PartialEq, Clone)]
pub enum SpecialCharacter {
    Assignment,
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
    OpenBracket,
    CloseBracket,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Print,
    Return,
    Import,
    Export,
    Const,
    If,
    Else,
    While,
    For,
    In,
    True,
    False,
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
