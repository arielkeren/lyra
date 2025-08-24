mod compiler;
mod extra;
mod generator;
mod input;
mod lexer;
mod stdlib;
mod types;

fn main() {
    let (command, args) = input::get_args();

    match command.as_str() {
        "help" => match args.len() {
            0 => extra::help(),
            _ => panic!("Too many arguments for help"),
        },
        "user" => match args.len() {
            0 => extra::user(),
            _ => panic!("Too many arguments for user"),
        },
        "init" => match args.len() {
            0 => extra::init(),
            _ => panic!("Too many arguments for init"),
        },
        "login" => match args.len() {
            0 | 1 => panic!("Expected username and password"),
            2 => extra::login(&args[0], &args[1]),
            _ => panic!("Too many arguments for login"),
        },
        "logout" => match args.len() {
            0 => extra::logout(),
            _ => panic!("Too many arguments for logout"),
        },
        "clean" => match args.len() {
            0 => extra::clean(),
            _ => panic!("Too many arguments for clean"),
        },
        "get" => match args.len() {
            0 => panic!("No package name provided"),
            1 => extra::get(&args[0]),
            _ => panic!("Too many arguments for get"),
        },
        "publish" => match args.len() {
            0 => extra::publish(),
            _ => panic!("Too many arguments for publish"),
        },
        "remove" => match args.len() {
            0 => panic!("No package name provided"),
            1 => extra::remove(&args[0]),
            _ => panic!("Too many arguments for remove"),
        },
        "build" => match args.len() {
            0 => {
                extra::clean();
                compiler::compile(&input::get_filenames(), "program", false);
            }
            1 if args[0] == "-r" || args[0] == "--release" => {
                extra::clean();
                compiler::compile(&input::get_filenames(), "program", true);
            }
            1 => {
                panic!("Expected argument -r or --release")
            }
            _ => panic!("Too many arguments for build"),
        },
        "run" => match args.len() {
            0 => {
                extra::clean();
                compiler::compile(&input::get_filenames(), "program", false);
                extra::run("program");
            }
            1 if args[0] == "-r" || args[0] == "--release" => {
                extra::clean();
                compiler::compile(&input::get_filenames(), "program", true);
                extra::run("program");
            }
            1 => {
                panic!("Expected argument -r or --release")
            }
            _ => panic!("Too many arguments for run"),
        },
        _ => {
            panic!("Unexpected command: {}", command);
        }
    }
}
