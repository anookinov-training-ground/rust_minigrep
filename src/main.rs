// i.e. cargo run args1 args2

use std::env;
// use std::env::args;
// use std::env::args_os; // accept arguments containing invalid Unicode
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}