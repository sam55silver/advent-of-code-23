use std::{env, io::{BufReader, BufRead}}

fn read_file(path: &str) {
    let file = File::open(path).expect("Failed to open path: {path}");
    let reader = BufReader::new(file);
    
    let inputs:
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <path_inputs> <part: 1 || 2>", args[0]);
        std::process::exit(1);
    }
}
