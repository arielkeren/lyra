mod compiler;
mod extra;
mod generator;
mod input;
mod lexer;
mod stdlib;
mod types;

fn main() {
    let (filenames, args) = input::get_input();

    match args.command.as_str() {
        "clean" => {
            extra::delete_build();
        }
        "build" => {
            compiler::compile(&filenames, &args.executable_name, args.release);
        }
        "run" => {
            compiler::compile(&filenames, &args.executable_name, args.release);
            extra::run_executable(&args.executable_name);
        }
        _ => unreachable!(),
    }
}
