use std::{
    env,
    fs::File,
    io::{BufRead, BufReader}, time::Instant, u32,
};

#[derive(Clone)]
#[derive(Debug)]
struct Card {
    wins: u32,
    score: u32,
}

fn create_card(line: &str) -> Card {
    let mut numbers = line.split(':').last().unwrap().split('|');

    let winning_nums: Vec<u32> = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let listed_nums: Vec<u32> = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let wins: u32 = winning_nums
        .iter()
        .map(|&num| listed_nums.contains(&num))
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap();
    
    let score: u32 = wins.pow(2);

    Card {
        wins,
        score,
    }
}

fn read_cards(path: &str) -> (Vec<Card>, u32) {
    let file = File::open(path).expect("Failed to open path: {path}");
    let reader = BufReader::new(file);

    let mut max_wins: u32 = 0;
    let cards: Vec<Card> = reader
        .lines()
        .map(|line| {
            let card = create_card(&line.unwrap());
            if card.wins > max_wins { max_wins = card.wins }
            card
        })
        .collect();

    (cards, max_wins)
}

fn circular_buff(tickets: &Vec<Card>, max_wins: u32) -> u32 {
    let mut total_tickets: u32 = 0;

    // Highest num of wins will only ever be max_wins
    // Therefor make buffer max_wins + 1 so you can store current card's number of wins and the cards after that
    // Even if a ticket does not win, it itself counts as one, therefor make buff all 1's
    let mut buff: Vec<u32> = vec![1; (max_wins + 1) as usize];

    for card in tickets {
        for i in 1..=card.wins {
            // Add the current amount of wins to the next
            // sequential numbers
            buff[i as usize] += buff[0];
        }

        // Pop off current card and add to total ticket count
        total_tickets += buff[0];

        buff.push(1);
        buff.remove(0);
    }

    total_tickets
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Ussage: {} <path_inputs> <part: 1 || 2>", args[0]);
        std::process::exit(1);
    }

    let path_inputs = &args[1];
    let part = &args[2];

    let start_time = Instant::now();

    let (cards, max_wins) = read_cards(path_inputs);

    if part == "1" {
        let total_score: u32 = cards.iter().map(|card| card.score).sum();
        println!("Total score: {}", total_score);
    }
    else if part == "2" {
        let total_tickets = circular_buff(&cards, max_wins);
        println!("Total tickets: {}", total_tickets); 
    }

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {} seconds, {} milliseconds", elapsed_time.as_secs(), elapsed_time.as_millis());
}
