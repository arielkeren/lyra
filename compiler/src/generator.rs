use crate::types::Keyword::*;
use crate::types::Literal::*;
use crate::types::SpecialCharacter::*;
use crate::types::Token;
use crate::types::Token::*;

pub fn generate(
    tokens: &Vec<Token>,
    filename: &str,
    after_imports: bool,
    tabs: u8,
    last_tabs: u8,
) -> (String, String, bool) {
    let scope = if tabs < last_tabs {
        (0..last_tabs - tabs)
            .map(|i| {
                let brace_tabs = last_tabs - i - 1;
                if filename != "main.ly" && brace_tabs == 0 {
                    return format!("{}}}\n", "\t".repeat(brace_tabs as usize));
                }
                format!("{}}}", "\t".repeat(brace_tabs as usize))
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    } else {
        "".to_string()
    };

    if let Some(first) = tokens.first() {
        if !after_imports && first != &Keyword(Import) {
            if filename == "main.ly" {
                return (
                    format!(
                        "\nint main() {{\n{}{}",
                        scope,
                        match_c_code(tokens, filename, tabs)
                    ),
                    "".to_string(),
                    true,
                );
            }

            return (
                format!("{}{}", scope, match_c_code(tokens, filename, tabs)),
                match_h_code(tokens),
                true,
            );
        }

        if after_imports && first == &Keyword(Import) {
            panic!("Import statements should be at the beginning of the file");
        }
    }

    (
        format!("{}{}", scope, match_c_code(tokens, filename, tabs)),
        match_h_code(tokens),
        after_imports,
    )
}

fn match_c_code(tokens: &Vec<Token>, mut filename: &str, tabs: u8) -> String {
    filename = filename.trim_end_matches(".ly");

    let code = match tokens.as_slice() {
        [] => "".to_string(),

        [Keyword(Break)] => "break;".to_string(),
        [Keyword(Continue)] => "continue;".to_string(),

        [Keyword(Import), Identifier(file)] => {
            format!("#include \"{file}.hpp\"\n")
        }

        [Keyword(Return)] if filename != "main" => "return Value(nullptr);".to_string(),

        [Keyword(Return), expression @ ..] if filename != "main" => {
            format!("return {};", generate_expression(expression, false))
        }

        [
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            params @ ..,
            SpecialCharacter(CloseParenthesis),
        ] if filename != "main" => {
            format!("Value {function}({}) {{", generate_params(params))
        }

        [
            Identifier(file),
            SpecialCharacter(Dot),
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            args @ ..,
            SpecialCharacter(CloseParenthesis),
        ] => {
            format!("{file}::{function}({});", generate_expression(args, true))
        }

        [
            Keyword(Print),
            SpecialCharacter(OpenParenthesis),
            args @ ..,
            SpecialCharacter(CloseParenthesis),
        ] => format!("print({});", generate_expression(args, true)),

        [
            SpecialCharacter(Plus),
            SpecialCharacter(Plus),
            Identifier(var),
        ] => format!("++{var};"),

        [
            Identifier(var),
            SpecialCharacter(Plus),
            SpecialCharacter(Plus),
        ] => format!("{var}++;"),

        [
            SpecialCharacter(Minus),
            SpecialCharacter(Minus),
            Identifier(var),
        ] => format!("--{var};"),

        [
            Identifier(var),
            SpecialCharacter(Minus),
            SpecialCharacter(Minus),
        ] => format!("{var}--;"),

        [Keyword(Let), Identifier(var)] => format!("Value {var} = Value(nullptr);"),

        [
            Keyword(Let),
            Identifier(var),
            SpecialCharacter(Equals),
            expression @ ..,
        ] => {
            format!("Value {var} = {};", generate_expression(expression, false))
        }

        [
            Keyword(Const),
            Identifier(var),
            SpecialCharacter(Equals),
            expression @ ..,
        ] => {
            format!(
                "const Value {var} = {};",
                generate_expression(expression, false)
            )
        }

        [Identifier(var), SpecialCharacter(Equals), expression @ ..] => {
            format!("{var} = {};", generate_expression(expression, false))
        }

        [Keyword(If), condition @ ..] => {
            format!("if ({}) {{", generate_expression(condition, false))
        }
        [Keyword(Else)] => "else {".to_string(),
        [Keyword(Else), Keyword(If), condition @ ..] => {
            format!("else if ({}) {{", generate_expression(condition, false))
        }

        [Keyword(Loop)] => "while (true) {".to_string(),

        [Keyword(Loop), Identifier(var), Keyword(In), expression @ ..] => {
            format!(
                "for (const Value& {var} : {}) {{",
                generate_expression(expression, false)
            )
        }

        [Keyword(Loop), expression @ ..] => {
            format!("while ({}) {{", generate_expression(expression, false))
        }

        _ => {
            panic!("Unexpected token sequence in file: {filename} - {tokens:?}")
        }
    };

    add_tabs_after_newlines(&code, tabs, filename)
}

fn match_h_code(tokens: &Vec<Token>) -> String {
    match tokens.as_slice() {
        [
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            params @ ..,
            SpecialCharacter(CloseParenthesis),
        ] => format!("Value {function}({});", generate_params(params)),

        _ => "".to_string(),
    }
}

fn add_tabs_after_newlines(code: &str, tabs: u8, filename: &str) -> String {
    let tab_str = "\t".repeat(tabs as usize);
    code.lines()
        .map(|line| {
            if filename == "main" && !line.contains("#include") {
                format!("\t{tab_str}{line}")
            } else {
                format!("{tab_str}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_params(params: &[Token]) -> String {
    let mut param_str = String::new();
    let mut is_comma = false;

    for param in params {
        match param {
            SpecialCharacter(Comma) if is_comma => {
                param_str.push_str(", ");
                is_comma = false;
            }
            Identifier(name) if !is_comma => {
                param_str.push_str(&format!("const Value& {name}"));
                is_comma = true;
            }
            _ => panic!("Unexpected token in function parameters: {:?}", param),
        }
    }

    param_str
}

fn generate_expression(expression: &[Token], are_commas_allowed: bool) -> String {
    if expression.is_empty() {
        panic!("Expression requires at least one token");
    }

    let mut expression_str = String::new();
    let mut tokens = expression.iter().peekable();

    while let Some(token) = tokens.next() {
        if (are_commas_allowed) && token == &SpecialCharacter(Comma) {
            expression_str.push_str(", ");
            continue;
        }

        match token {
            Identifier(id) => {
                expression_str.push_str(id);
            }
            Literal(value) => match value {
                Str(s) => {
                    expression_str.push_str(&format!("Value(\"{s}\")"));
                }
                Character(c) => {
                    expression_str.push_str(&format!("Value('{c}')"));
                }
                Number(n) => {
                    expression_str.push_str(&format!("Value({n})"));
                }
            },
            SpecialCharacter(Equals) => expression_str.push_str("="),
            SpecialCharacter(Dot) => expression_str.push_str("::"),
            SpecialCharacter(Plus) => expression_str.push_str("+"),
            SpecialCharacter(Minus) => expression_str.push_str("-"),
            SpecialCharacter(Asterisk) => expression_str.push_str("*"),
            SpecialCharacter(Slash) => expression_str.push_str("/"),
            SpecialCharacter(Percent) => expression_str.push_str("%"),
            SpecialCharacter(LargerThan) => expression_str.push_str(">"),
            SpecialCharacter(SmallerThan) => expression_str.push_str("<"),
            SpecialCharacter(OpenParenthesis) => expression_str.push_str("("),
            SpecialCharacter(CloseParenthesis) => expression_str.push_str(")"),
            SpecialCharacter(OpenBracket) => expression_str.push_str("["),
            SpecialCharacter(CloseBracket) => expression_str.push_str("]"),
            SpecialCharacter(OpenBrace) => expression_str.push_str("Value(std::vector<Value>{"),
            SpecialCharacter(CloseBrace) => expression_str.push_str("})"),
            Keyword(True) => expression_str.push_str("Value(true)"),
            Keyword(False) => expression_str.push_str("Value(false)"),
            Keyword(Null) => expression_str.push_str("Value(nullptr)"),
            Keyword(And) => expression_str.push_str("&&"),
            Keyword(Or) => expression_str.push_str("||"),
            Keyword(Not) => expression_str.push_str("!"),
            _ => panic!("Unexpected token in expression: {:?}", token),
        }
    }

    expression_str
}
