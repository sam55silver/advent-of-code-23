use std::fmt::{Display, Result, Formatter};

use utilities::get_input_lines;

#[derive(Debug, PartialEq, Eq)]
enum Pipes {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

struct Map {
    map: Vec<Vec<Pipes>>,
    start: Pos
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in &self.map {
            for item in row {
                write!(f, "{:?} ", item)?; // Display each enum item
            }
            write!(f, "\n")?; // Add a new line after each row
        }
        Ok(())
    }
}

impl Map {
    fn new(raw_map: Vec<Vec<char>>) -> Map {
        let mut start = Pos { x: 0, y: 0 };
        let map: Vec<Vec<Pipes>> = raw_map.iter().enumerate().map(|(row_i, row)| {
            row.iter().enumerate().map(|(c_i, c)| {
                match c {
                    '|' => Pipes::NS,
                    '-' => Pipes::EW,
                    'L' => Pipes::NE,
                    'J' => Pipes::NW,
                    '7' => Pipes::SW,
                    'F' => Pipes::SE,
                    '.' => Pipes::Ground,
                    'S' => {
                        start = Pos { x: c_i, y: row_i };
                        Pipes::Start
                    },
                    _ => panic!("unknown pipe!")
                }
            }).collect()
        }).collect();

        Map { map, start }
    }

    fn get_pipe(&self, pos: Pos) -> Option<&Pipes> {
        match self.map.get(pos.y) {
            Some(row) => row.get(pos.x),
            None => None
        }
    }

    fn get_pos(&self, x: i64, y: i64) -> Option<Pos> {
        if x < 0 || y < 0 {
            return None;
        }

        if y >= self.map.len() as i64 {
            return None;
        }

        if x >= self.map[y as usize].len() as i64 {
            return None;
        }

        Some(Pos { x: x as usize, y: y as usize })
    }

    // TODO: Make a get_pos fn to return some(pos) only if in map boundary
    fn find_connections(&self, pos: Pos) -> Vec<Pos> {
        println!("Pos: {:?}", pos);
        let up = self.get_pos(pos.x as i64, pos.y as i64 - 1);
        let down = self.get_pos(pos.x as i64, pos.y as i64 + 1);
        let left = self.get_pos(pos.x as i64 - 1, pos.y as i64);
        let right = self.get_pos(pos.x as i64 + 1, pos.y as i64);

        let mut locs: Vec<Pos> = Vec::new();
        match up {
            Some(up) => match self.get_pipe(up) {
                Some(pipe) => {
                    match pipe {
                        Pipes::SW => locs.push(up),
                        Pipes::SE => locs.push(up),
                        Pipes::NS => locs.push(up),
                        _ => ()
                    }
                },
                None => ()
            },
            None => ()
        }
        match down {
            Some(down) => match self.get_pipe(down) {
                Some(pipe) => match pipe {
                    Pipes::NE => locs.push(down),
                    Pipes::NW => locs.push(down),
                    Pipes::NS => locs.push(down),
                    _ => ()
                },
                None => ()
            },
            None => ()
        }
        match left {
            Some(left) => match self.get_pipe(left) {
                Some(pipe) => match pipe {
                    Pipes::SE => locs.push(left),
                    Pipes::NE => locs.push(left),
                    Pipes::EW => locs.push(left),
                    _ => ()
                },
                None => ()
            },
            None =>()
        }
        match right {
            Some(right) => match self.get_pipe(right) {
                Some(pipe) => match pipe {
                    Pipes::NW => locs.push(right),
                    Pipes::SW => locs.push(right),
                    Pipes::EW => locs.push(right),
                    _ => ()
                },
                None => ()
            },
            None => ()
        }

        locs
    }
}

fn main() {
    let (lines, _part) = get_input_lines();

    let map = Map::new(lines.iter().map(|line| line.chars().collect()).collect());
    println!("{}", map);
    let postions_start = map.find_connections(map.start);
    let mut last_pos = map.start;
    let mut curr_pos = postions_start[0];
    loop {
        let pipe = map.get_pipe(curr_pos);
        println!("Pipe: {:?}", pipe.unwrap());

        let next_positions: Vec<Pos> = map.find_connections(curr_pos);
        match next_positions.iter().filter(|p| **p != last_pos).collect::<Vec<&Pos>>().first() {
            Some(forward_pos) => {
                last_pos = curr_pos;
                curr_pos = **forward_pos;
            },
            None => break
        }
       
    }
    println!("Loop complete")
}
