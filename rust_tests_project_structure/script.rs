#!/usr/bin/env run-cargo-script

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Please provide exactly two integer arguments.");
        std::process::exit(1);
    }

    let num1: i32 = match args[1].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Failed to parse the first argument as an integer.");
            std::process::exit(2);
        }
    };

    let num2: i32 = match args[2].parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Failed to parse the second argument as an integer.");
            std::process::exit(3);
        }
    };

    let sum = num1 + num2;
    println!("{}", sum);
}
