use std::{env, io::{self, BufRead, BufReader}, fs::File};


fn get_calibration(line: &str) -> u32 {
    let mut nums: Vec<u32> = Vec::new();
    let mut word = String::new();

    for ch in line.chars() {
        // If a num then add to list of nums
        // If not, add it to
        match ch.to_digit(10) {
            Some(num) => {
                nums.push(num);
                word = String::new();
            },
            None => word.push(ch)
        }
        
        if word.contains("one") {
            nums.push(1);
            word = String::new();
        } 
        else if word.contains("two") { 
            nums.push(2);
            word = String::new();
        } 
        else if word.contains("three") { 
            nums.push(3);
            word = String::new();
        } 
        else if word.contains("four") {
            nums.push(4);
            word = String::new();
        } 
        else if word.contains("five") { 
            nums.push(5);
            word = String::new();
        } 
        else if word.contains("six") { 
            nums.push(6);
            word = String::new();
        } 
        else if word.contains("seven") { 
            nums.push(7);
            word = String::new();
        } 
        else if word.contains("eight") {
            nums.push(8);
            word = String::new();

        } 
        else if word.contains("nine") { 
            nums.push(9);
            word = String::new();
        }; 
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

    for num in &nums {
        println!("{num}");
    }

    let sum: u32 = nums.iter().sum();
    println!("Calibration Values: {sum}");
}
