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
) -> (String, String, bool) {
    if let Some(first) = tokens.first() {
        if !after_imports && first != &Keyword(Import) {
            if filename == "main.ly" {
                return (
                    format!("int main() {{\n{}", match_c_code(tokens, filename)),
                    "".to_string(),
                    true,
                );
            }

            return (
                match_c_code(tokens, filename),
                match_h_code(tokens, filename),
                true,
            );
        } else if tokens.contains(&SpecialCharacter(Colon)) {
            return (
                format!("}}\n\n{}", match_c_code(tokens, filename)),
                match_h_code(tokens, filename),
                true,
            );
        }

        if after_imports && first == &Keyword(Import) {
            panic!("Import statements should be at the beginning of the file");
        }
    }

    (
        match_c_code(tokens, filename),
        match_h_code(tokens, filename),
        after_imports,
    )
}

fn match_c_code(tokens: &Vec<Token>, filename: &str) -> String {
    let filename = filename.trim_end_matches(".ly");

    match tokens.as_slice() {
        [] => "".to_string(),

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
            SpecialCharacter(OpenSquareBracket),
            Literal(index),
            SpecialCharacter(CloseSquareBracket),
        ] => {
            format!("_print_item(&{list}, {index});")
        }
        [Keyword(Print), Identifier(var)] => format!("_print(&{var});"),
        [Keyword(Print), Literal(msg)] => format!("printf(\"{msg}\");"),
        [Keyword(Print), Keyword(True)] => "printf(\"true\");".to_string(),
        [Keyword(Print), Keyword(False)] => "printf(\"false\");".to_string(),

        [
            Keyword(Println),
            Identifier(list),
            SpecialCharacter(OpenSquareBracket),
            Literal(index),
            SpecialCharacter(CloseSquareBracket),
        ] => {
            format!("_println_item(&{list}, {index});")
        }
        [Keyword(Println), Identifier(var)] => format!("_println(&{var});"),
        [Keyword(Println), Literal(msg)] => format!("printf(\"{msg}\\n\");"),
        [Keyword(Println), Keyword(True)] => "printf(\"true\\n\");".to_string(),
        [Keyword(Println), Keyword(False)] => "printf(\"false\\n\");".to_string(),
        [Keyword(Println)] => "printf(\"\\n\");".to_string(),

        [Keyword(var_type), Identifier(var)] => generate_declaration(&var_type, var),
        [
            Keyword(var_type),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => generate_variable_initialization(var_type, dest, src),
        [
            Keyword(var_type),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => generate_literal_initialization(&var_type, var, value),

        [
            Keyword(Bool),
            Identifier(var),
            SpecialCharacter(Assignment),
            Keyword(True),
        ] => format!("Var {var} = {{ TYPE_BOOL, 1.0 }};"),
        [
            Keyword(Bool),
            Identifier(var),
            SpecialCharacter(Assignment),
            Keyword(False),
        ] => format!("Var {var} = {{ TYPE_BOOL, 0.0 }};"),

        [
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => format!("_assign(&{var}, {value});"),
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

        _ => {
            panic!("Unexpected token sequence in file: {filename} - {tokens:?}")
        }
    }
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

fn keyword_to_type(keyword: &Keyword) -> String {
    match keyword {
        Number => "TYPE_NUMBER".to_string(),
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

fn generate_literal_initialization(var_type: &Keyword, var: &str, value: &str) -> String {
    let type_name = keyword_to_type(var_type);

    if value.parse::<f64>().is_ok() {
        format!("Var {var} = {{ {type_name}, .value = 0.0 }};\n_assign(&{var}, {value});")
    } else {
        format!("Var {var} = {{ {type_name}, .value = '{value}' }};")
    }
}

fn generate_variable_initialization(var_type: &Keyword, dest: &str, src: &str) -> String {
    let type_name = keyword_to_type(var_type);
    format!("Var {dest} = {{ {type_name}, .value = 0.0 }};\n_assign(&{dest}, {src}.value);")
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
    } else {
        left.to_string()
    };
    let right_expr = if right_is_var {
        format!("{right}.value")
    } else {
        right.to_string()
    };
    let op_expr = match op {
        Plus => '+',
        Minus => '-',
        Multiply => '*',
        Divide => '/',
        Modulo => '%',
        BitwiseAnd => '&',
        BitwiseOr => '|',
        BitwiseXor => '^',
        _ => panic!("Expected a binary operator, but got: {:?}", op),
    };

    format!(
        "_assign(&{}, {} {} {});",
        dest, left_expr, op_expr, right_expr
    )
}

fn generate_append_literal(list: &str, value: &str) -> String {
    let type_name = if value.parse::<f64>().is_ok() {
        "TYPE_NUMBER"
    } else {
        "TYPE_CHAR"
    };
    format!("_append_literal(&{list}, {type_name}, {value});")
}
