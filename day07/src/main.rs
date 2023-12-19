use utilities::get_input_lines;

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<char>,
    bet: u64,
    kind: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind == other.kind {
            self.cards.iter().for_each(|c| {
                other.cards.iter().for_each(|o| {
                    match 
                })
            })
        } else {
            
        }
    }
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
        }
        3 => {
            if char_count.values().any(|&x| x == 3) {
                HandType::Three
            } else {
                HandType::Two
            }
        }
        4 => HandType::One,
        5 => HandType::High,
        _ => panic!("Invalid hand"),
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
                kind,
            }
        })
        .collect();

    let type1 = HandType::Five;
    let type2 = HandType::High;

    println!("{:?}", type1 > type2);
}
