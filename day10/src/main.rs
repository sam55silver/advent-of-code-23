use std::fmt::{Display, Result, Formatter};

use utilities::get_input_lines;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Debug, Clone, Copy)]
struct PipeDirection {
    pipe: Pipes,
    dir: Direction,
    pos: Pos
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

    fn find_forward_connections(&self, pipe: PipeDirection) -> PipeDirection {
        match pipe.dir {
            Direction::East => {
                let new_pos = self.get_pos((pipe.pos.x + 1) as i64, pipe.pos.y as i64).unwrap();
                let new_pipe = self.get_pipe(new_pos).unwrap();
                match new_pipe {
                    Pipes::SW => PipeDirection { pipe: *new_pipe, dir: Direction::South, pos: new_pos },
                    Pipes::NW => PipeDirection { pipe: *new_pipe, dir: Direction::North, pos: new_pos },
                    Pipes::EW => PipeDirection { pipe: *new_pipe, dir: Direction::East, pos: new_pos },
                    Pipes::Start => PipeDirection { pipe: *new_pipe, dir: Direction::North, pos: new_pos },
                    _ => panic!("No direction!")
                }
            },
            Direction::West => {
                let new_pos = self.get_pos((pipe.pos.x - 1) as i64, pipe.pos.y as i64).unwrap();
                let new_pipe = self.get_pipe(new_pos).unwrap();
                match new_pipe {
                    Pipes::EW => PipeDirection { pipe: *new_pipe, dir: Direction::West, pos: new_pos },
                    Pipes::SE => PipeDirection { pipe: *new_pipe, dir: Direction::South, pos: new_pos },
                    Pipes::NE => PipeDirection { pipe: *new_pipe, dir: Direction::North, pos: new_pos },
                    Pipes::Start => PipeDirection { pipe: *new_pipe, dir: Direction::North, pos: new_pos },
                    _ => panic!("No direction!")
                }
            },
            Direction::North => {
                let new_pos = self.get_pos(pipe.pos.x as i64, (pipe.pos.y - 1) as i64).unwrap();
                let new_pipe = self.get_pipe(new_pos).unwrap();
                match new_pipe {
                    Pipes::SW => PipeDirection { pipe: *new_pipe, dir: Direction::West, pos: new_pos },
                    Pipes::SE => PipeDirection { pipe: *new_pipe, dir: Direction::East, pos: new_pos },
                    Pipes::NS => PipeDirection { pipe: *new_pipe, dir: Direction::North, pos: new_pos },
                    Pipes::Start => PipeDirection { pipe: *new_pipe, dir: Direction::North, pos: new_pos },
                    _ => panic!("No direction!")
                }
            },
            Direction::South => {
                let new_pos = self.get_pos(pipe.pos.x as i64, (pipe.pos.y + 1) as i64).unwrap();
                let new_pipe = self.get_pipe(new_pos).unwrap();
                match new_pipe {
                    Pipes::NE => PipeDirection { pipe: *new_pipe, dir: Direction::East, pos: new_pos },
                    Pipes::NS => PipeDirection { pipe: *new_pipe, dir: Direction::South, pos: new_pos },
                    Pipes::NW => PipeDirection { pipe: *new_pipe, dir: Direction::West, pos: new_pos },
                    Pipes::Start => PipeDirection { pipe: *new_pipe, dir: Direction::North, pos: new_pos },
                   _ => panic!("No direction!")
                }
            }
        }
        
    }

    fn find_connections(&self, pos: Pos) -> Vec<PipeDirection> {
        let up = self.get_pos(pos.x as i64, pos.y as i64 - 1);
        let down = self.get_pos(pos.x as i64, pos.y as i64 + 1);
        let left = self.get_pos(pos.x as i64 - 1, pos.y as i64);
        let right = self.get_pos(pos.x as i64 + 1, pos.y as i64);

        let mut locs: Vec<PipeDirection> = Vec::new();
        match up {
            Some(up) => match self.get_pipe(up) {
                Some(pipe) => {
                    match pipe {
                        Pipes::SW => locs.push(PipeDirection { pipe: *pipe, dir: Direction::West, pos: up}),
                        Pipes::SE => locs.push(PipeDirection { pipe: *pipe, dir: Direction::East, pos: up}),
                        Pipes::NS => locs.push(PipeDirection { pipe: *pipe, dir: Direction::North, pos: up}),
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
                    Pipes::NE => locs.push(PipeDirection { pipe: *pipe, dir: Direction::East, pos: down }),
                    Pipes::NW => locs.push(PipeDirection { pipe: *pipe, dir: Direction::West, pos: down }),
                    Pipes::NS => locs.push(PipeDirection { pipe: *pipe, dir: Direction::South, pos: down }),
                    _ => ()
                },
                None => ()
            },
            None => ()
        }
        match left {
            Some(left) => match self.get_pipe(left) {
                Some(pipe) => match pipe {
                    Pipes::NE => locs.push(PipeDirection { pipe: *pipe, dir: Direction::North, pos: left }),
                    Pipes::SE => locs.push(PipeDirection { pipe: *pipe, dir: Direction::South, pos: left }),
                    Pipes::EW => locs.push(PipeDirection { pipe: *pipe, dir: Direction::West, pos: left }),
                    _ => ()
                },
                None => ()
            },
            None =>()
        }
        match right {
            Some(right) => match self.get_pipe(right) {
                Some(pipe) => match pipe {
                    Pipes::NW => locs.push(PipeDirection { pipe: *pipe, dir: Direction::North, pos: right }),
                    Pipes::SW => locs.push(PipeDirection { pipe: *pipe, dir: Direction::South, pos: right }),
                    Pipes::EW => locs.push(PipeDirection { pipe: *pipe, dir: Direction::East, pos: right }),
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
    let postions_start = map.find_connections(map.start);
    let mut curr_pos = postions_start.first().unwrap().clone(); 
    let mut steps = 1;
    loop {
        steps += 1;
        let next_position: PipeDirection = map.find_forward_connections(curr_pos);
        if next_position.pipe == Pipes::Start {
            break;
        }
        curr_pos = next_position.clone();
    }
    println!("Furthest pipe: {:?}", steps / 2);
}
