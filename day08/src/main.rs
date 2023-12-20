use core::panic;
use std::collections::HashMap;

use utilities::get_input_lines;

fn main() {
    let (lines, _part) = get_input_lines();
    
    let mut lines_iter = lines.into_iter();

    let instructions: Vec<u32> = lines_iter.next().unwrap().chars().map(|c| if c == 'R' { 1 } else { 0 }).collect();

    // skip empty line
    lines_iter.next().unwrap();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    loop {
        match lines_iter.next() {
            Some(line) => {
                let mut split = line.split("=");
                let title = split.next().unwrap().trim().to_string();
                let mut nodes = split.next().unwrap().trim().trim_start_matches('(').trim_end_matches(')').split(',');
                let first = nodes.next().unwrap().trim().to_string();
                let second = nodes.next().unwrap().trim().to_string();

                map.insert(title, (first, second));
            },
            None => break
        }
    }

    let mut i_pos: usize = 0;
    let mut loc: String = "AAA".to_string();
    let mut steps = 0;
    loop {
        steps += 1;

        if i_pos >= instructions.len() {
            i_pos = 0;
        }

        match map.get(&loc) {
            Some(nodes) => { 
                let new_loc = match instructions[i_pos] {
                    0 => &nodes.0,
                    1 => &nodes.1,
                    _ => panic!("invalide instruction")
                };

                if new_loc == "ZZZ" {
                    break
                }
                
                loc = new_loc.to_string();
                i_pos += 1;
            }, 
            None => panic!("Node does not exist!")
        }
    }

    println!("Steps: {}", steps)
}
