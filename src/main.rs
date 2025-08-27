use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    filename: String
}

// struct for storing column values and its attributes
#[derive(Debug)]
struct Column<T>{
    name: String,
    values: Vec<T>,
    hasnan: bool,
}

fn main() {
    println!("Running csv parser");
    let args = Args::parse();
    let file_path = args.filename;
    let file = File::open(file_path);
    match file {
        Ok(f) => {
            println!("Successfully opened the file!");
            let reader = BufReader::new(f);
            
            let mut columns: HashMap<String, Column<String>> = HashMap::new();
            let mut column_names: Vec<String> = Vec::new();
            //let mut columns_to_values: HashMap<String, Vec<String>> = HashMap::new();

            let mut first_line_read = false;
            for line in reader.lines(){
                match line {
                    Ok(l) => {
                        // parsing column names 
                        if !first_line_read{
                            // line split and collect names in a vec
                            let cols: Vec<&str> = l.split(",").collect();
                            // iterate over the vec and push to columns_to_values for key, create
                            // value of type `Column`
                            for c in cols {
                                columns.entry(c.to_string()).or_insert(Column{
                                    name: c.to_string(),
                                    values: vec![],
                                    hasnan: false,
                                });
                                column_names.push(c.to_string());
                            }
                            first_line_read = true;
                            // go to next line
                            continue;
                        }
                        // parsing values 
                        let values: Vec<&str> = l.split(",").collect();
                        for (k, v) in column_names.iter().zip(values.iter()) {
                            //columns.entry(k.to_string()).and_modify(|c: &mut Vec<_>| c.push(v.to_string())).or_insert(vec![v.to_string()]);
                            columns.entry(k.to_string()).and_modify(|col: &mut Column<String>| {
                                col.values.push(v.to_string()); 
                                if !col.hasnan{
                                    col.hasnan = v.is_empty() || v.trim().to_string().to_lowercase() == "na".to_string() || v.trim().to_string().to_lowercase() == "nan".to_string();
                                    if col.hasnan{
                                        println!("Found null value for {} {}", col.name, k);
                                        println!("{:?}", column_names);
                                        println!("{:?}", values);
                                        println!("{:?}", column_names.iter().zip(values.iter()).collect::<Vec<_>>());
                                    }
                                };
                            });
                        }

                    },
                    Err(_) => {print!("Error reading a line.");}
                }
            }
            println!("{:?}", column_names);
            for c in column_names{
                println!("{} hasnan: {}", columns[&c].name, columns[&c].hasnan);
            }

        },
        Err(e) => {
            println!("Error while opening the file: {}", e);
        }
    }
}
