use std::collections::HashMap;
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
            let mut columns: Vec<String> = Vec::new();
            let mut columns_to_values: HashMap<String, Vec<String>> = HashMap::new();
            let mut first_line_read = false;
            for line in reader.lines(){
                match line {
                    Ok(l) => {
                        if !first_line_read{
                            let cols: Vec<&str> = l.split(",").collect();
                            for c in cols {
                                columns.push(c.to_string());
                                columns_to_values.entry(c.to_string()).or_insert([].to_vec());
                            }
                            first_line_read = true;
                            continue;
                        }
                        let values: Vec<&str> = l.split(",").collect();
                        for (k, v) in columns.iter().zip(values.iter()).collect::<Vec<_>>() {
                            columns_to_values.entry(k.to_string()).and_modify(|c: &mut Vec<_>| c.push(v.to_string())).or_insert(vec![v.to_string()]);
                        }

                    },
                    Err(_) => {print!("Error reading a line.");}
                }
            }
            println!("{:?}", columns_to_values);
        },
        Err(e) => {
            println!("Error while opening the file: {}", e);
        }
    }
}
