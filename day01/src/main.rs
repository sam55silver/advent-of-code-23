use std::{env, io::{self, BufRead, BufReader}, fs::File};

fn get_calibration(line: &str) -> u32 {
    let mut nums: Vec<u32> = Vec::new();

    for ch in line.chars() {
        match ch.to_digit(10) {
            Some(num) => nums.push(num),
            None => continue
        }
    }

    return (nums[0] * 10) + nums.last().unwrap();
}

fn read_file(file_name: &str) -> io::Result<Vec<u32>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let cali_vals: Vec<u32> = reader.lines()
        .map(|line| get_calibration(&line.unwrap()))
        .collect();

    Ok(cali_vals)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Ussage: {} <file_name>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];
    
    let nums: Vec<u32> = match read_file(file_name) {
        Ok(nums) => nums,
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
            std::process::exit(1);
        }
    };

    let sum: u32 = nums.iter().sum();
    println!("Calibration Values: {sum}");
}
