// i.e. cargo run args1 args2

use std::env;
// use std::env::args;
// use std::env::args_os; // accept arguments containing invalid Unicode

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
