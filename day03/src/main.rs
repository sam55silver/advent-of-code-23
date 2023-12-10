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

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Ussage: {} <path_inputs>", args[0]);
        std::process::exit(1);
    }

    let path_inputs = &args[1];
    let grid = create_grid(path_inputs);

    let mut real_nums: Vec<u32> = Vec::new();
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
    
    let sum: u32 = real_nums.iter().sum();
    println!("Sum: {}", sum);
}
