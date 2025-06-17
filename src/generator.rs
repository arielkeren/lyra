use crate::types::Keyword::*;
use crate::types::Token::*;

pub fn generate(
    tokens: &Vec<crate::types::Token>,
    filename: &str,
    after_imports: bool,
) -> (String, bool) {
    if let Some(first) = tokens.first() {
        if !after_imports && first != &Keyword(Import) {
            if filename == crate::constants::ENTRY_FILENAME {
                return (
                    format!("int main() {{\n{}", match_statement(tokens, filename)),
                    true,
                );
            }

            return (match_statement(tokens, filename), true);
        }

        if after_imports && first == &Keyword(Import) {
            panic!("Import statements should be at the beginning of the file");
        }
    }

    (match_statement(tokens, filename), after_imports)
}

fn match_statement(tokens: &Vec<crate::types::Token>, filename: &str) -> String {
    let filename = filename.trim_end_matches(crate::constants::EXTENSION);

    match tokens.as_slice() {
        [] => {
            return "".to_string();
        }
        [Identifier(function), Keyword(Colon)] => {
            return format!("void _{}_private_{}() {{\n", filename, function);
        }
        [Keyword(Export), Identifier(function), Keyword(Colon)] => {
            return format!("void _{}_public_{}() {{\n", filename, function);
        }
        [Keyword(EndFunction)] => {
            return "}\n".to_string();
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
            Keyword(Dot),
            Identifier(function),
        ] => {
            return format!(
                "_{}_public_{}();\n",
                file.trim_end_matches(crate::constants::EXTENSION),
                function
            );
        }
        [Keyword(Import), Identifier(file)] => {
            return format!(
                "#include \"{}.c\"\n",
                file.trim_end_matches(crate::constants::EXTENSION)
            );
        }
        _ => {
            panic!(
                "Unexpected token sequence in file: {} - {:?}",
                filename, tokens
            );
        }
    }
}
