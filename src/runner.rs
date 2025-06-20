pub fn run_executable(executable_name: &str) {
    std::process::Command::new(format!("./build/{}", executable_name))
        .status()
        .expect("Failed to run executable");
}
