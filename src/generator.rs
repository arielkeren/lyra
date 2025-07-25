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
    let scope = if tabs < last_tabs { "}\n" } else { "" };

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
            Identifier(list),
            SpecialCharacter(Dot),
            Literal(index),
        ] => {
            format!("_print_item(&{list}, {index});")
        }
        [
            Keyword(Print),
            Identifier(list),
            SpecialCharacter(Dot),
            Identifier(index),
        ] => {
            format!("_print_item(&{list}, (int){index}.value);")
        }
        [Keyword(Print), Identifier(var)] => format!("_print(&{var});"),
        [Keyword(Print), Literal(msg)] => format!("printf(\"{msg}\");"),
        [Keyword(Print), Keyword(True)] => "printf(\"true\");".to_string(),
        [Keyword(Print), Keyword(False)] => "printf(\"false\");".to_string(),

        [
            Keyword(Println),
            Identifier(list),
            SpecialCharacter(Dot),
            Literal(index),
        ] => {
            format!("_println_item(&{list}, {index});")
        }
        [
            Keyword(Println),
            Identifier(list),
            SpecialCharacter(Dot),
            Identifier(index),
        ] => {
            format!("_print_item(&{list}, (int){index}.value);")
        }
        [Keyword(Println), Identifier(var)] => format!("_println(&{var});"),
        [Keyword(Println), Literal(text)] => format!("printf(\"{text}\\n\");"),
        [Keyword(Println), Keyword(True)] => "printf(\"true\\n\");".to_string(),
        [Keyword(Println), Keyword(False)] => "printf(\"false\\n\");".to_string(),
        [Keyword(Println)] => "printf(\"\\n\");".to_string(),

        [
            Identifier(var),
            SpecialCharacter(Plus),
            SpecialCharacter(Plus),
        ] => {
            format!("++{var}.value;")
        }
        [
            Identifier(var),
            SpecialCharacter(Minus),
            SpecialCharacter(Minus),
        ] => {
            format!("--{var}.value;")
        }

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
        .enumerate()
        .map(|(_, line)| format!("{}{}", tab_str, line))
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

fn generate_expression(expression: &[Token]) -> String {
    if expression.is_empty() {
        panic!("Expression requires at least one token");
    }

    let mut is_first_token = true;
    let mut expression_str = String::new();
    let mut tokens = expression.iter().peekable();
    while let Some(token) = tokens.next() {
        match token {
            Identifier(var) => expression_str.push_str(&format!("{var}.value")),
            Literal(value) => {
                if value.parse::<f64>().is_ok() {
                    expression_str.push_str(value);
                } else {
                    expression_str.push_str(&format!("'{value}'"));
                }
            }
            SpecialCharacter(OpenParenthesis) => expression_str.push_str("("),
            SpecialCharacter(CloseParenthesis) => expression_str.push_str(")"),
            SpecialCharacter(Plus) => expression_str.push_str(" + "),
            SpecialCharacter(Minus) => expression_str.push_str(" - "),
            SpecialCharacter(Multiply) => expression_str.push_str(" * "),
            SpecialCharacter(Divide) => expression_str.push_str(" / "),
            SpecialCharacter(Modulo) => expression_str.push_str(" % "),
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
            Keyword(True) => expression_str.push_str("true"),
            Keyword(False) => expression_str.push_str("false"),
            Keyword(And) => expression_str.push_str(" && "),
            Keyword(Or) => expression_str.push_str(" || "),
            _ => panic!("Unexpected token in expression: {:?}", token),
        }

        is_first_token = false;
    }

    expression_str
}
