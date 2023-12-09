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

    for i in 1..(grid.len() - 1) {
        let mut num: Vec<usize> = Vec::new();
        for j in 1..(grid[i].len() - 1) {
            let c = &grid[i][j];
            if c.is_digit(10) {
                num.push(j);
            } else if num.len() > 0 {
                let number: String = num.iter().map(|loc| &grid[i][*loc]).collect();
                println!("{number}");
                num.clear();
            }
        }
    }
}
