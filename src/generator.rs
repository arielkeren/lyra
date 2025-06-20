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
            return format!("unsigned char *{var} = _alloc({bits});");
        }
        [
            Identifier(var),
            SpecialCharacter(SquareBracketOpen),
            Literal(start),
            SpecialCharacter(Dash),
            Literal(end),
            SpecialCharacter(SquareBracketClose),
            SpecialCharacter(Assignment),
            Literal(value),
        ] => {
            return format!("_assign({var}, {start}, {end}, {value});",);
        }
        [
            Keyword(Print),
            SpecialCharacter(ParanthesisOpen),
            Keyword(Binary),
            SpecialCharacter(ParanthesisClose),
            Identifier(var),
            SpecialCharacter(SquareBracketOpen),
            Literal(start),
            SpecialCharacter(Dash),
            Literal(end),
            SpecialCharacter(SquareBracketClose),
        ] => {
            return format!("_print_binary({var}, {start}, {end});");
        }
        [
            Keyword(Print),
            SpecialCharacter(ParanthesisOpen),
            Keyword(Octal),
            SpecialCharacter(ParanthesisClose),
            Identifier(var),
            SpecialCharacter(SquareBracketOpen),
            Literal(start),
            SpecialCharacter(Dash),
            Literal(end),
            SpecialCharacter(SquareBracketClose),
        ] => {
            return format!("_print_octal({var}, {start}, {end});");
        }
        [
            Keyword(Print),
            SpecialCharacter(ParanthesisOpen),
            Keyword(Hex),
            SpecialCharacter(ParanthesisClose),
            Identifier(var),
            SpecialCharacter(SquareBracketOpen),
            Literal(start),
            SpecialCharacter(Dash),
            Literal(end),
            SpecialCharacter(SquareBracketClose),
        ] => {
            return format!("_print_hex({var}, {start}, {end});");
        }
        [
            Keyword(Print),
            SpecialCharacter(ParanthesisOpen),
            Keyword(Signed),
            SpecialCharacter(ParanthesisClose),
            Identifier(var),
            SpecialCharacter(SquareBracketOpen),
            Literal(start),
            SpecialCharacter(Dash),
            Literal(end),
            SpecialCharacter(SquareBracketClose),
        ] => {
            return format!("_print_signed({var}, {start}, {end});");
        }
        [
            Keyword(Print),
            SpecialCharacter(ParanthesisOpen),
            Keyword(Unsigned),
            SpecialCharacter(ParanthesisClose),
            Identifier(var),
            SpecialCharacter(SquareBracketOpen),
            Literal(start),
            SpecialCharacter(Dash),
            Literal(end),
            SpecialCharacter(SquareBracketClose),
        ] => {
            return format!("_print_unsigned({var}, {start}, {end});");
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
