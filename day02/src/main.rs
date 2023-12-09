use std::{env, io::{self, BufRead, BufReader}, fs::File, u32};

#[derive(Debug)]
struct SubGame {
    red: u32,
    green: u32,
    blue: u32,
}

impl SubGame {
    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Game {
    id: u32,
    sub_games: Vec<SubGame>,
}

impl Game {
    fn max_check(&self, constraint: &SubGame) -> Option<u32> {
        for sub_game in &self.sub_games {
            if sub_game.red > constraint.red || sub_game.green > constraint.green || sub_game.blue > constraint.blue {
                return None
            }
        }

        Some(self.id)
    }

    fn get_power(&self) -> u32 {
        let mut min_game = SubGame {
            red: 0,
            green: 0,
            blue: 0,
        };

        for sub_game in &self.sub_games {
            if sub_game.red > min_game.red {
                min_game.red = sub_game.red;
            }

            if sub_game.green > min_game.green {
                min_game.green = sub_game.green;
            }

            if sub_game.blue > min_game.blue {
                min_game.blue = sub_game.blue;
            }
        }

        min_game.power()
    }
}

fn get_game(line: &str) -> Game {
    let mut split = line.split(':');
    let id = split.next().unwrap().get(5..).unwrap().parse::<u32>().unwrap();

    let mut sub_games: Vec<SubGame> = Vec::new();
    for games in split.next().unwrap().split(';') {
        let mut sub_game = SubGame {
            red: 0,
            green: 0,
            blue: 0,
        };

        for game in games.split(',') {
            let mut split = game.trim().split_whitespace();
            let num = split.next().unwrap().parse::<u32>().unwrap();
            let color = split.next().unwrap();

            match color {
                "red" => sub_game.red = num,
                "green" => sub_game.green = num,
                "blue" => sub_game.blue = num,
                _ => panic!("Invalid color: {}", color),
            }
        }

        sub_games.push(sub_game);
    }

    Game {
        id,
        sub_games,
    }
}

fn read_file(file_name: &str) -> io::Result<Vec<Game>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let games: Vec<Game> = reader.lines()
        .map(|line| get_game(&line.unwrap()))
        .collect();

    Ok(games)
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
        eprintln!("Invalid part, parts = 1 || 2");
        std::process::exit(1);
    }
    
    let games = read_file(path_inputs).unwrap();
    
    if part == "1" {
        let constraint = SubGame {
            red: 12,
            green: 13,
            blue: 14,
        };

        let ids: Vec<u32> = games.iter()
            .filter_map(|game| game.max_check(&constraint))
            .collect();

        let sum_ids = ids.iter().sum::<u32>();
        println!("Sum IDs: {}", sum_ids);
    }
    else if part == "2" {
        let powers: Vec<u32> = games.iter()
            .map(|game| game.get_power())
            .collect();

        let sum_powers = powers.iter().sum::<u32>();
        println!("Sum Powers: {}", sum_powers);
    }
}
