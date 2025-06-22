use clap::{command,Arg};
use std::fs;
use std::io;

fn input() -> String {
    let matches = command!()
        .arg(Arg::new("input").help("your help input").required(true))
        .get_matches();

    let input = matches
        .get_one::<String>("input")
        .expect("input is required")
        .trim();
    input.to_string()
}

fn match_and_check(input: Option<String>) {
    match input {
        Some(input) => {
            println!("Here was your input {:?} ", input);
        },
        None => {
            println!("No input was given");
        }
    }
}


fn list_files() -> io::Result<Vec<String>> {
    fs::read_dir(".")?
        .map(|res|res.map(|entry|entry.path().display().to_string()))
        .collect()
}

fn read_dir() {
    let files = list_files().unwrap();
    for file in files.iter() {
        match file.as_str() {
            f if f.ends_with("src") => {
                println!("Found lock file: {}", f);
            }
            _ => {
                println!("rest of files  {}", file)
            }
        }
    }

}



fn run() {
    let input = input();
    match_and_check(Some(input));
    read_dir();
}

fn main() {
    println!("Hello, world!");
    run();
}



