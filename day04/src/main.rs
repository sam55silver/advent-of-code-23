use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct Card {
    winning_nums: Vec<u32>,
    listed_nums: Vec<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let mut score = 0;
        for num in &self.winning_nums {
            if self.listed_nums.contains(num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }
}

fn create_card(line: &str) -> Card {
    let split = line.split(':');
    let mut numbers = split.last().unwrap().split('|');
    let winning_nums = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let listed_nums = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    Card {
        winning_nums,
        listed_nums,
    }
}

fn read_cards(path: &str) -> Vec<Card> {
    let file = File::open(path).expect("Failed to open path: {path}");
    let reader = BufReader::new(file);

    let cards: Vec<Card> = reader
        .lines()
        .map(|line| create_card(&line.unwrap()))
        .collect();

    cards
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Ussage: {} <path_inputs>", args[0]);
        std::process::exit(1);
    }

    let path_inputs = &args[1];
    let cards = read_cards(path_inputs);

    let total_score: u32 = cards.iter().map(|card| card.score()).sum();
    println!("Total score: {}", total_score);
}
