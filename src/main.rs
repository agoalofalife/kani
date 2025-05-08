mod token;
mod scanner;

use std::fmt::{Display};
use std::fs;
use std::{env, fmt};
use crate::scanner::Scanner;

fn main() {
    let mut has_error = false;

    let args: Vec<String> = env::args().collect();

    println!("{}", args[0]);
    if args.len() == 1 {
        eprintln!("Need to choose path to source");
    }
    let file_path = &args[1];
    let text_content = fs::read_to_string(file_path).expect("Can't read file!");

    let mut scanner = Scanner::new();
    scanner.scan_tokens(&text_content);

    println!("{:#?}", scanner)
}

fn error(line: i32, message: &str) {
    report(line, "", message);
}

fn report(line: i32, location: &str, message: &str) {
    eprintln!("[line {}] Error {} : {}", line, location, message);
}
