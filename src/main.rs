mod days;

use std::env;
use std::fs::File;
use std::io::{BufReader, Write};

pub fn get_input(preview: &str) -> String {
    let mut input = String::new();
    print!("{}",preview);
    //flush output to make sure the print! is shown
    let _ = (&mut std::io::stdout()).flush();
    //read input
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    return input;
}

pub fn main() {
    //input string
    let binding = get_input("Filepath: ");
    let input = binding.trim();
    let file = File::open(input);

    if file.is_err() {
        let pwd = env::current_dir();
        if pwd.is_err() {
            println!("File {} does not exist", input);
            return;
        }
        println!("File {}/{} does not exist", pwd.expect("").to_str().expect(""), input);
        return;
    }
    //get reader for file
    let f = BufReader::new(file.expect(""));


    let input = get_input("Day: ");

    if input.trim() == "" {
        println!("No day given");
        return;
    }
    let id = input.trim().parse::<u32>();
    if id.is_err() {
        println!("{} is not a number", input.trim());
        return;
    }

    days::run(id.expect(""),f);
}
