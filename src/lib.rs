use clap::{command,Arg};
use std::fs;
use std::io;
use colored::*;

pub fn input() -> String {
    let matches = command!()
        .arg(Arg::new("input").help("your help input").required(true))
        .get_matches();

    let input = matches
        .get_one::<String>("input")
        .expect("input is required")
        .trim();
    input.to_string()
}

pub fn match_and_check(input: Option<String>) {
    match input {
        Some(input) => {
            println!("Here was your input {:?} ", input);
        },
        None => {
            println!("No input was given");
        }
    }
}

pub fn list_files() -> io::Result<Vec<String>> {
    fs::read_dir(".")?
        .map(|res|res.map(|entry|entry.path().display().to_string()))
        .collect()
}

pub fn check_file_type(){
    let files = list_files().unwrap();

    for file in files.iter() {
        // maybe change to foreach
        match fs::metadata(file) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    println!("{}", format!("DIR -{}", file).red());
                } else if metadata.is_symlink() {
                    println!("{}", format!("SYM{}",file).blue());
                } else {
                    println!("{}", format!("FILE{}",file).green());
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}


pub fn run() {
    let input = input();
    match_and_check(Some(input));
    check_file_type();
}