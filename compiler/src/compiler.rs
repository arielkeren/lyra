use crate::types::{Method, Reader, Writer};
use std::io::BufRead;
use std::io::Write;

pub fn compile(filenames: &Vec<String>, executable_name: &str, release: bool) {
    let mut readers = get_readers(filenames);
    let mut writers = get_writers(filenames);

    for ((filename, reader), writer) in filenames
        .iter()
        .zip(readers.iter_mut())
        .zip(writers.iter_mut())
    {
        generate_c_file(filename, reader, writer);
    }

    flush_writers(&mut writers);
    crate::stdlib::write_stdlib();
    create_executable(filenames, executable_name, release);
}

fn get_readers(filenames: &Vec<String>) -> Vec<Reader> {
    filenames
        .iter()
        .map(|filename| {
            let file =
                std::fs::File::open(format!("src/{filename}")).expect("Failed to open input file");
            std::io::BufReader::new(file)
        })
        .collect()
}

fn get_writers(filenames: &Vec<String>) -> Vec<Writer> {
    std::fs::create_dir_all(std::path::Path::new("build/src"))
        .expect("Failed to create a directory for the generated C++ files");
    std::fs::create_dir_all(std::path::Path::new("build/include"))
        .expect("Failed to create a directory for the generated C++ files");

    filenames
        .iter()
        .map(|filename| {
            let base = std::path::Path::new(filename)
                .file_name()
                .expect("Invalid filename")
                .to_string_lossy();
            let output_filename = format!("build/src/{}", base.replace(".ly", ".cpp"));
            let file =
                std::fs::File::create(&output_filename).expect("Failed to create output file");
            std::io::BufWriter::new(file)
        })
        .collect()
}

fn generate_c_file(filename: &str, reader: &mut Reader, writer: &mut Writer) {
    write_includes(filename, writer);

    let mut last_tabs = 0;
    let mut c_code;
    let mut h_code;
    let mut lines = reader.lines().peekable();
    let mut methods = Vec::new();
    let mut header_writer = if filename == "main.ly" {
        None
    } else {
        Some(get_header_writer(filename))
    };

    write_header_guard(filename, &mut header_writer);

    while let Some(line) = lines.peek() {
        let line = line
            .as_ref()
            .expect("Failed to read line from input file")
            .clone();
        let (tokens, _) = crate::lexer::get_tokens(&line);

        if tokens.is_empty() {
            lines.next();
            continue;
        }

        let code = crate::generator::generate_imports(&tokens);
        if let Some(code) = code {
            writeln!(writer, "{code}").expect("Failed to write to output file");
        } else {
            break;
        }

        lines.next();
    }

    if filename == "main.ly" {
        writeln!(writer, "int main() {{").expect("Failed to write to output file");
    }

    while let Some(line) = lines.next() {
        let line = line.expect("Failed to read line from input file");
        let (tokens, mut tabs) = crate::lexer::get_tokens(&line);

        if tokens.is_empty() {
            tabs = last_tabs;
        }

        (c_code, h_code) =
            crate::generator::generate(&tokens, filename, tabs, last_tabs, &mut methods);

        if !c_code.is_empty() {
            writeln!(writer, "{c_code}").expect("Failed to write to output file");
        }

        if let Some(h_writer) = &mut header_writer {
            if !h_code.is_empty() {
                writeln!(h_writer, "{h_code}").expect("Failed to write to header file");
            }
        }

        last_tabs = tabs;
    }

    let scope = (0..last_tabs)
        .map(|i| {
            let brace_tabs = last_tabs - 1 - i;
            format!("{}}}", "\t".repeat(brace_tabs as usize))
        })
        .collect::<Vec<_>>()
        .join("\n");

    write!(writer, "{scope}").expect("Failed to write scope end");
    write_header_ending(&mut header_writer);

    if filename == "main.ly" {
        write_main_ending(writer);
    } else {
        write_ending(writer, methods);
    }
}

fn get_header_writer(filename: &str) -> Writer {
    let base = std::path::Path::new(filename)
        .file_name()
        .expect("Invalid filename")
        .to_string_lossy();
    let header_filename = format!("build/include/{}", base.replace(".ly", ".hpp"));
    let file = std::fs::File::create(&header_filename).expect("Failed to create output file");
    std::io::BufWriter::new(file)
}

fn write_includes(filename: &str, writer: &mut Writer) {
    if filename == "main.ly" {
        writeln!(writer, "#include \"std.hpp\"\n").expect("Failed to write includes");
    } else {
        writeln!(
            writer,
            "#include \"{}.hpp\"\n",
            filename.trim_end_matches(".ly")
        )
        .expect("Failed to write includes");
    }
}

fn write_header_guard(filename: &str, header_writer: &mut Option<Writer>) {
    if let Some(h_writer) = header_writer {
        let namespace = filename.trim_end_matches(".ly");
        write!(
            h_writer,
            "#ifndef {namespace}_HPP\n#define {namespace}_HPP\n\n#include \"std.hpp\"\n\nnamespace {namespace} {{\n",
        )
        .expect("Failed to write header guard");
    }
}

fn write_main_ending(writer: &mut Writer) {
    write!(writer, "}}").expect("Failed to write main function end");
}

fn write_ending(writer: &mut Writer, methods: Vec<Method>) {
    writeln!(writer, "\n[[maybe_unused]] static bool _ = []() {{")
        .expect("Failed to write method registration start");
    for method in methods {
        let Method {
            method,
            num_params,
            args_str,
        } = method;
        writeln!(
            writer,
            r#"Value::register_method(
            "{method}", [](const std::vector<Value>& args) -> Value {{
                if (args.size() != {num_params})
                    throw std::runtime_error("{method} expects {num_params} args");
                return utils_{method}({args_str});
            }});"#
        )
        .expect("Failed to write method registration");
    }
    write!(writer, "return true;\n}}();").expect("Failed to write method registration end");
}

fn write_header_ending(header_writer: &mut Option<Writer>) {
    if let Some(h_writer) = header_writer {
        write!(h_writer, "}}\n\n#endif").expect("Failed to write header end");
    }
}

fn flush_writers(writers: &mut Vec<Writer>) {
    for writer in writers.iter_mut() {
        writer.flush().expect("Failed to flush output file");
    }
}

fn create_executable(filenames: &Vec<String>, executable_name: &str, release: bool) {
    let c_files = filenames
        .iter()
        .map(|filename| {
            let base = std::path::Path::new(filename)
                .file_name()
                .expect("Invalid filename")
                .to_string_lossy();
            format!("build/src/{}", base.replace(".ly", ".cpp"))
        })
        .collect::<Vec<_>>();

    let mut cmd = std::process::Command::new("g++");
    cmd.arg("-Ibuild/include");
    cmd.args(&["-std=c++17", "-Werror", "-Wall", "-Wextra", "-pedantic"]);
    if release {
        cmd.args([
            "-O3",
            "-march=native",
            "-flto",
            "-funroll-loops",
            "-fomit-frame-pointer",
        ]);
    }
    cmd.args(&c_files)
        .arg("build/src/std.cpp")
        .arg("-o")
        .arg(format!("build/{executable_name}"));

    let status = cmd.status().expect("Failed to run g++");

    if !status.success() {
        panic!("gcc failed to compile");
    }
}
