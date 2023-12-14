use utilities::get_input_lines;

fn line_to_vec(line: String) -> Vec<u64> {
    line.split(":").last().unwrap().split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn line_to_number(line: String) -> u64 {
    line.split(":").last().unwrap().split_whitespace().collect::<Vec<&str>>().concat().parse().unwrap()
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn wins(&self) -> u64 {
        let mut wins: u64 = 0;
        for hold_time in 1..self.time {
            let time_left = self.time - hold_time;
            // distance = (1 mm)(hold_time ms) / 1 ms
            let distance = time_left * hold_time;
            if distance > self.distance {
                wins += 1;
            }
        }
        wins
    }
}

fn main() {
    let (lines, part) = get_input_lines();
    let mut iter_lines = lines.into_iter();

    if part == 1 {
        let time: Vec<u64> = line_to_vec(iter_lines.next().unwrap()); 
        let distance: Vec<u64> = line_to_vec(iter_lines.next().unwrap()); 

        let races: Vec<Race> = time.into_iter().zip(distance.into_iter()).map(|(time, distance)| Race { time, distance }).collect();
        let num_wins: u64 = races.iter().map(|race| race.wins()).product(); 
        println!("{num_wins}");
    } else {
        let time: u64 = line_to_number(iter_lines.next().unwrap()); 
        let distance: u64 = line_to_number(iter_lines.next().unwrap()); 

        let race = Race { time, distance };
        let num_wins: u64 = race.wins(); 
        println!("{num_wins}");
    }
}
