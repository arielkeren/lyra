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

fn match_c_code(tokens: &Vec<Token>, mut filename: &str, tabs: u8) -> String {
    filename = filename.trim_end_matches(".ly");

    let code = match tokens.as_slice() {
        [] => "".to_string(),

        [Keyword(Break)] => "break;".to_string(),
        [Keyword(Continue)] => "continue;".to_string(),

        [Keyword(Import), Identifier(file)] => {
            format!("#include \"{}.h\"\n", file.trim_end_matches(".ly"))
        }

        [Keyword(Return), expression @ ..] if filename != "main" => {
            format!("return {};", generate_expression(expression))
        }

        [
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            params @ ..,
            SpecialCharacter(CloseParenthesis),
        ] if filename != "main" => {
            let (param_str, body_str) = generate_function(params);
            format!("double _{filename}_private_{function}({param_str}) {{\n{body_str}")
        }

        [
            Keyword(Export),
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            params @ ..,
            SpecialCharacter(CloseParenthesis),
        ] if filename != "main" => {
            let (param_str, body_str) = generate_function(params);
            format!("double _{filename}_public_{function}({param_str}) {{\n{body_str}")
        }

        [
            Identifier(file),
            SpecialCharacter(Dot),
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            args @ ..,
            SpecialCharacter(CloseParenthesis),
        ] => {
            format!(
                "_{file}_public_{function}({});",
                generate_function_args(args)
            )
        }

        [
            Keyword(list_type),
            SpecialCharacter(OpenBracket),
            SpecialCharacter(CloseBracket),
            Identifier(list_name),
        ] => format!(
            "List {list_name} = _create_list({});",
            keyword_to_type(list_type)
        ),

        [
            Keyword(list_type),
            SpecialCharacter(OpenBracket),
            SpecialCharacter(CloseBracket),
            Identifier(list_name),
            SpecialCharacter(Assignment),
            SpecialCharacter(OpenBracket),
            elements @ ..,
            SpecialCharacter(CloseBracket),
        ] => generate_list_initialization(list_type, list_name, elements),

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

        [Keyword(var_type), Identifier(var)] if matches!(var_type, Int | Float | Bool | Char) => {
            format!("Var {var} = {{ {}, 0.0 }};", keyword_to_type(var_type))
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
            Keyword(new_type),
        ] if matches!(new_type, Int | Float | Bool | Char) => {
            format!(
                "{var}.type = {};\n_assign(&{var}, {var}.value);",
                keyword_to_type(new_type)
            )
        }

        [
            Identifier(var),
            SpecialCharacter(Assignment),
            expression @ ..,
        ] => format!("_assign(&{var}, {});", generate_expression(expression)),

        [Keyword(If), condition @ ..] => format!("if ({}) {{", generate_expression(condition)),
        [Keyword(Else)] => "else {".to_string(),
        [Keyword(Else), Keyword(If), condition @ ..] => {
            format!("else if ({}) {{", generate_expression(condition))
        }

        [Keyword(While), condition @ ..] => {
            format!("while ({}) {{", generate_expression(condition))
        }

        [Keyword(For), Identifier(var), Keyword(In), Identifier(list)] => {
            format!(
                "for (size_t _index = 0; _index < {list}.length; ++_index) {{\n\tVar {var};\n\t{var}.type = {list}.type;\n\t{var}.value = {list}.data[_index];"
            )
        }

        _ => {
            panic!("Unexpected token sequence in file: {filename} - {tokens:?}")
        }
    };

    add_tabs_after_newlines(&code, tabs, filename)
}

fn match_h_code(tokens: &Vec<Token>, mut filename: &str) -> String {
    filename = filename.trim_end_matches(".ly");

    match tokens.as_slice() {
        [
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            params @ ..,
            SpecialCharacter(CloseParenthesis),
        ] => format!(
            "double _{filename}_private_{function}({});",
            generate_function(params).0
        ),

        [
            Keyword(Export),
            Identifier(function),
            SpecialCharacter(OpenParenthesis),
            params @ ..,
            SpecialCharacter(CloseParenthesis),
        ] => format!(
            "double _{filename}_public_{function}({});",
            generate_function(params).0
        ),

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

fn keyword_to_type(keyword: &Keyword) -> String {
    match keyword {
        Int => "TYPE_INT".to_string(),
        Float => "TYPE_FLOAT".to_string(),
        Bool => "TYPE_BOOL".to_string(),
        Char => "TYPE_CHAR".to_string(),
        _ => panic!("Expected a type, but got: {:?}", keyword),
    }
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

fn generate_list_initialization(
    list_type: &Keyword,
    list_name: &str,
    elements: &[Token],
) -> String {
    let mut initialization_str = format!(
        "List {list_name} = _create_list({});",
        keyword_to_type(list_type)
    );
    let split_elements = split_arguments(elements);

    for element in split_elements.iter() {
        initialization_str.push_str(&format!(
            "\n_append(&{list_name}, {});",
            generate_expression(element)
        ));
    }
    initialization_str
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
        "int" => ("%d".to_string(), format!("(int)({expr})")),
        "float" => ("%.15g".to_string(), format!("(double)({expr})")),
        "char" => ("%c".to_string(), format!("(char)({expr})")),
        "bool" => ("%s".to_string(), format!("({expr}) ? \"true\" : \"false\"")),
        "hex" => ("%x".to_string(), format!("(unsigned int)({expr})")),
        "octal" => ("%o".to_string(), format!("(unsigned int)({expr})")),
        _ => panic!("Unknown format specifier: {type_spec}"),
    }
}

fn generate_function(params: &[Token]) -> (String, String) {
    let parameters = split_arguments(params);
    if parameters.is_empty() {
        return ("void".to_string(), "".to_string());
    }

    let mut body_str = String::new();
    let mut param_str = String::new();
    for param in &parameters {
        if param.len() != 2
            || !matches!(param[0], Keyword(Int | Float | Bool | Char))
            || !matches!(param[1], Identifier(_))
        {
            panic!(
                "Function parameters must be in the form: <type_name> <variable_name>, found: {:?}",
                param
            );
        }

        if let Keyword(keyword) = &param[0] {
            if let Identifier(var_name) = &param[1] {
                param_str.push_str(&format!("double _{var_name}, "));
                body_str.push_str(&format!(
                    "\tVar {var_name} = {{{}, _{var_name}}};\n",
                    keyword_to_type(keyword)
                ));
            }
        }
    }

    param_str.pop();
    param_str.pop();

    (param_str, body_str)
}

fn generate_function_args(args: &[Token]) -> String {
    split_arguments(args)
        .iter()
        .map(|arg| generate_expression(arg))
        .collect::<Vec<_>>()
        .join(", ")
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
            Identifier(id) => {
                if tokens.peek() == Some(&&SpecialCharacter(Modulo)) {
                    tokens.next();
                    if let Some(next_token) = tokens.next() {
                        if let Literal(next_value) = next_token {
                            if next_value.parse::<f64>().is_ok() {
                                expression_str.push_str(&format!("_mod({id}.value, {next_value})"));
                            } else {
                                expression_str
                                    .push_str(&format!("_mod({id}.value, '{next_value}')"));
                            }
                        } else if let Identifier(next_var) = next_token {
                            expression_str.push_str(&format!("_mod({id}.value, {next_var}.value)"));
                        }
                    }
                } else if tokens.peek() == Some(&&SpecialCharacter(Dot)) {
                    tokens.next();
                    if let Some(next_token) = tokens.next() {
                        if let Identifier(function) = next_token {
                            let mut function_args = Vec::new();

                            if tokens.peek() != Some(&&SpecialCharacter(OpenParenthesis)) {
                                panic!("Expected opening parenthesis after function name");
                            }
                            tokens.next();

                            let mut paren_depth = 1;
                            while let Some(token) = tokens.peek() {
                                match token {
                                    SpecialCharacter(OpenParenthesis) => {
                                        paren_depth += 1;
                                        function_args.push((*tokens.next().unwrap()).clone());
                                    }
                                    SpecialCharacter(CloseParenthesis) => {
                                        paren_depth -= 1;
                                        if paren_depth == 0 {
                                            tokens.next();
                                            break;
                                        } else {
                                            function_args.push((*tokens.next().unwrap()).clone());
                                        }
                                    }
                                    _ => {
                                        function_args.push((*tokens.next().unwrap()).clone());
                                    }
                                }
                            }

                            if paren_depth > 0 {
                                panic!(
                                    "Missing closing parenthesis for function call: {id}.{function}"
                                );
                            }

                            expression_str.push_str(&format!(
                                "_{id}_public_{function}({})",
                                generate_function_args(&function_args)
                            ));
                        } else {
                            panic!("Expected function after dot, but found something else");
                        }
                    } else {
                        panic!("Expected function after dot, but found none");
                    }
                } else {
                    expression_str.push_str(&format!("{id}.value"));
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
