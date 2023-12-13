use utilities::get_input_lines;

#[derive(Debug)]
struct Mapping {
    start: i64,
    end: i64,
    diff: i64,
}

#[derive(Debug)]
struct Map {
    mappings: Vec<Mapping>
}

impl Map {
    fn get_output(&self, input: i64) -> i64 {
        for map in &self.mappings {
            if (map.start..map.end).contains(&input) {
               return input - map.diff  
            }
        }

        input
    }
}

fn main() {
    let (lines, part) = get_input_lines();
    
    let mut lines_iter = lines.into_iter();

    let seeds_input: Vec<i64> = lines_iter
        .next().unwrap()
        .split(":")
        .last().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let seeds: Vec<i64> = match part {
        1 => seeds_input,
        2 => seeds_input
            .chunks(2)
            .filter(|chunck| chunck.len() == 2)
            .flat_map(|chunck| (chunck[0]..(chunck[0] + chunck[1])).collect::<Vec<i64>>())
            .collect(), 
        _ => panic!("No part: {part}")
    };

    println!("Done getting seeds");

    let mut maps: Vec<Map> = Vec::new();
    
    // get rid of first while space line
    lines_iter.next().unwrap();
    'outer: loop {
        // Get rid of plain text descirbing mapping
        lines_iter.next().unwrap();

        let mut map: Vec<Mapping> = Vec::new();
        loop {
            let line: Vec<i64> = match lines_iter.next() {  
                Some(l) => {
                    if l.len() == 0 {
                        break;
                    }

                    l.split_whitespace().map(|i| i.parse().unwrap()).collect()
                },
                None => {
                    maps.push(Map { mappings: map});
                    break 'outer
                }, 
            };

            map.push(
                Mapping { start: line[1], end: line[1] + line[2], diff: line[1] - line[0] }
            )
        }
        maps.push(Map { mappings: map})
    }
    println!("Done getting outputs");

    let mut outputs: Vec<i64> = Vec::new();
    for seed in seeds {
        let mut output = seed;
        for map in &maps {
            output = map.get_output(output);
        }
        outputs.push(output)
    }

    let lowest = outputs.iter().min().unwrap();
    println!("Lowest location: {lowest}")
}
