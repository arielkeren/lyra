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

        [Keyword(Print), Identifier(var)] => format!("_print(&{var});"),
        [Keyword(Print), Literal(msg)] => format!("printf(\"{msg}\");"),
        [Keyword(Print), Keyword(True)] => "printf(\"true\");".to_string(),
        [Keyword(Print), Keyword(False)] => "printf(\"false\");".to_string(),

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
        ] => generate_variable_initilization(var_type, dest, src),
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
        ] => format!("Var {var} = {{ TYPE_BOOL, .value.b = true }};"),
        [
            Keyword(Bool),
            Identifier(var),
            SpecialCharacter(Assignment),
            Keyword(False),
        ] => format!("Var {var} = {{ TYPE_BOOL, .value.b = false }};"),

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

fn keyword_to_type(keyword: &Keyword) -> (String, String, String) {
    match keyword {
        I8 => (
            "TYPE_I8".to_string(),
            "i8".to_string(),
            "int8_t".to_string(),
        ),
        I16 => (
            "TYPE_I16".to_string(),
            "i16".to_string(),
            "int16_t".to_string(),
        ),
        I32 => (
            "TYPE_I32".to_string(),
            "i32".to_string(),
            "int32_t".to_string(),
        ),
        I64 => (
            "TYPE_I64".to_string(),
            "i64".to_string(),
            "int64_t".to_string(),
        ),
        U8 => (
            "TYPE_U8".to_string(),
            "u8".to_string(),
            "uint8_t".to_string(),
        ),
        U16 => (
            "TYPE_U16".to_string(),
            "u16".to_string(),
            "uint16_t".to_string(),
        ),
        U32 => (
            "TYPE_U32".to_string(),
            "u32".to_string(),
            "uint32_t".to_string(),
        ),
        U64 => (
            "TYPE_U64".to_string(),
            "u64".to_string(),
            "uint64_t".to_string(),
        ),
        F32 => (
            "TYPE_F32".to_string(),
            "f32".to_string(),
            "float".to_string(),
        ),
        F64 => (
            "TYPE_F64".to_string(),
            "f64".to_string(),
            "double".to_string(),
        ),
        Bool => ("TYPE_BOOL".to_string(), "b".to_string(), "bool".to_string()),
        Char => ("TYPE_CHAR".to_string(), "c".to_string(), "char".to_string()),
        _ => panic!("Expected a type, but got: {:?}", keyword),
    }
}

fn generate_declaration(var_type: &Keyword, var: &str) -> String {
    if var_type == &List {
        return format!(
            "List {var} = {{ .length = 0, .capacity = 8, .data = malloc(sizeof(Var) * 8) }};"
        );
    }

    let (enum_type, union_type, _) = keyword_to_type(var_type);
    format!("Var {var} = {{ {enum_type}, .value.{union_type} = 0 }};")
}

fn generate_literal_initialization(var_type: &Keyword, var: &str, value: &str) -> String {
    let (enum_type, union_type, c_type) = keyword_to_type(var_type);

    if var_type == &Char {
        format!("Var {var} = {{ {enum_type}, .value.{union_type} = (char)'{value}' }};")
    } else {
        format!("Var {var} = {{ {enum_type}, .value.{union_type} = ({c_type}){value} }};")
    }
}

fn generate_variable_initilization(var_type: &Keyword, dest: &str, src: &str) -> String {
    let (enum_type, union_type, _) = keyword_to_type(var_type);
    format!(
        "Var {dest} = {{ {enum_type}, .value.{union_type} = 0 }};\n_assign(&{dest}, GET_VALUE({src}));"
    )
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
        format!("GET_VALUE({})", left)
    } else {
        left.to_string()
    };
    let right_expr = if right_is_var {
        format!("GET_VALUE({})", right)
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
