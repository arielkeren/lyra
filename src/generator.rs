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
        [Keyword(Print), Literal(msg)] => {
            return format!("printf({msg});");
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
        [Keyword(Import), Identifier(file)] => {
            return format!("#include \"{}.h\"\n", file.trim_end_matches(".ly"));
        }
        [
            Identifier(var),
            SpecialCharacter(Assignment),
            Keyword(Alloc),
            Literal(bits),
        ] => {
            return format!(
                "unsigned char *{var} = calloc(({bits} + 7) / 8, 1);\nif (!{var}) {{\nfprintf(stderr, \"Memory allocation failed for variable \\\"{var}\\\"\\n\");\nexit(1);\n}}"
            );
        }
        [
            Identifier(var),
            SpecialCharacter(SquareBracketOpen),
            Literal(start),
            SpecialCharacter(Tilde),
            Literal(end),
            SpecialCharacter(SquareBracketClose),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!(
                "for (size_t i = {start}; i <= {end} - {start}; i++) {{\nif({value} & (1 << i)) {var}[({start} + i) / 8] |= (1 << (({start} + i) % 8));\nelse {var}[({start} + i) / 8] &= ~(1 << (({start} + i) % 8));\n}}"
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
