use core::panic;
use std::env;

pub fn get_input_lines() {
    let args: Vec<String> = env::args().collect();
    
    let sys_exit = || {
        eprintln!("Usage: {} <part: 1 || 2> [test]", args[0]);
        std::process::exit(1);   
    };

    if args.len() != 2 {
        sys_exit();
    }

    let mut test = false;
    if args.len() == 3 {
        if args[2] == "test" {
            test = true;
        } else {
            sys_exit();
        }
    }

}

