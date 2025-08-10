use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    println!("Running csv parser");
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = File::open(file_path);
    match file {
        Ok(f) => {
            println!("Successfully opened the file!");
            let reader = BufReader::new(f);
            for line in reader.lines(){
                match line {
                    Ok(l) => {
                        println!("{}", l);
                    },
                    Err(_) => {print!("Error reading a line.");}
                }
            }
        },
        Err(e) => {
            println!("Error while opening the file: {}", e);
        }
    }
}
