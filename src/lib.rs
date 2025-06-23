
use clap::{command,Arg};
use std::fs;
use std::io;
use colored::*;

pub fn input() -> bool {
    let matches = command!()
        .arg(
            Arg::new("input")
            .short('k')
            .long("force")
         .help("your help input")
         .action(clap::ArgAction::SetTrue),
    )
    .get_matches();

    let input = matches.get_flag("input");
    input
    

}




pub fn list_files() -> io::Result<Vec<String>> {
    fs::read_dir(".")?
        .map(|res|res.map(|entry|entry.path().display().to_string()))
        .collect()
}

pub fn check_file_type(){
    let files = list_files().unwrap();

    for file in files.iter()
        .filter(|line| !line.contains("/.")){
        // maybe change to foreach
        match fs::metadata(file) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    let size = metadata.len();
                    println!("{}", format!("DIR- {} SIZE- {}", file,size).blue());
                } else if metadata.is_symlink() {
                    println!("{}", format!("SYM{}",file).green());
                } else {
                    let size = metadata.len();
                    println!("{}", format!("FILE{} SIZE- {}",file,size).red());
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}


pub fn find_hid() {
    let files = match list_files() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error listing hidden files: {}", e);
            return;
        }
    };

    // dont use enumerate when filtering it fucks everythign up
    
    
    let hidden_files: Vec<&String> = files
        .iter()
        .filter(|file_path| file_path.contains("/.")) //
        .collect();


    
    for file_path in hidden_files {
        match fs::metadata(file_path) {
            Ok(metadata) => {
                let size = metadata.len();
                println!("{}", format!("HID {} SIZE- {}", file_path,size).bright_yellow());
            }
            Err(error) => {
                println!("Error {}",error);
            }
        }
    }
}
  
// if file is hidden (has a ./. and is a dir then its hidden
// make it so you put a flag like -h and then it it will
// show the hidden files

// make it so the file size is also a flag 



// to run use cargo run -- -k 
//
// make a match statement if the k flag is used then run a fuction with finds hidden if not just
// normal file type check

pub fn run() {
    if input() {
        find_hid();
    }
    
    
    check_file_type();
}
