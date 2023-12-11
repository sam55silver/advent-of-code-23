use std::{
    env,
    fs::File,
    io::{BufRead, BufReader}, time::Instant,
};

#[derive(Clone)]
#[derive(Debug)]
struct Card {
    id: u32,
    winning_nums: Vec<u32>,
    listed_nums: Vec<u32>,
}

impl Card {
    fn num_wins(&self) -> Vec<bool> {
        self.winning_nums
            .iter()
            .map(|&num| self.listed_nums.contains(&num))
            .collect()
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        self.num_wins()
            .iter()
            .for_each(|win| if *win {
                if score == 0 { score = 1}
                else { score *= 2}
            }); 
        
        score
    }

}

fn create_card(line: &str) -> Card {
    let mut split = line.split(':');
    let id: u32 = split.next().unwrap().get(5..).unwrap().trim().parse().unwrap();
    let mut numbers = split.next().unwrap().split('|');
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
        id,
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

fn find_copies(cards: &Vec<Card>, id: u32) -> Vec<u32> {
    let card = &cards[(id - 1) as usize];
    let mut copies = vec![id];
    let mut match_id = id;
    for num in &card.winning_nums {
        if card.listed_nums.contains(num) {
            match_id += 1;
            copies.extend(find_copies(cards, match_id))
        }
    }
    return copies;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Ussage: {} <path_inputs> <part: 1 || 2>", args[0]);
        std::process::exit(1);
    }

    let path_inputs = &args[1];
    let part = &args[2];
    let cards = read_cards(path_inputs);

    let start_time = Instant::now();

    if part == "1" {
        let total_score: u32 = cards.iter().map(|card| card.score()).sum();
        println!("Total score: {}", total_score);
    }
    else if part == "2" {
        let mut total_copies: Vec<u32> = Vec::new();
        for card in &cards {
            total_copies.extend(find_copies(&cards, card.id));
        }
        println!("{:?}", total_copies.len());
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {} seconds, {} milliseconds", elapsed_time.as_secs(), elapsed_time.as_millis());
}
