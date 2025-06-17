pub fn get_input() -> (Vec<String>, String) {
    let executable_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "program".to_string());

    let mut filenames: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(".").expect("Failed to read current directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str())
            == Some(crate::constants::EXTENSION.trim_start_matches('.'))
        {
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                filenames.push(filename.to_string());
            }
        }
    }

    if filenames.is_empty() {
        panic!(
            "No {} files (extension {}) found in the current directory",
            crate::constants::LANGUAGE_NAME,
            crate::constants::EXTENSION
        );
    }

    if !filenames.contains(&crate::constants::ENTRY_FILENAME.to_string()) {
        panic!(
            "No entry file {} found in the current directory",
            crate::constants::ENTRY_FILENAME
        );
    }

    (filenames, executable_name)
}
