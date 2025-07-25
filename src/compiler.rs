use crate::types::{Reader, Writer};
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
            let file = std::fs::File::open(filename).expect("Failed to open input file");
            std::io::BufReader::new(file)
        })
        .collect()
}

fn get_writers(filenames: &Vec<String>) -> Vec<Writer> {
    std::fs::create_dir_all(std::path::Path::new("build/src"))
        .expect("Failed to create a directory for the generated C files");
    std::fs::create_dir_all(std::path::Path::new("build/include"))
        .expect("Failed to create a directory for the generated C files");

    filenames
        .iter()
        .map(|filename| {
            let base = std::path::Path::new(filename)
                .file_name()
                .expect("Invalid filename")
                .to_string_lossy();
            let output_filename = format!("build/src/{}", base.replace(".ly", ".c"));
            let file =
                std::fs::File::create(&output_filename).expect("Failed to create output file");
            std::io::BufWriter::new(file)
        })
        .collect()
}

fn generate_c_file(filename: &str, reader: &mut Reader, writer: &mut Writer) {
    write_includes(writer);

    let mut after_imports = false;
    let mut c_code;
    let mut h_code;
    let mut header_writer = if filename == "main.ly" {
        None
    } else {
        Some(get_header_writer(filename))
    };
    let mut last_tabs = 0;

    write_header_guard(filename, &mut header_writer);

    for line in reader.lines() {
        let line = line.expect("Failed to read line from input file");
        let (tokens, tabs) = crate::lexer::get_tokens(&line);
        (c_code, h_code, after_imports) =
            crate::generator::generate(&tokens, filename, after_imports, tabs, last_tabs);

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
        .join("\n")
        + "\n";

    write!(writer, "{scope}").expect("Failed to write scope end");

    write_ending(filename, writer);
    write_header_ending(&mut header_writer);
}

fn get_header_writer(filename: &str) -> Writer {
    let base = std::path::Path::new(filename)
        .file_name()
        .expect("Invalid filename")
        .to_string_lossy();
    let header_filename = format!("build/include/{}", base.replace(".ly", ".h"));
    let file = std::fs::File::create(&header_filename).expect("Failed to create output file");
    std::io::BufWriter::new(file)
}

fn write_includes(writer: &mut Writer) {
    writeln!(
        writer,
        "#include <stdio.h>\n#include <stdlib.h>\n#include <stdbool.h>\n#include \"std.h\"\n"
    )
    .expect("Failed to write includes");
}

fn write_header_guard(filename: &str, header_writer: &mut Option<Writer>) {
    if let Some(h_writer) = header_writer {
        let guard_name = format!("{}_H", filename.trim_end_matches(".ly").to_uppercase());
        writeln!(h_writer, "#ifndef {}\n#define {}\n", guard_name, guard_name)
            .expect("Failed to write header guard");
    }
}

fn write_ending(filename: &str, writer: &mut Writer) {
    if filename == "main.ly" {
        write!(writer, "_free_memory();\nreturn EXIT_SUCCESS;\n}}")
            .expect("Failed to write main function end");
    } else {
        write!(writer, "}}").expect("Failed to write function end");
    }
}

fn write_header_ending(header_writer: &mut Option<Writer>) {
    if let Some(h_writer) = header_writer {
        write!(h_writer, "\n#endif").expect("Failed to write header end");
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
            format!("build/src/{}", base.replace(".ly", ".c"))
        })
        .collect::<Vec<_>>();

    let mut cmd = std::process::Command::new("gcc");
    cmd.arg("-Ibuild/include");
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
        .arg("build/src/std.c")
        .arg("-o")
        .arg(format!("build/{}", executable_name));

    let status = cmd.status().expect("Failed to run gcc");

    if !status.success() {
        panic!("gcc failed to compile");
    }
}
