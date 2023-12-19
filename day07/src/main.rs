use utilities::get_input_lines;

#[derive(Debug, PartialOrd, Eq, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, // 11
    Queen, // 12
    King, // 13
    Ace, // 14
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Card::Two => match other {
                Card::Two => true,
                _ => false,
            },
            Card::Three => match other {
                Card::Three => true,
                _ => false,
            },
            Card::Four => match other {
                Card::Four => true,
                _ => false,
            },
            Card::Five => match other {
                Card::Five => true,
                _ => false,
            },
            Card::Six => match other {
                Card::Six => true,
                _ => false,
            },
            Card::Seven => match other {
                Card::Seven => true,
                _ => false,
            },
            Card::Eight => match other {
                Card::Eight => true,
                _ => false,
            },
            Card::Nine => match other {
                Card::Nine => true,
                _ => false,
            },
            Card::Ten => match other {
                Card::Ten => true,
                _ => false,
            },
            Card::Jack => match other {
                Card::Jack => true,
                _ => false,
            },
            Card::Queen => match other {
                Card::Queen => true,
                _ => false,
            },
            Card::King => match other {
                Card::King => true,
                _ => false,
            },
            Card::Ace => match other {
                Card::Ace => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug, PartialOrd, Ord, Eq)]
enum HandType {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            HandType::High => match other {
                HandType::High => true,
                _ => false,
            },
            HandType::One => match other {
                HandType::One => true,
                _ => false,
            },
            HandType::Two => match other {
                HandType::Two => true,
                _ => false,
            },
            HandType::Three => match other {
                HandType::Three => true,
                _ => false,
            },
            HandType::Full => match other {
                HandType::Full => true,
                _ => false,
            },
            HandType::Four => match other {
                HandType::Four => true,
                _ => false,
            },
            HandType::Five => match other {
                HandType::Five => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bet: u64,
    kind: HandType,
}

impl Hand {
    fn new(cards: Vec<char>, bet: u64) -> Hand {
        let kind = get_kind(&cards);
        let cards = cards.iter().map(|c| get_card(*c)).collect();
        Hand { cards, bet, kind }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind == other.kind {
            for c in 0..self.cards.len() {
                match self.cards[c].cmp(&other.cards[c]) {
                    std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                }
            }
            return std::cmp::Ordering::Equal;
        } else {
            return self.kind.cmp(&other.kind);
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_card(card: char) -> Card {
    match card {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Invalid card"),
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
    let mut hands: Vec<Hand> = line
        .iter()
        .map(|line| {
            let mut line_div = line.split_whitespace();
            let cards = line_div.next().unwrap().chars().collect();
            let bet = line_div.next().unwrap().parse().unwrap();
            Hand::new(cards, bet)
        })
        .collect();

    hands.sort();
    
    let mut winnings = 0;
    for i in 0..hands.len() {
        let rank = i + 1;
        let hand = &hands[i];
        winnings += hand.bet * rank as u64; 
    }

    println!("{}", winnings);
}
