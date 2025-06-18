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
                format!("}}\n{}", match_c_code(tokens, filename)),
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
        [Identifier(function), SpecialCharacter(Colon)] => {
            return format!("void _{}_private_{}() {{\n", filename, function);
        }
        [
            Keyword(Export),
            Identifier(function),
            SpecialCharacter(Colon),
        ] => {
            return format!("void _{}_public_{}() {{\n", filename, function);
        }
        [Keyword(Print), Literal(msg)] => {
            return format!("printf({});\n", msg);
        }
        [Keyword(Call), Identifier(function)] => {
            return format!("_{}_private_{}();\n", filename, function);
        }
        [
            Keyword(Call),
            Identifier(file),
            SpecialCharacter(Dot),
            Identifier(function),
        ] => {
            return format!("_{}_public_{}();\n", file.trim_end_matches(".ly"), function);
        }
        [Keyword(Import), Identifier(file)] => {
            return format!("#include \"{}.h\"\n", file.trim_end_matches(".ly"));
        }
        _ => {
            panic!(
                "Unexpected token sequence in file: {} - {:?}",
                filename, tokens
            );
        }
    }
}

fn match_h_code(tokens: &Vec<crate::types::Token>, filename: &str) -> String {
    let filename = filename.trim_end_matches(".ly");

    match tokens.as_slice() {
        [Identifier(function), SpecialCharacter(Colon)] => {
            return format!("void _{}_private_{}();", filename, function);
        }
        [
            Keyword(Export),
            Identifier(function),
            SpecialCharacter(Colon),
        ] => {
            return format!("void _{}_public_{}();", filename, function);
        }
        _ => {
            return "".to_string();
        }
    }
}
