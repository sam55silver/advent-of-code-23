use core::panic;
use std::{env, fs::File, io::{BufReader, BufRead}};

fn read_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Failed to open path: {path}");
    let reader = BufReader::new(file);

    match reader.lines().collect() {
        Ok(lines) => lines,
        Err(e) => panic!("Error reading lines in {}, Error: {:?}", path, e)
    }
}

pub fn get_input_lines() -> (Vec<String>, u32) {
    let args: Vec<String> = env::args().collect();
    
    let sys_exit = || {
        eprintln!("Usage: {} <part: 1 || 2> [test || test2]", args[0]);
        std::process::exit(1);   
    };

    if args.len() < 2 {
        sys_exit();
    }

    if args.len() == 3 {
        if args[2] == "test" {
            return (read_file("test-input.txt"), args[1].parse().unwrap())
        } else if args[2] == "test2" {
            return (read_file("test-input2.txt"), args[1].parse().unwrap())
        } else {
            sys_exit();
        }
    }

    (read_file("input.txt"), args[1].parse().unwrap())
}

