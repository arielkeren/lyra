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
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    OpenParenthesis,
    CloseParenthesis,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Print,
    Println,
    Call,
    Import,
    Export,
    True,
    False,
    List,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Char,
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
