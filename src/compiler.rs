use std::io::BufRead;
use std::io::Write;

use crate::constants::ENTRY_FILENAME;

pub fn compile(filenames: &Vec<String>, executable_name: &str) {
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
    create_executable(executable_name);
}

fn get_readers(filenames: &Vec<String>) -> Vec<std::io::BufReader<std::fs::File>> {
    filenames
        .iter()
        .map(|filename| {
            let file = std::fs::File::open(filename).expect("Failed to open input file");
            std::io::BufReader::new(file)
        })
        .collect()
}

fn get_writers(filenames: &Vec<String>) -> Vec<std::io::BufWriter<std::fs::File>> {
    filenames
        .iter()
        .map(|filename| {
            let output_filename = filename.replace(crate::constants::EXTENSION, ".c");
            let file =
                std::fs::File::create(&output_filename).expect("Failed to create output file");
            std::io::BufWriter::new(file)
        })
        .collect()
}

fn generate_c_file(
    filename: &str,
    input_reader: &mut std::io::BufReader<std::fs::File>,
    output_writer: &mut std::io::BufWriter<std::fs::File>,
) {
    write_includes(output_writer);
    let mut after_imports = false;

    for line in input_reader.lines() {
        let line = line.expect("Failed to read line from input file");
        let tokens = crate::lexer::get_tokens(&line);
        let output = crate::generator::generate(&tokens, filename, after_imports);
        let generated_code = output.0;
        after_imports = output.1;

        writeln!(output_writer, "{}", generated_code).expect("Failed to write to output file");
    }

    if filename == ENTRY_FILENAME {
        writeln!(output_writer, "return 0;\n}}").expect("Failed to write main function");
    }
}

fn write_includes(output_writer: &mut std::io::BufWriter<std::fs::File>) {
    writeln!(output_writer, "#include <stdio.h>").expect("Failed to write includes");
    writeln!(output_writer, "#include <stdlib.h>").expect("Failed to write includes");
}

fn flush_writers(writers: &mut Vec<std::io::BufWriter<std::fs::File>>) {
    for writer in writers.iter_mut() {
        writer.flush().expect("Failed to flush output file");
    }
}

fn create_executable(executable_name: &str) {
    let status = std::process::Command::new("gcc")
        .arg("main.c")
        .arg("-o")
        .arg(executable_name)
        .status()
        .expect("Failed to run gcc");

    if !status.success() {
        panic!("gcc failed to compile");
    }
}
