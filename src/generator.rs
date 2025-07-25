use crate::types::Keyword;
use crate::types::Keyword::*;
use crate::types::SpecialCharacter;
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

        [Keyword(type_name), Identifier(var)] if matches!(type_name, Int | Float | Bool | Char) => {
            generate_declaration(type_name, var)
        }

        [
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => generate_assignment(var, value),
        [Identifier(var), SpecialCharacter(Assignment), Keyword(True)] => {
            format!("_assign(&{var}, true);")
        }
        [
            Identifier(var),
            SpecialCharacter(Assignment),
            Keyword(False),
        ] => format!("_assign(&{var}, false);"),

        [
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(left),
            SpecialCharacter(op),
            Identifier(right),
        ] => generate_operation_assignment(dest, left, op, right, true, true),
        [
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(left),
            SpecialCharacter(op),
            Literal(right),
        ] => generate_operation_assignment(dest, left, op, right, true, false),
        [
            Identifier(dest),
            SpecialCharacter(Assignment),
            Literal(left),
            SpecialCharacter(op),
            Identifier(right),
        ] => generate_operation_assignment(dest, left, op, right, false, true),
        [
            Identifier(dest),
            SpecialCharacter(Assignment),
            Literal(left),
            SpecialCharacter(op),
            Literal(right),
        ] => generate_operation_assignment(dest, left, op, right, false, false),

        [
            Identifier(var),
            SpecialCharacter(Assignment),
            Keyword(var_type),
        ] if matches!(var_type, Int | Float | Bool | Char) => {
            generate_variable_type_change(var, var_type)
        }

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

        [Keyword(If), Identifier(condition)] => format!("if ({condition}.value) {{"),
        [Keyword(Else)] => "else {".to_string(),
        [Keyword(Else), Keyword(If), Identifier(condition)] => {
            format!("else if ({condition}.value) {{")
        }

        [Keyword(While), Identifier(condition)] => format!("while ({condition}.value) {{"),

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
    format!("Var {var} = {{ {type_name}, .value = 0.0 }};")
}

fn generate_assignment(var: &str, value: &str) -> String {
    if value.parse::<f64>().is_ok() {
        format!("_assign(&{var}, {value});")
    } else {
        format!("_assign(&{var}, '{value}');")
    }
}

fn generate_operation_assignment(
    dest: &str,
    left: &str,
    op: &SpecialCharacter,
    right: &str,
    left_is_var: bool,
    right_is_var: bool,
) -> String {
    let left_expr = if left_is_var {
        format!("{left}.value")
    } else if left.parse::<f64>().is_ok() {
        left.to_string()
    } else {
        format!("'{left}'")
    };
    let right_expr = if right_is_var {
        format!("{right}.value")
    } else if right.parse::<f64>().is_ok() {
        right.to_string()
    } else {
        format!("'{right}'")
    };
    let op_expr = match op {
        Plus => '+',
        Minus => '-',
        Multiply => '*',
        Divide => '/',
        Modulo => '%',
        _ => panic!("Expected a binary operator, but got: {:?}", op),
    };

    format!(
        "_assign(&{}, {} {} {});",
        dest, left_expr, op_expr, right_expr
    )
}

fn generate_variable_type_change(var: &str, var_type: &Keyword) -> String {
    let type_name = keyword_to_type(var_type);
    format!("{var}.type = {type_name};\n_assign(&{var}, {var}.value);")
}

fn generate_append_literal(list: &str, value: &str) -> String {
    let type_name = if value.parse::<f64>().is_ok() {
        "TYPE_NUMBER"
    } else {
        "TYPE_CHAR"
    };
    format!("_append_literal(&{list}, {type_name}, {value});")
}
