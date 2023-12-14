use utilities::get_input_lines;

#[derive(Debug)]
#[derive(PartialEq)]
enum HandType {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bet: u64,
    kind: HandType
}

fn get_kind(cards: &Vec<char>) -> HandType {
    let mut char_count = std::collections::HashMap::new();

    for card in cards {
        let count = char_count.entry(card).or_insert(0);
        *count += 1;
    }

    let kind = match char_count.len() {
        1 => HandType::Five,
        2 => {
            if char_count.values().any(|&x| x == 4) {
                HandType::Four
            } else {
                HandType::Full
            }
        },
        3 => {
            if char_count.values().any(|&x| x == 3) {
                HandType::Three
            } else {
                HandType::Two
            }
        },
        4 => HandType::One,
        5 => HandType::High,
        _ => panic!("Invalid hand")
    };  

    kind
}

fn main() {
    let (line, _part) = get_input_lines();
    let hands: Vec<Hand> = line
        .iter()
        .map(|line| {
            let mut line_div = line.split_whitespace();
            let cards = line_div.next().unwrap().chars().collect();
            let kind = get_kind(&cards);
            Hand {
                cards, 
                bet: line_div.next().unwrap().parse().unwrap(),
                kind
            }
        })
        .collect();

    println!("{:?}", hands);
}
