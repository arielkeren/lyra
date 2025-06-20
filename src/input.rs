use crate::types::Args;

pub fn get_input() -> (Vec<String>, Args) {
    let mut filenames: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(".").expect("Failed to read current directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("ly") {
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                filenames.push(filename.to_string());
            }
        }
    }

    if filenames.is_empty() {
        panic!("No Lyra files (extension .ly) found in the current directory");
    }

    if !filenames.contains(&"main.ly".to_string()) {
        panic!("No entry file main.ly found in the current directory");
    }

    let args = read_args();

    (filenames, args)
}

fn read_args() -> Args {
    let mut args = std::env::args().skip(1);
    let mut executable_name = "program".to_string();
    let mut release = false;

    let command = args.next().unwrap_or_else(|| {
        panic!("No command provided.");
    });

    if command != "build" && command != "run" && command != "clean" {
        panic!("Invalid command");
    }

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--output" | "-o" => {
                if let Some(next_arg) = args.next() {
                    executable_name = next_arg;
                } else {
                    panic!("Expected an executable name after --output or -o");
                }
            }
            "--release" | "-r" => {
                release = true;
            }
            _ => {
                panic!("Unknown argument: {}", arg);
            }
        }
    }

    Args {
        command,
        executable_name,
        release,
    }
}
