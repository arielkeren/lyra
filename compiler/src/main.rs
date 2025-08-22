mod compiler;
mod extra;
mod generator;
mod input;
mod lexer;
mod stdlib;
mod types;

fn main() {
    let (filenames, args) = input::get_input();
    extra::delete_build();

    match args.command.as_str() {
        "build" => {
            compiler::compile(&filenames, &args.executable_name, args.release);
        }
        "run" => {
            compiler::compile(&filenames, &args.executable_name, args.release);
            extra::run_executable(&args.executable_name);
        }
        "clean" => {}
        _ => unreachable!(),
    }
}
