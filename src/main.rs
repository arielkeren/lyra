mod compiler;
mod generator;
mod input;
mod lexer;
mod types;

fn main() {
    let (filenames, executable_name) = input::get_input();
    println!("Starting compilation");

    compiler::compile(&filenames, &executable_name);
    println!("Compilation completed successfully");
    println!("Run the executable using: ./build/{executable_name}");
}
