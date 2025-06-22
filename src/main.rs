use clap::{command,Arg};
use std::fs;
use std::io;
use std::path::{Path,PathBuf};

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

// fn read_dir() {
//     let files = list_files().unwrap();
//     for file in files.iter() {
//         match file.as_str() {
//             f if f.ends_with("src") => {
//                 println!("Found lock file: {}", f);
//             }
//             _ => {
//                 println!("rest of files  {}", file)
//             }
//         }
//     }
// }

fn check_file_type(){
    let files = list_files().unwrap();

    for file in files.iter() {
        // basically just loop over the dir and
        // updating the path to the file  then matching for the
        // metadata of the file since we now have the total path
        // let path = Path::new(file);
        match fs::metadata(file) {
            Ok(metadata) => {
             if metadata.is_dir() {
                  println!("This is a directory {}  ", file);
             } else {
                    println!("This is a file {} ", file);
                }
            }
            Err(error) => {
              println!("Error: {}", error);
             }
        }
    }
}



fn run() {
    let input = input();
    match_and_check(Some(input));
    // read_dir();
    check_file_type();
}

fn main() {
    println!("Hello, world!");
    run();
}



