pub fn help() {
    println!("Lyra Package Manager and Compiler");
    println!("Usage: lyra <command> [arguments]");
    println!();
    println!("Commands:");
    println!("  help                    Show this help message");
    println!("  init                    Create a new lyra.json project file");
    println!("  build [--release|-r]    Compile the project (optional: release mode)");
    println!("  run [--release|-r]      Compile and run the project (optional: release mode)");
    println!("  clean                   Remove the build directory");
    println!();
    println!("Package Management:");
    println!("  get <package_name>      Download and install a package");
    println!("  remove <package_name>   Remove a package from the project");
    println!("  publish                 Publish the current project as a package");
    println!();
    println!("Authentication:");
    println!("  login <email> <password>  Log in to your account");
    println!("  logout                    Log out and remove stored credentials");
    println!("  user                      Show current logged-in user");
    println!();
    println!("Examples:");
    println!("  lyra init                 # Initialize a new project");
    println!("  lyra build                # Build in debug mode");
    println!("  lyra build --release      # Build in release mode");
    println!("  lyra run                  # Build and run in debug mode");
    println!("  lyra get math             # Install the 'math' package");
    println!("  lyra login user@email.com password123");
    println!("  lyra publish              # Publish current project");
    println!();
    println!("Files:");
    println!("  lyra.json                 Project configuration file");
    println!("  lyra.auth                 Authentication token (auto-generated)");
    println!("  packages/                 Downloaded packages directory");
    println!("  build/                    Compiled output directory");
}

pub fn clean() {
    let build_dir = std::path::Path::new("build");
    if build_dir.exists() && std::fs::remove_dir_all(build_dir).is_err() {
        println!("Failed to remove build directory");
    }
}

pub fn run(executable_name: &str) {
    std::process::Command::new(format!("./build/{}", executable_name))
        .status()
        .expect("Failed to run executable");
}

pub fn init() {
    use std::fs::{self, File};
    use std::io::Write;

    let lyra_config = r#"{
  "name": "project",
  "version": "0.0.0",
  "description": "",
  "packages": {}
}"#;

    let gitignore = r#"lyra.auth
build/
packages/"#;

    let main_lyra_content = r#"print('Run me with `lyra run`')"#;

    // Create lyra.json
    match File::create("lyra.json") {
        Ok(mut file) => {
            if let Err(e) = file.write_all(lyra_config.as_bytes()) {
                println!("Failed to write lyra.json: {}", e);
            } else {
                println!("Created lyra.json");
            }
        }
        Err(e) => {
            println!("Failed to create lyra.json: {}", e);
        }
    }

    // Create .gitignore
    match File::create(".gitignore") {
        Ok(mut file) => {
            if let Err(e) = file.write_all(gitignore.as_bytes()) {
                println!("Failed to write .gitignore: {}", e);
            } else {
                println!("Created .gitignore");
            }
        }
        Err(e) => {
            println!("Failed to create .gitignore: {}", e);
        }
    }

    // Create src directory
    if let Err(e) = fs::create_dir("src") {
        println!("Failed to create src directory: {}", e);
        return;
    } else {
        println!("Created src/ directory");
    }

    // Create main.ly inside src directory
    match File::create("src/main.ly") {
        Ok(mut file) => {
            if let Err(e) = file.write_all(main_lyra_content.as_bytes()) {
                println!("Failed to write src/main.ly: {}", e);
            } else {
                println!("Created src/main.ly");
            }
        }
        Err(e) => {
            println!("Failed to create src/main.lyra: {}", e);
        }
    }

    println!("Project initialized successfully!");
}

pub fn remove(package_name: &str) {
    use std::fs;
    use std::path::Path;

    let lyra_config_path = "lyra.json";
    let config_content = fs::read_to_string(lyra_config_path).expect("Failed to read lyra.json");
    let mut config: serde_json::Value =
        serde_json::from_str(&config_content).expect("Failed to parse lyra.json");

    if let Some(packages) = config.get_mut("packages") {
        if packages.get(package_name).is_some() {
            packages.as_object_mut().unwrap().remove(package_name);

            // Extract values
            let name = config
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("project");
            let version = config
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("0.0.0");
            let description = config
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let packages_str =
                serde_json::to_string_pretty(config.get("packages").unwrap()).unwrap();

            let ordered_json = format!(
                r#"{{
  "name": "{}",
  "version": "{}",
  "description": "{}",
  "packages": {}
}}"#,
                name, version, description, packages_str
            );

            fs::write(lyra_config_path, ordered_json).expect("Failed to write lyra.json");

            let package_dir = Path::new("packages").join(&package_name);
            if package_dir.exists() {
                if let Err(e) = fs::remove_dir_all(&package_dir) {
                    println!(
                        "Failed to remove package directory '{}': {}",
                        package_name, e
                    );
                }
            }
        } else {
            println!("Package '{}' not found in lyra.json", package_name);
        }
    } else {
        println!("No packages section found in lyra.json");
    }
}

pub fn get(package_name: &str) {
    use std::fs;
    use std::path::Path;

    // Call the API
    let api_url = format!("http://localhost:8080/packages/{}", package_name);
    let response = match std::process::Command::new("curl")
        .arg("-s")
        .arg(&api_url)
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            println!("Failed to call API: {}", e);
            return;
        }
    };

    if !response.status.success() {
        println!("Failed to fetch package '{}' from API", package_name);
        return;
    }

    let response_str = match std::str::from_utf8(&response.stdout) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to parse API response: {}", e);
            return;
        }
    };

    // Parse the JSON response
    let package_data: serde_json::Value = match serde_json::from_str(response_str) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to parse JSON response: {}", e);
            return;
        }
    };

    // Extract version and files
    let version = package_data
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0");

    let files = match package_data.get("files").and_then(|f| f.as_array()) {
        Some(files) => files,
        None => {
            println!("No files found in package '{}'", package_name);
            return;
        }
    };

    // Create package directory
    let package_dir = Path::new("packages").join(&package_name);
    if let Err(_) = fs::create_dir_all(&package_dir) {
        println!("Failed to create package directory");
        return;
    }

    // Write all files
    for file in files {
        let name = file
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unnamed");
        let content = file.get("content").and_then(|c| c.as_str()).unwrap_or("");

        let file_name = format!("{name}.ly");

        if let Err(_) = fs::write(package_dir.join(&file_name), content) {
            println!("Failed to write file");
        }
    }

    // Update lyra.json
    let lyra_config_path = "lyra.json";
    let config_content = match fs::read_to_string(lyra_config_path) {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read lyra.json: {}", e);
            return;
        }
    };

    let mut config: serde_json::Value = match serde_json::from_str(&config_content) {
        Ok(config) => config,
        Err(e) => {
            println!("Failed to parse lyra.json: {}", e);
            return;
        }
    };

    // Add package to the packages object
    if let Some(packages) = config.get_mut("packages") {
        packages.as_object_mut().unwrap().insert(
            package_name.to_string(),
            serde_json::Value::String(version.to_string()),
        );
    }

    // Preserve order when writing back
    let name = config
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("project");
    let proj_version = config
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0");
    let description = config
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let packages_str = serde_json::to_string_pretty(config.get("packages").unwrap()).unwrap();

    let ordered_json = format!(
        r#"{{
  "name": "{}",
  "version": "{}",
  "description": "{}",
  "packages": {}
}}"#,
        name, proj_version, description, packages_str
    );

    if let Err(e) = fs::write(lyra_config_path, ordered_json) {
        println!("Failed to write lyra.json: {}", e);
        return;
    }

    println!(
        "Package '{}' version '{}' installed successfully",
        package_name, version
    );
}

use std::fs;
use std::path::Path;

pub fn login(email: &str, password: &str) {
    let login_data = serde_json::json!({
        "email": email,
        "password": password
    });

    let response = std::process::Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(login_data.to_string())
        .arg("http://localhost:8080/auth/login")
        .output()
        .expect("Failed to call login API");

    if !response.status.success() {
        println!("Login failed");
        return;
    }

    let response_str = std::str::from_utf8(&response.stdout).unwrap();
    let response_json: serde_json::Value = serde_json::from_str(response_str).unwrap();

    if let Some(token) = response_json.get("token").and_then(|t| t.as_str()) {
        // Store token in current directory
        fs::write("lyra.auth", token).expect("Failed to store token");
    } else {
        println!("Login failed: Invalid response");
    }
}

pub fn logout() {
    if Path::new("lyra.auth").exists() {
        fs::remove_file("lyra.auth").expect("Failed to remove token");
        println!("Logged out successfully");
    } else {
        println!("Not logged in");
    }
}

fn get_stored_token() -> Option<String> {
    if Path::new("lyra.auth").exists() {
        fs::read_to_string("lyra.auth").ok()
    } else {
        None
    }
}

pub fn publish() {
    let token = match get_stored_token() {
        Some(token) => token.trim().to_string(),
        None => {
            println!("Not logged in. Please run 'lyra login' first.");
            return;
        }
    };

    // Read lyra.json to get package info
    let config_content = match fs::read_to_string("lyra.json") {
        Ok(content) => content,
        Err(_) => {
            println!("lyra.json not found. Run 'lyra init' first.");
            return;
        }
    };

    let config: serde_json::Value =
        serde_json::from_str(&config_content).expect("Failed to parse lyra.json");

    let name = config
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("project");
    let version = config
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("0.0.0");
    let description = config
        .get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Collect all .ly files and create the files array
    let files = collect_source_files();

    let publish_data = serde_json::json!({
        "name": name,
        "version": version,
        "description": description,
        "files": files
    });

    let response = std::process::Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-H")
        .arg(format!("Authorization: Bearer {}", token))
        .arg("-d")
        .arg(publish_data.to_string())
        .arg("http://localhost:8080/packages")
        .output()
        .expect("Failed to call publish API");

    if response.status.success() {
        println!(
            "Package '{}' version '{}' published successfully!",
            name, version
        );
    } else {
        let error = std::str::from_utf8(&response.stderr).unwrap_or("Unknown error");
        if error.contains("401") {
            println!("Authentication failed. Please login again with 'lyra login'");
        } else {
            println!("Publish failed: {}", error);
        }
    }
}

fn collect_source_files() -> Vec<serde_json::Value> {
    use std::fs;

    let mut files = Vec::new();

    // Read all .ly files in the src directory
    if let Ok(entries) = fs::read_dir("src") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "ly") {
                if let Some(file_name) = path.file_stem().and_then(|n| n.to_str()) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        files.push(serde_json::json!({
                            "name": file_name,
                            "content": content
                        }));
                    }
                }
            }
        }
    } else {
        println!("Warning: src directory not found");
    }

    files
}

pub fn user() {
    let token = match get_stored_token() {
        Some(token) => token.trim().to_string(),
        None => {
            println!("Not logged in. Please run 'lyra login' first.");
            return;
        }
    };

    // Decode JWT token (simple base64 decode of payload)
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        println!("Invalid token format");
        return;
    }

    // JWT payload is the second part (index 1)
    let payload = parts[1];

    // Add padding if needed for base64 decoding
    let padded_payload = match payload.len() % 4 {
        0 => payload.to_string(),
        n => format!("{}{}", payload, "=".repeat(4 - n)),
    };

    // Decode base64
    let decoded_bytes = match base64_decode(&padded_payload) {
        Ok(bytes) => bytes,
        Err(_) => {
            println!("Failed to decode token");
            return;
        }
    };

    let decoded_str = match std::str::from_utf8(&decoded_bytes) {
        Ok(s) => s,
        Err(_) => {
            println!("Invalid token encoding");
            return;
        }
    };

    // Parse JSON
    let claims: serde_json::Value = match serde_json::from_str(decoded_str) {
        Ok(claims) => claims,
        Err(_) => {
            println!("Failed to parse token claims");
            return;
        }
    };

    // Extract username
    if let Some(username) = claims.get("username").and_then(|u| u.as_str()) {
        println!("Logged in as: {username}");
    } else {
        println!("Username not found in token");
    }
}

// Simple base64 decode function
fn base64_decode(input: &str) -> Result<Vec<u8>, ()> {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = Vec::new();
    let mut buffer = 0u32;
    let mut bits = 0;

    for byte in input.bytes() {
        if byte == b'=' {
            break;
        }

        let value = CHARS.iter().position(|&c| c == byte).ok_or(())? as u32;
        buffer = (buffer << 6) | value;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            result.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }

    Ok(result)
}
