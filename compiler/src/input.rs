pub fn get_args() -> (String, Vec<String>) {
    let mut args = std::env::args().skip(1);

    let command = args.next().unwrap_or_else(|| {
        panic!("No command provided");
    });

    (command, args.collect())
}

pub fn get_filenames() -> Vec<String> {
    let mut filenames: Vec<String> = Vec::new();

    for entry in std::fs::read_dir("src").expect("Failed to read src directory") {
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

    filenames
}
