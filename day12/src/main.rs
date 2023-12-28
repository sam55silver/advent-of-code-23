use utilities::get_input_lines;

fn get_count(springs: &str, nums: &str) -> u64 {
    let vec_nums: Vec<usize> = nums.chars().map(|c| c as usize).collect();

    if springs == "" {
        match vec_nums.len() {
            0 => return 1,
            _ => return 0
        }
    }

    if vec_nums.len() == 0 {
        if springs.contains('#') { return 0 } else { return 1 }
    }

    let mut res = 0;

    if springs.chars().nth(0).unwrap() == '.' || springs.chars().nth(0).unwrap() == '?' {
        res += get_count(&springs[1..springs.len()], &nums);
    }

    if springs.chars().nth(0).unwrap() == '#' || springs.chars().nth(0).unwrap() == '?' {
        if vec_nums[0] <= springs.len() && !springs[..vec_nums[0]].contains('.') && (vec_nums[0] == springs.len() || springs.chars().nth(vec_nums[0]).unwrap() != '#') {
            res += get_count(&springs[vec_nums[0] + 1..], &nums[1..])
        }
    }

    return res
}

fn main() {
    let (line, _part) = get_input_lines();

    let mut sum: u64 = 0;
    line.iter().for_each(|line| {
        let mut split_line = line.split_whitespace();
        let springs = split_line.next().unwrap();
        let nums = split_line.next().unwrap();
        sum += get_count(&springs, &nums);
    });
    
    println!("Sum: {}", sum);
}
