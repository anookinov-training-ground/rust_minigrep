// i.e. cargo run args1 args2
// i.e. cargo run > output.txt

use std::env;
// use std::env::args;
// use std::env::args_os; // accept arguments containing invalid Unicode
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}