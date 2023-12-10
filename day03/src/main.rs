use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn create_grid(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).expect("Failed to open path: {path}");
    let reader = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let line_len = grid.get(0).unwrap().len();
    grid.insert(0, vec!['.'; line_len]);
    grid.insert(grid.len(), vec!['.'; line_len]);

    for line in &mut grid {
        line.insert(0, '.');
        line.insert(line.len(), '.');
    }

    grid
}

fn part1(real_nums: &mut Vec<u32>, grid: &Vec<Vec<char>>) {
    for i in 1..=(grid.len() - 1) {
        let mut num: Vec<usize> = Vec::new();
        for j in 1..=(grid[i].len() - 1) {
            let c = &grid[i][j];
            if c.is_digit(10) {
                num.push(j);
            } else if num.len() > 0 {
                let start = num.get(0).unwrap() - 1;
                let end = num.get(num.len() - 1).unwrap() + 1;
    
                'outer: for k in (i-1)..=(i+1) {
                    for l in start..=end {
                        if !grid[k][l].is_digit(10) && grid[k][l] != '.' {
                            let number: String = num.iter().map(|loc| &grid[i][*loc]).collect();
                            real_nums.push(number.parse::<u32>().unwrap());
                            break 'outer;
                        }
    
                    }
                }
                num.clear();
            }
        }
    }
}

fn part2(real_nums: &mut Vec<u32>, grid: &Vec<Vec<char>>) {
    for i in 1..=(grid.len() - 1) {
        for j in 1..=(grid[i].len() - 1) {
            let c = &grid[i][j];
            if *c == '*' {
                let mut cog_nums: Vec<[usize; 2]> = Vec::new();
                for k in (i-1)..=(i+1) {
                    let mut num_index: Vec<[usize; 2]> = Vec::new();
                    let mut found = false;
                    for l in (j-1)..=(j+1) {
                        if grid[k][l].is_digit(10) {
                            if !found {
                                num_index.push([k, l]);
                                found = true;
                            } 
                        } else if found {
                            found = false;
                        }
                    }

                    for nums in &num_index {
                        cog_nums.push(*nums);
                    }
                }

                if cog_nums.len() == 2 { 
                    let mut cog: Vec<u32> = Vec::new();
                    for pos in cog_nums {
                        let mut number: String = String::new();
                        // move right
                        let mut right = 0;
                        loop {
                            let c = grid[pos[0]][pos[1] + right];
                            if c.is_digit(10) {
                                number.push(c);
                                right += 1;
                            } else {
                                break;
                            }
                        }

                        // move left
                        let mut left = 1;
                        loop {
                            let c = grid[pos[0]][pos[1] - left];
                            if c.is_digit(10) {
                                number.insert(0, c);
                                left += 1;
                            } else {
                                break;
                            }
                        }

                       cog.push(number.parse().unwrap());
                    }
                    let product = cog.iter().fold(1, |acc, x| acc * x);
                    real_nums.push(product);
                }
            }
            
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Ussage: {} <path_inputs> <part: 1 || 2>", args[0]);
        std::process::exit(1);
    }

    let path_inputs = &args[1];
    let part = &args[2];

    if part != "1" && part != "2" {
        eprintln!("Ussage: {} <path_inputs> <part: 1 || 2>", args[0]);
        std::process::exit(1);
    }

    let grid = create_grid(path_inputs);

    let mut real_nums: Vec<u32> = Vec::new();

    if part == "1" {
        part1(&mut real_nums, &grid);
    } else {
        part2(&mut real_nums, &grid);
    }
    
    let sum: u32 = real_nums.iter().sum();
    println!("Sum: {}", sum);
}
