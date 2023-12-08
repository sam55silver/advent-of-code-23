use std::{env, io::{self, BufRead, BufReader}, fs::File, u32};

fn get_digit(view: &str) -> Option<u32> {
    match view.chars().next().unwrap().to_digit(10) {
        Some(num) => Some(num),
        None => {
            for i in 0..=view.len() {
                match &view[0..i] {
                    "one" => return Some(1),
                    "two" => return Some(2),
                    "three" => return Some(3),
                    "four" => return Some(4),
                    "five" => return Some(5),
                    "six" => return Some(6),
                    "seven" => return Some(7),
                    "eight" => return Some(8),
                    "nine" => return Some(9),
                    _ => continue
                };
            }
            None
        }
    }
}

fn get_calibration(line: &str) -> u32 {
    let mut nums: Vec<u32> = Vec::new();

    for i in 0..line.len() {
        match get_digit(&line[i..]) {
            Some(num) => nums.push(num),
            None => continue,
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
    println!("Calibration: {sum}");
}
