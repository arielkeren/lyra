use crate::types::Keyword::*;
use crate::types::SpecialCharacter::*;
use crate::types::Token::*;

pub fn generate(
    tokens: &Vec<crate::types::Token>,
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

fn match_c_code(tokens: &Vec<crate::types::Token>, filename: &str) -> String {
    let filename = filename.trim_end_matches(".ly");

    match tokens.as_slice() {
        [] => {
            return "".to_string();
        }

        [Keyword(Import), Identifier(file)] => {
            return format!("#include \"{}.h\"\n", file.trim_end_matches(".ly"));
        }

        [Identifier(function), SpecialCharacter(Colon)] => {
            return format!("void _{filename}_private_{function}() {{");
        }
        [
            Keyword(Export),
            Identifier(function),
            SpecialCharacter(Colon),
        ] => {
            return format!("void _{filename}_public_{function}() {{");
        }

        [Keyword(Call), Identifier(function)] => {
            return format!("_{filename}_private_{function}();");
        }
        [
            Keyword(Call),
            Identifier(file),
            SpecialCharacter(Dot),
            Identifier(function),
        ] => {
            return format!("_{}_public_{function}();", file.trim_end_matches(".ly"));
        }

        [Keyword(Print), Identifier(var)] => {
            return format!("_print(&{var});");
        }
        [Keyword(Println), Identifier(var)] => {
            return format!("_println(&{var});");
        }
        [Keyword(Print), Literal(msg)] => {
            return format!("printf({msg});");
        }
        [Keyword(Println), Literal(msg)] => {
            return format!("printf({}\\n\");", msg.trim_end_matches("\""));
        }

        [Keyword(I8), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_I8, .value.i8 = 0 }};");
        }
        [Keyword(I16), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_I16, .value.i16 = 0 }};");
        }
        [Keyword(I32), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_I32, .value.i32 = 0 }};");
        }
        [Keyword(I64), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_I64, .value.i64 = 0 }};");
        }
        [Keyword(U8), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_U8, .value.u8 = 0 }};");
        }
        [Keyword(U16), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_U16, .value.u16 = 0 }};");
        }
        [Keyword(U32), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_U32, .value.u32 = 0 }};");
        }
        [Keyword(U64), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_U64, .value.u64 = 0 }};");
        }
        [Keyword(F32), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_F32, .value.f32 = 0.0 }};");
        }
        [Keyword(F64), Identifier(var)] => {
            return format!("Var {var} = {{ TYPE_F64, .value.f64 = 0.0 }};");
        }

        [
            Keyword(I8),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_I8, .value.i8 = (uint8_t){value} }};");
        }
        [
            Keyword(I16),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_I16, .value.i16 = (uint16_t){value} }};");
        }
        [
            Keyword(I32),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_I32, .value.i32 = (int32_t){value} }};");
        }
        [
            Keyword(I64),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_I64, .value.i64 = (int64_t){value} }};");
        }
        [
            Keyword(U8),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_U8, .value.u8 = (uint8_t){value} }};");
        }
        [
            Keyword(U16),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_U16, .value.u16 = (uint16_t){value} }};");
        }
        [
            Keyword(U32),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_U32, .value.u32 = (uint32_t){value} }};");
        }
        [
            Keyword(U64),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_U64, .value.u64 = (uint64_t){value} }};");
        }
        [
            Keyword(F32),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_F32, .value.f32 = (float){value} }};");
        }
        [
            Keyword(F64),
            Identifier(var),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("Var {var} = {{ TYPE_F64, .value.f64 = (double){value} }};");
        }

        [
            Keyword(I8),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_I8, .value.i8 = 0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        [
            Keyword(I16),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_I16, .value.i16 = 0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        [
            Keyword(I32),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_I32, .value.i32 = 0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        [
            Keyword(I64),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_I64, .value.i64 = 0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        [
            Keyword(U8),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_U8, .value.u8 = 0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        [
            Keyword(U16),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_U16, .value.u16 = 0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        [
            Keyword(U32),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_U32, .value.u32 = 0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        [
            Keyword(U64),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_U64, .value.u64 = 0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        [
            Keyword(F32),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_F32, .value.f32 = 0.0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        [
            Keyword(F64),
            Identifier(dest),
            SpecialCharacter(Assignment),
            Identifier(src),
        ] => {
            return format!(
                "Var {dest} = {{ TYPE_F64, .value.f64 = 0.0 }};\n_assign_var(&{dest}, &{src});"
            );
        }
        _ => {
            panic!("Unexpected token sequence in file: {filename} - {tokens:?}");
        }
    }
}

fn match_h_code(tokens: &Vec<crate::types::Token>, filename: &str) -> String {
    let filename = filename.trim_end_matches(".ly");

    match tokens.as_slice() {
        [Identifier(function), SpecialCharacter(Colon)] => {
            return format!("void _{filename}_private_{function}();");
        }
        [
            Keyword(Export),
            Identifier(function),
            SpecialCharacter(Colon),
        ] => {
            return format!("void _{filename}_public_{function}();");
        }
        _ => {
            return "".to_string();
        }
    }
}
