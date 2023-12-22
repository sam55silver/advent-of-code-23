use utilities::get_input_lines;

fn get_diff(history: &Vec<i64>, part: u32) -> i64 {
    let mut new_history: Vec<i64> = Vec::new();
    for i in 1..history.len() {
        let last = history[i-1];
        let curr = history[i];
        let diff = curr - last;
        new_history.push(diff);
    }

    if new_history.iter().filter(|i| **i != 0).count() > 0 {
        let incr = get_diff(&new_history, part);
        if part == 1 {
            return new_history.last().unwrap() + incr;
        } else {
            return new_history.first().unwrap() - incr; 
        }
    } else {
        return 0
    }
}

fn main() {
    let (lines, part) = get_input_lines();

    let mut line_iter = lines.into_iter();

    let mut added_history: i64 = 0;
    loop {
        match line_iter.next() {
            Some(line) => {
                let history: Vec<i64> = line.split_whitespace().map(|n| n.parse().unwrap()).collect(); 
                let incr = get_diff(&history, part);
                let fin: i64 = if part == 1 { 
                    history.last().unwrap() + incr
                } else {
                    history.first().unwrap() - incr
                };

                added_history += fin;
            },
            None => break
        }
    }

    println!("History Values added: {:?}", added_history);
}
