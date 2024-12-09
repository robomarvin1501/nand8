use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

mod compiler;
mod instructions;
mod parser;

static VM_FILE_EXTENSION: &'static str = "vm";
static ASM_FILE_EXTENSION: &'static str = "asm";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid usage, please use: VMTranslator <input path>")
    }
    let argument_path = env::args().nth(1).expect("No path provided");
    let argument_path = fs::canonicalize(&argument_path).expect("Invalid path provided");
    let files_to_compile: Vec<PathBuf> = if argument_path.is_dir() {
        fs::read_dir(&argument_path)
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect()
    } else {
        vec![argument_path]
    };

    for input_path in files_to_compile {
        if let Some(extension) = input_path.extension() {
            if extension.to_str().unwrap_or("").to_lowercase() != VM_FILE_EXTENSION {
                continue;
            }
        } else {
            continue;
        }

        let output_path = input_path.with_extension(ASM_FILE_EXTENSION);

        let input_file = PathBuf::from(input_path);
        let output_file = PathBuf::from(output_path);

        compile_file(input_file, output_file);
    }
}

fn compile_file(input_path: PathBuf, output_path: PathBuf) {
    let contents: String =
        fs::read_to_string(input_path).expect("Should have been able to read file");
    let lines: Vec<String> = contents.split("\n").map(|s| s.trim().to_string()).collect();

    let instructions = parser::parse(lines);

    let file_name = &output_path.file_stem().unwrap().to_str().unwrap();
    let compiled_asm = compiler::compile(instructions, file_name);

    let mut file = File::create(output_path).unwrap();
    for line in compiled_asm {
        file.write(line.as_bytes()).unwrap();
    }
}
