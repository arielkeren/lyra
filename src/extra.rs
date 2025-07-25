pub fn delete_build() {
    let build_dir = std::path::Path::new("build");
    if build_dir.exists() && std::fs::remove_dir_all(build_dir).is_err() {
        println!("Failed to remove build directory");
    }
}

pub fn run_executable(executable_name: &str) {
    std::process::Command::new(format!("./build/{}", executable_name))
        .status()
        .expect("Failed to run executable");
}
