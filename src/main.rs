use std::collections::HashMap;
use std::fs::File;
use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

mod asm_templates;
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
    let output_path = create_vm_file_path(&argument_path).unwrap();

    let files_to_compile: Vec<PathBuf> = if argument_path.is_dir() {
        fs::read_dir(&argument_path)
            .expect("Failed to read directory")
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect()
    } else {
        vec![argument_path]
    };

    let _ = File::create(&output_path).unwrap(); // Wipe the file if it exists

    compile_files(files_to_compile, &output_path);
}

fn compile_files(input_paths: Vec<PathBuf>, output_path: &PathBuf) {
    let bootstrap = compiler::create_bootstrap_code();
    append_to_file(output_path, vec![bootstrap]);

    let mut function_calls: HashMap<String, u16> = HashMap::new();
    for input_path in input_paths {
        if let Some(extension) = input_path.extension() {
            if extension.to_str().unwrap_or("").to_lowercase() != VM_FILE_EXTENSION {
                continue;
            }
        } else {
            continue;
        }

        let input_file = PathBuf::from(input_path);

        compile_file(input_file, output_path, &mut function_calls);
    }
}

fn compile_file(input_path: PathBuf, output_path: &PathBuf, function_calls: &mut HashMap<String, u16>) {
    let contents: String =
        fs::read_to_string(&input_path).expect("Should have been able to read file");
    let lines: Vec<String> = contents.split("\n").map(|s| s.trim().to_string()).collect();

    let file_name = &input_path.file_stem().unwrap().to_str().unwrap();
    let instructions = parser::parse(lines, function_calls);

    let compiled_asm = compiler::compile(instructions, file_name);

    append_to_file(output_path, compiled_asm);
}

fn append_to_file(path: &PathBuf, s: Vec<String>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();
    for line in s {
        file.write(line.as_bytes()).unwrap();
    }
}

fn create_vm_file_path(input: &Path) -> Result<PathBuf, String> {
    if input.is_file() {
        // Input is a file, change its extension to VM_FILE_EXTENSION
        let mut new_file_path = input.to_path_buf();
        new_file_path.set_extension(ASM_FILE_EXTENSION);
        Ok(new_file_path)
    } else if input.is_dir() {
        // Input is a directory, create a file named after the directory with the new extension
        let dir_name = input
            .file_name()
            .ok_or("Failed to extract directory name")?;
        let mut new_file_path = input.to_path_buf();
        new_file_path.push(dir_name);
        new_file_path.set_extension(ASM_FILE_EXTENSION);
        Ok(new_file_path)
    } else {
        Err(format!(
            "Input path {:?} is neither a file nor a directory",
            input
        ))
    }
}

