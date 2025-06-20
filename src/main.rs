mod compiler;
mod extra;
mod generator;
mod input;
mod lexer;
mod types;

fn main() {
    let (filenames, command, executable_name) = input::get_input();

    match command.as_str() {
        "clean" => {
            extra::delete_build();
        }
        "build" => {
            compiler::compile(&filenames, &executable_name);
        }
        "run" => {
            compiler::compile(&filenames, &executable_name);
            extra::run_executable(&executable_name);
        }
        _ => unreachable!(),
    }
}
