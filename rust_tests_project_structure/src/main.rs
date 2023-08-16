use std::io;
use std::process::{Command, Output};

pub(crate) mod common;
use common::print_bytes;
use common::print_file_contents::print_file_contents;

// Dart-like way:
#[path = "lib/another_function.rs"]
mod another_function;
#[path = "lib/some_function.rs"]
mod some_function;

// Best way -
pub(crate) mod main_extended; // imports main_extended
use main_extended::main_extended;

mod my;
use my::c::ccc as cccFunction;

//
//
//

fn main() {
    match compile_and_run_script("./script.rs") {
        Ok(output) => println!("Script output:\n{}", output),
        Err(e) => eprintln!("{}", e),
    }

    // cccFunction();

    // let content_str = include_str!("../rust_notes.yaml");
    // println!("{}", content_str);

    // let content_bytes = include_bytes!("../rust_notes.yaml");
    // //common::print_bytes(content_bytes);
    // print_bytes(content_bytes);

    // let result = print_file_contents("rust_notes.yaml");

    // some_function::some_function();
    // another_function::another_function();

    // main_extended::main_extended();
    // main_extended();
}

//
//
//

fn compile_and_run_script(script_path: &str) -> Result<String, String> {
    if let Err(e) = compile_script(script_path) {
        return Err(format!("Error compiling script: {}", e));
    }

    // Assuming the script's binary has the same name without the ".rs" extension
    let binary_name = script_path.trim_end_matches(".rs");
    match run_binary(binary_name, &["33", "77"]) {
        Ok(output) => Ok(output),
        Err(e) => Err(format!("Error executing binary: {}", e)),
    }
}

fn compile_script(script_path: &str) -> io::Result<()> {
    let compile_status = Command::new("rustc").arg(script_path).status()?;

    if !compile_status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to compile the Rust script.",
        ));
    }
    Ok(())
}

fn run_binary(binary_name: &str, args: &[&str]) -> io::Result<String> {
    let output: Output = Command::new(binary_name).args(args).output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Binary exited with a non-zero status code. STDERR: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
