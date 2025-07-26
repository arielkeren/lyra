use crate::types::Keyword;
use crate::types::Keyword::*;
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
                let brace_tabs = last_tabs - 1 - i;
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
                        "int main() {{\n{}{}",
                        scope,
                        match_c_code(tokens, filename, tabs)
                    ),
                    "".to_string(),
                    true,
                );
            }

            return (
                format!("{}{}", scope, match_c_code(tokens, filename, tabs)),
                match_h_code(tokens, filename),
                true,
            );
        } else if tokens.contains(&SpecialCharacter(Colon)) {
            return (
                format!("}}\n\n{}{}", scope, match_c_code(tokens, filename, tabs)),
                match_h_code(tokens, filename),
                true,
            );
        }

        if after_imports && first == &Keyword(Import) {
            panic!("Import statements should be at the beginning of the file");
        }
    }

    (
        format!("{}{}", scope, match_c_code(tokens, filename, tabs)),
        match_h_code(tokens, filename),
        after_imports,
    )
}

fn match_c_code(tokens: &Vec<Token>, filename: &str, tabs: u8) -> String {
    let filename = filename.trim_end_matches(".ly");

    let code = match tokens.as_slice() {
        [] => "".to_string(),

        [Keyword(Break)] => "break;".to_string(),
        [Keyword(Continue)] => "continue;".to_string(),

        [Keyword(Import), Identifier(file)] => {
            format!("#include \"{}.h\"\n", file.trim_end_matches(".ly"))
        }

        [Identifier(function), SpecialCharacter(Colon)] => {
            format!("void _{filename}_private_{function}() {{")
        }
        [
            Keyword(Export),
            Identifier(function),
            SpecialCharacter(Colon),
        ] => format!("void _{filename}_public_{function}() {{"),

        [Keyword(Call), Identifier(function)] => format!("_{filename}_private_{function}();"),
        [
            Keyword(Call),
            Identifier(file),
            SpecialCharacter(Dot),
            Identifier(function),
        ] => format!("_{}_public_{function}();", file.trim_end_matches(".ly")),

        [
            Keyword(Print),
            SpecialCharacter(OpenParenthesis),
            args @ ..,
            SpecialCharacter(CloseParenthesis),
        ] => generate_print(args),

        [
            Identifier(var),
            SpecialCharacter(Plus),
            SpecialCharacter(Plus),
        ] => format!("++{var}.value;"),

        [
            Identifier(var),
            SpecialCharacter(Minus),
            SpecialCharacter(Minus),
        ] => format!("--{var}.value;"),

        [Keyword(var_type), Identifier(var)]
            if matches!(var_type, List | Int | Float | Bool | Char) =>
        {
            generate_declaration(var_type, var)
        }

        [
            Keyword(var_type),
            Identifier(var),
            SpecialCharacter(Assignment),
            expression @ ..,
        ] if matches!(var_type, Int | Float | Bool | Char) => {
            generate_initialization(var_type, var, expression)
        }

        [
            Keyword(Const),
            Keyword(var_type),
            Identifier(var),
            SpecialCharacter(Assignment),
            expression @ ..,
        ] if matches!(var_type, Int | Float | Bool | Char) => {
            format!(
                "const {}",
                generate_initialization(var_type, var, expression)
            )
        }

        [
            Identifier(var),
            SpecialCharacter(Assignment),
            Keyword(var_type),
        ] if matches!(var_type, Int | Float | Bool | Char) => {
            generate_variable_type_change(var, var_type)
        }

        [
            Identifier(var),
            SpecialCharacter(Assignment),
            expression @ ..,
        ] => format!("_assign(&{var}, {});", generate_expression(expression)),

        [Identifier(list), SpecialCharacter(Plus), Identifier(var)] => {
            format!("_append_var(&{list}, &{var});")
        }
        [Identifier(list), SpecialCharacter(Plus), Literal(value)] => {
            generate_append_literal(list, value)
        }
        [Identifier(list), SpecialCharacter(Plus), Keyword(True)] => {
            format!("_append_literal(&{list}, TYPE_BOOL, 1.0);")
        }
        [Identifier(list), SpecialCharacter(Plus), Keyword(False)] => {
            format!("_append_literal(&{list}, TYPE_BOOL, 0.0);")
        }

        [Keyword(If), condition @ ..] => format!("if ({}) {{", generate_expression(condition)),
        [Keyword(Else)] => "else {".to_string(),
        [Keyword(Else), Keyword(If), condition @ ..] => {
            format!("else if ({}) {{", generate_expression(condition))
        }

        [Keyword(While), condition @ ..] => {
            format!("while ({}) {{", generate_expression(condition))
        }

        _ => {
            panic!("Unexpected token sequence in file: {filename} - {tokens:?}")
        }
    };

    add_tabs_after_newlines(&code, tabs)
}

fn match_h_code(tokens: &Vec<Token>, filename: &str) -> String {
    let filename = filename.trim_end_matches(".ly");

    match tokens.as_slice() {
        [Identifier(function), SpecialCharacter(Colon)] => {
            format!("void _{filename}_private_{function}();")
        }

        [
            Keyword(Export),
            Identifier(function),
            SpecialCharacter(Colon),
        ] => format!("void _{filename}_public_{function}();"),

        _ => "".to_string(),
    }
}

fn add_tabs_after_newlines(code: &str, tabs: u8) -> String {
    let tab_str = "\t".repeat(tabs as usize);
    code.lines()
        .map(|line| format!("{}{}", tab_str, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn keyword_to_type(keyword: &Keyword) -> String {
    match keyword {
        Int => "TYPE_INT".to_string(),
        Float => "TYPE_FLOAT".to_string(),
        Bool => "TYPE_BOOL".to_string(),
        Char => "TYPE_CHAR".to_string(),
        _ => panic!("Expected a type, but got: {:?}", keyword),
    }
}

fn generate_declaration(var_type: &Keyword, var: &str) -> String {
    if var_type == &List {
        return format!("List {var} = _create_list();");
    }

    let type_name = keyword_to_type(var_type);
    format!("Var {var} = {{ {type_name}, 0.0 }};")
}

fn generate_initialization(var_type: &Keyword, var: &str, expression: &[Token]) -> String {
    if expression.is_empty() {
        panic!("Initialization cannot be empty");
    }

    let type_name = keyword_to_type(var_type);
    format!(
        "Var {var} = {{ {type_name}, 0.0 }};\n_assign(&{var}, {});",
        generate_expression(expression)
    )
}

fn generate_variable_type_change(var: &str, var_type: &Keyword) -> String {
    let type_name = keyword_to_type(var_type);
    format!("{var}.type = {type_name};\n_assign(&{var}, {var}.value);")
}

fn generate_append_literal(list: &str, value: &str) -> String {
    let type_name = if value.parse::<i64>().is_ok() {
        "TYPE_INT"
    } else if value.parse::<f64>().is_ok() {
        "TYPE_FLOAT"
    } else {
        "TYPE_CHAR"
    };
    format!("_append_literal(&{list}, {type_name}, {value});")
}

fn generate_print(args: &[Token]) -> String {
    let arguments = split_arguments(args);

    if arguments.is_empty() {
        return "printf(\"\\n\");".to_string();
    }

    let format_arg = &arguments[0];
    let expr_args = &arguments[1..];

    if is_string_literal(format_arg) {
        let format_str = extract_string_content(format_arg);
        let (c_format, c_args) = process_format_string(format_str, expr_args);
        format!("printf(\"{}\"{});", c_format, c_args)
    } else {
        panic!("First argument of print must be a string literal");
    }
}

fn is_string_literal(tokens: &[Token]) -> bool {
    tokens.len() == 1 && matches!(tokens[0], Literal(ref s) if !s.parse::<f64>().is_ok())
}

fn extract_string_content(tokens: &[Token]) -> &str {
    if let [Literal(s)] = tokens {
        s
    } else {
        panic!("Expected string literal");
    }
}

fn split_arguments(args: &[Token]) -> Vec<Vec<Token>> {
    let mut arguments = Vec::new();
    let mut current_arg = Vec::new();
    let mut paren_depth = 0;

    for token in args {
        match token {
            SpecialCharacter(Comma) if paren_depth == 0 => {
                if !current_arg.is_empty() {
                    arguments.push(current_arg.clone());
                    current_arg.clear();
                }
            }
            SpecialCharacter(OpenParenthesis) => {
                paren_depth += 1;
                current_arg.push(token.clone());
            }
            SpecialCharacter(CloseParenthesis) => {
                paren_depth -= 1;
                current_arg.push(token.clone());
            }
            _ => current_arg.push(token.clone()),
        }
    }

    if !current_arg.is_empty() {
        arguments.push(current_arg);
    }

    arguments
}

fn process_format_string(format_str: &str, args: &[Vec<Token>]) -> (String, String) {
    let mut c_format = String::new();
    let mut c_args = Vec::new();
    let mut arg_index = 0;

    let mut chars = format_str.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut type_spec = String::new();
            while let Some(&next_ch) = chars.peek() {
                if next_ch == '}' {
                    chars.next();
                    break;
                }
                type_spec.push(chars.next().unwrap());
            }

            if arg_index < args.len() {
                let (c_spec, c_arg) = convert_type_spec(&type_spec, &args[arg_index]);
                c_format.push_str(&c_spec);
                c_args.push(c_arg);
                arg_index += 1;
            } else {
                panic!("Not enough arguments for format string");
            }
        } else {
            c_format.push(ch);
        }
    }

    c_format.push_str("\\n");

    let args_str = if c_args.is_empty() {
        String::new()
    } else {
        format!(", {}", c_args.join(", "))
    };

    (c_format, args_str)
}

fn convert_type_spec(type_spec: &str, arg_tokens: &[Token]) -> (String, String) {
    let expr = generate_expression(arg_tokens);

    match type_spec {
        "int" => ("%d".to_string(), format!("(int)({})", expr)),
        "float" => ("%.15g".to_string(), format!("(double)({})", expr)),
        "char" => ("%c".to_string(), format!("(char)({})", expr)),
        "bool" => (
            "%s".to_string(),
            format!("({}) ? \"true\" : \"false\"", expr),
        ),
        "hex" => ("%x".to_string(), format!("(unsigned int)({})", expr)),
        "octal" => ("%o".to_string(), format!("(unsigned int)({})", expr)),
        _ => panic!("Unknown format specifier: {}", type_spec),
    }
}

fn generate_expression(expression: &[Token]) -> String {
    if expression.is_empty() {
        panic!("Expression requires at least one token");
    }

    let mut is_first_token = true;
    let mut expression_str = String::new();
    let mut tokens = expression.iter().peekable();
    while let Some(token) = tokens.next() {
        match token {
            Identifier(var) => {
                if tokens.peek() == Some(&&SpecialCharacter(Modulo)) {
                    tokens.next();
                    if let Some(next_token) = tokens.next() {
                        if let Literal(next_value) = next_token {
                            if next_value.parse::<f64>().is_ok() {
                                expression_str
                                    .push_str(&format!("_mod({var}.value, {next_value})"));
                            } else {
                                expression_str
                                    .push_str(&format!("_mod({var}.value, '{next_value}')"));
                            }
                        } else if let Identifier(next_var) = next_token {
                            expression_str
                                .push_str(&format!("_mod({var}.value, {next_var}.value)"));
                        }
                    }
                } else {
                    expression_str.push_str(&format!("{var}.value"));
                }
            }
            Literal(value) => {
                let to_push = if value.parse::<f64>().is_ok() {
                    value.to_string()
                } else {
                    format!("'{value}'")
                };

                if tokens.peek() == Some(&&SpecialCharacter(Modulo)) {
                    tokens.next();
                    if let Some(next_token) = tokens.next() {
                        if let Literal(next_value) = next_token {
                            if next_value.parse::<f64>().is_ok() {
                                expression_str.push_str(&format!("_mod({to_push}, {next_value})"));
                            } else {
                                expression_str
                                    .push_str(&format!("_mod({to_push}, '{next_value}')"));
                            }
                        } else if let Identifier(next_var) = next_token {
                            expression_str.push_str(&format!("_mod({to_push}, {next_var}.value)"));
                        }
                    }
                } else {
                    expression_str.push_str(&to_push);
                }
            }
            SpecialCharacter(OpenParenthesis) => expression_str.push_str("("),
            SpecialCharacter(CloseParenthesis) => expression_str.push_str(")"),
            SpecialCharacter(Plus) => expression_str.push_str(" + "),
            SpecialCharacter(Minus) => expression_str.push_str(" - "),
            SpecialCharacter(Multiply) => expression_str.push_str(" * "),
            SpecialCharacter(Divide) => expression_str.push_str(" / "),
            SpecialCharacter(Assignment) => expression_str.push_str(" == "),
            SpecialCharacter(LargerThan) => {
                if tokens.peek() == Some(&&SpecialCharacter(Assignment)) {
                    expression_str.push_str(" >= ");
                    tokens.next();
                } else {
                    expression_str.push_str(" > ");
                }
            }
            SpecialCharacter(SmallerThan) => {
                if tokens.peek() == Some(&&SpecialCharacter(Assignment)) {
                    expression_str.push_str(" <= ");
                    tokens.next();
                } else {
                    expression_str.push_str(" < ");
                }
            }
            SpecialCharacter(ExclamationMark) => {
                let space_before = if is_first_token { "" } else { " " };
                if tokens.peek() == Some(&&SpecialCharacter(Assignment)) {
                    expression_str.push_str(&format!("{space_before}!= "));
                    tokens.next();
                } else {
                    expression_str.push_str(&format!("{space_before}!"));
                }
            }
            Keyword(True) => expression_str.push_str("1"),
            Keyword(False) => expression_str.push_str("0"),
            Keyword(And) => expression_str.push_str(" && "),
            Keyword(Or) => expression_str.push_str(" || "),
            _ => panic!("Unexpected token in expression: {:?}", token),
        }

        is_first_token = false;
    }

    expression_str
}
