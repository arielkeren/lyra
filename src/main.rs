mod compiler;
mod generator;
mod input;
mod lexer;
mod runner;
mod types;

fn main() {
    let (filenames, executable_name, should_run) = input::get_input();
    compiler::compile(&filenames, &executable_name);

    if should_run {
        runner::run_executable(&executable_name);
    }
}
