use crate::types::Keyword::*;
use crate::types::Literal::*;
use crate::types::Method;
use crate::types::SpecialCharacter;
use crate::types::SpecialCharacter::*;
use crate::types::Token;
use crate::types::Token::*;

pub fn generate_imports(tokens: &[Token]) -> Option<String> {
    if let [Keyword(Import), Identifier(file)] = tokens {
        if file == "std" || file == "main" {
            panic!("Cannot import reserved module name: {}", file);
        }
        Some(format!("#include \"{file}.hpp\""))
    } else {
        None
    }
}

pub fn generate(
    tokens: &Vec<Token>,
    filename: &str,
    tabs: u8,
    last_tabs: u8,
    methods: &mut Vec<Method>,
) -> (String, String) {
    let scope = if tabs < last_tabs {
        (0..last_tabs - tabs)
            .map(|i| {
                let brace_tabs = last_tabs - i - 1;

                let closing_brace = if filename != "main.ly" && brace_tabs == 0 {
                    "\treturn Value(nullptr);\n}".to_string()
                } else {
                    "}".to_string()
                };

                format!("{}{}", "\t".repeat(brace_tabs as usize), closing_brace)
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    } else {
        "".to_string()
    };

    if filename != "main" && tabs == 0 {
        if let [
            Keyword(Method),
            Identifier(method),
            SpecialCharacter(OpenParenthesis),
            params @ ..,
            SpecialCharacter(CloseParenthesis),
        ] = tokens.as_slice()
        {
            let params_str = generate_params(params);
            let num_params = if params_str.is_empty() {
                0
            } else {
                params_str.matches(',').count() + 1
            };
            let args_str = generate_args(num_params);

            methods.push(Method {
                method: method.to_string(),
                num_params,
                args_str,
            });
        }
    }

    (
        format!("{}{}", scope, match_c_code(tokens, filename, tabs)),
        if filename == "main.ly" || tabs > 0 {
            "".to_string()
        } else {
            match_h_code(tokens)
        },
    )
}

fn match_c_code(tokens: &Vec<Token>, mut filename: &str, tabs: u8) -> String {
    filename = filename.trim_end_matches(".ly");

    let code = match tokens.as_slice() {
        [] => "".to_string(),

        [Keyword(Break)] => "break;".to_string(),
        [Keyword(Continue)] => "continue;".to_string(),

        [Keyword(Return), expression @ ..] if filename != "main" => {
            let return_value = if expression.is_empty() {
                "Value(nullptr)".to_string()
            } else {
                generate_expression(expression)
            };

            format!("return {};", return_value)
        }

        [
            Keyword(Method),
            Identifier(method),
            SpecialCharacter(OpenParenthesis),
            params @ ..,
            SpecialCharacter(CloseParenthesis),
        ] if filename != "main" && tabs == 0 => {
            format!("Value {filename}_{method}({}) {{", generate_params(params))
        }

        [
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            params @ ..,
            SpecialCharacter(CloseParenthesis),
        ] if filename != "main" && tabs == 0 => {
            format!(
                "Value {filename}::{function}({}) {{",
                generate_params(params)
            )
        }

        [
            Identifier(file),
            SpecialCharacter(Colon),
            SpecialCharacter(Colon),
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            args @ ..,
            SpecialCharacter(CloseParenthesis),
        ] => {
            let arguments = if args.is_empty() {
                "".to_string()
            } else {
                generate_expression(args)
            };

            format!("{file}::{function}({});", arguments)
        }

        [
            Identifier(object),
            SpecialCharacter(Dot),
            Identifier(method),
            SpecialCharacter(OpenParenthesis),
            args @ ..,
            SpecialCharacter(CloseParenthesis),
        ] => {
            let arguments = if args.is_empty() {
                "".to_string()
            } else {
                generate_expression(args)
            };

            format!("{object}[\"{method}\"]({});", arguments)
        }

        [
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            args @ ..,
            SpecialCharacter(CloseParenthesis),
        ] if function == "print" || function == "exit" => {
            let arguments = if args.is_empty() {
                "".to_string()
            } else {
                generate_expression(args)
            };

            format!("_{function}({arguments});")
        }

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

        [Keyword(Let), Identifier(var)] if filename != "main" && tabs == 0 => {
            format!("Value {filename}::{var}(nullptr);")
        }

        [Keyword(Let), Identifier(var)] => format!("Value {var}(nullptr);"),

        [
            Keyword(Let),
            Identifier(var),
            SpecialCharacter(Equals),
            expression @ ..,
        ] if filename != "main" && tabs == 0 => {
            format!(
                "Value {filename}::{var}({});",
                generate_expression(expression)
            )
        }

        [
            Keyword(Let),
            Identifier(var),
            SpecialCharacter(Equals),
            expression @ ..,
        ] => {
            format!("Value {var}({});", generate_expression(expression))
        }

        [
            Keyword(Const),
            Identifier(var),
            SpecialCharacter(Equals),
            expression @ ..,
        ] if filename != "main" && tabs == 0 => {
            format!(
                "const Value {filename}::{var}({});",
                generate_expression(expression)
            )
        }

        [
            Keyword(Const),
            Identifier(var),
            SpecialCharacter(Equals),
            expression @ ..,
        ] => {
            format!("const Value {var}({});", generate_expression(expression))
        }

        [Identifier(var), SpecialCharacter(Equals), expression @ ..] => {
            format!("{var} = {};", generate_expression(expression))
        }

        [
            Identifier(var),
            SpecialCharacter(operation),
            SpecialCharacter(Equals),
            expression @ ..,
        ] if [Plus, Minus, Asterisk, Slash].contains(operation) => {
            format!(
                "{var} {}= {};",
                to_operation_sign(operation),
                generate_expression(expression)
            )
        }

        [
            Identifier(file),
            SpecialCharacter(Colon),
            SpecialCharacter(Colon),
            Identifier(var),
            SpecialCharacter(Equals),
            expression @ ..,
        ] => {
            format!("{file}::{var} = {};", generate_expression(expression))
        }

        [Identifier(var), SpecialCharacter(OpenBracket)] => {
            panic!("Missing index and assignment in array access for variable: {var}")
        }
        [Identifier(var), SpecialCharacter(OpenBracket), rest @ ..] => {
            // Find the position of CloseBracket and Equals
            let close_bracket_pos = rest
                .iter()
                .position(|t| t == &SpecialCharacter(CloseBracket));
            let equals_pos = rest.iter().position(|t| t == &SpecialCharacter(Equals));
            if let (Some(cb), Some(eq)) = (close_bracket_pos, equals_pos) {
                if eq > cb {
                    let index = &rest[..cb];
                    let expression = &rest[eq + 1..];
                    format!(
                        "{var}[{}] = {};",
                        generate_expression(index),
                        generate_expression(expression)
                    )
                } else {
                    panic!(
                        "Equals sign must come after closing bracket in array assignment for variable: {var}"
                    )
                }
            } else {
                panic!("Malformed array assignment for variable: {var}")
            }
        }

        [Keyword(If), condition @ ..] => {
            format!("if ({}) {{", generate_expression(condition))
        }
        [Keyword(Else)] => "else {".to_string(),
        [Keyword(Else), Keyword(If), condition @ ..] => {
            format!("else if ({}) {{", generate_expression(condition))
        }

        [Keyword(Loop)] => "while (true) {".to_string(),

        [Keyword(Loop), Identifier(var), Keyword(In), expression @ ..] => {
            format!(
                "for (const Value& {var} : {}) {{",
                generate_iterator(expression)
            )
        }

        [Keyword(Loop), expression @ ..] => {
            format!("while ({}) {{", generate_expression(expression))
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

        [Keyword(Let), Identifier(var), ..] => {
            format!("extern Value {var};")
        }

        [Keyword(Const), Identifier(var), ..] => {
            format!("extern const Value {var};")
        }

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

fn to_operation_sign(operation: &SpecialCharacter) -> &str {
    match operation {
        Plus => "+",
        Minus => "-",
        Asterisk => "*",
        Slash => "/",
        _ => panic!("Unexpected token for operation sign: {:?}", operation),
    }
}

fn generate_args(num_args: usize) -> String {
    let mut args_str = String::new();

    for i in 0..num_args {
        args_str.push_str(&format!("args[{i}], "));
    }

    args_str.pop();
    args_str.pop();

    args_str
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
                param_str.push_str(&format!("Value {name}"));
                is_comma = true;
            }
            _ => panic!("Unexpected token in function parameters: {:?}", param),
        }
    }

    param_str
}

fn generate_iterator(expression: &[Token]) -> String {
    if expression.is_empty() {
        panic!("Iterator requires at least one token");
    }

    if let Some(dotdot_pos) = expression
        .windows(2)
        .position(|window| matches!(window, [SpecialCharacter(Dot), SpecialCharacter(Dot)]))
    {
        let start_tokens = &expression[..dotdot_pos];
        let end_tokens = &expression[dotdot_pos + 2..];

        let start_expr = if start_tokens.is_empty() {
            "Value(0)".to_string()
        } else {
            generate_expression(start_tokens)
        };

        let end_expr = if end_tokens.is_empty() {
            "Value(2147483647)".to_string() // Max 32-bit signed integer
        } else {
            generate_expression(end_tokens)
        };

        format!("Range({}, {})", start_expr, end_expr)
    } else {
        generate_expression(expression)
    }
}

fn generate_expression(expression: &[Token]) -> String {
    if expression.is_empty() {
        panic!("Expression requires at least one token");
    }

    let mut expression_str = String::new();
    let mut tokens = expression.iter().peekable();

    while let Some(token) = tokens.next() {
        match token {
            Identifier(id) => {
                if [
                    "print", "type", "len", "exit", "null", "int", "float", "bool", "char",
                    "string", "list",
                ]
                .contains(&id.as_str())
                {
                    expression_str.push_str(&format!("_{}", id));
                } else {
                    expression_str.push_str(id);
                }
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
            SpecialCharacter(Dot) => {
                if let Some(Identifier(method)) = tokens.next() {
                    expression_str.push_str(&format!("[\"{method}\"]"));
                } else {
                    panic!(
                        "Expected method name after dot, {:?}, {:?}",
                        token, expression
                    );
                }
            }
            SpecialCharacter(Comma) => {
                expression_str.push_str(", ");
            }
            SpecialCharacter(Equals) => expression_str.push_str("="),
            SpecialCharacter(ExclamationMark) => expression_str.push_str("!"),
            SpecialCharacter(Colon) => expression_str.push_str(":"),
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
