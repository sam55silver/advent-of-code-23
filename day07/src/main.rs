use utilities::get_input_lines;

#[derive(Debug, PartialOrd, Eq, Ord, PartialEq)]
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

#[derive(Debug, PartialOrd, Eq, Ord, PartialEq)]
enum CardP2 {
    Joker, 
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen, // 12
    King, // 13
    Ace, // 14
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
enum HandType {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}


#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bet: u64,
    kind: HandType,
}

impl Hand {
    fn new(cards: Vec<char>, bet: u64) -> Hand {
        let mut char_count = std::collections::HashMap::new();

        for card in &cards {
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

#[derive(Debug, PartialEq, Eq)]
struct HandP2 {
    cards: Vec<CardP2>,
    bet: u64,
    kind: HandType,
}

impl HandP2 {
    fn new(cards: Vec<char>, bet: u64) -> HandP2 {
        let mut char_count = std::collections::HashMap::new();

        for card in &cards {
            let count = char_count.entry(card).or_insert(0);
            *count += 1;
        }

        let kind = match char_count.len() {
            1 => HandType::Five,
            2 => {
                if char_count.contains_key(&'J') {
                    HandType::Five
                } else {
                    if char_count.values().any(|&x| x == 4) {
                        HandType::Four
                    } else {
                        HandType::Full
                    }
                }
            }
            3 => {
                if char_count.contains_key(&'J') {
                    if char_count.values().any(|&x| x == 3) {
                        HandType::Four
                    } else {
                        HandType::Full
                    }
                } else {
                    if char_count.values().any(|&x| x == 3) {
                        HandType::Three
                    } else {
                        HandType::Two
                    }
                }
            }
            4 => {
                if char_count.contains_key(&'J') {
                    if char_count.get(&'J').unwrap() == &2 {
                        HandType::Three
                    } else {
                        HandType::Two
                    }
                } else {
                    HandType::One
                }
            },
            5 => {
                if char_count.contains_key(&'J') {
                    HandType::One
                } else {
                    HandType::High
                }
            },
            _ => panic!("Invalid hand"),
        };

        let cards = cards.iter().map(|c| get_card_p2(*c)).collect();
        HandP2 { cards, bet, kind }
    }
}

impl HasBet for HandP2 {
    fn get_bet(&self) -> u64 {
        self.bet
    }
}

impl HasBet for Hand {
    fn get_bet(&self) -> u64 {
        self.bet
    }
}

impl Ord for HandP2 {
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

impl PartialOrd for HandP2 {
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

fn get_card_p2(card: char) -> CardP2 {
    match card {
        '2' => CardP2::Two,
        '3' => CardP2::Three,
        '4' => CardP2::Four,
        '5' => CardP2::Five,
        '6' => CardP2::Six,
        '7' => CardP2::Seven,
        '8' => CardP2::Eight,
        '9' => CardP2::Nine,
        'T' => CardP2::Ten,
        'J' => CardP2::Joker,
        'Q' => CardP2::Queen,
        'K' => CardP2::King,
        'A' => CardP2::Ace,
        _ => panic!("Invalid card"),
    }
}

trait HasBet {
    fn get_bet(&self) -> u64;
}

fn get_winnings<T>(hands: &mut Vec<T>) 
where 
    T: Ord + HasBet
{
    hands.sort();
    
    let mut winnings = 0;
    for i in 0..hands.len() {
        let rank = i + 1;
        let hand = &hands[i];
        winnings += hand.get_bet() * rank as u64; 
    }

    println!("{}", winnings);
}

fn main() {
    let (line, part) = get_input_lines();
    if part == 1 {
        let mut hands: Vec<Hand> = line
            .iter()
            .map(|line| {
                let mut line_div = line.split_whitespace();
                let cards = line_div.next().unwrap().chars().collect();
                let bet = line_div.next().unwrap().parse().unwrap();
                Hand::new(cards, bet)
            })
            .collect();

        get_winnings(&mut hands);
    } else {
        let mut hands: Vec<HandP2> = line
            .iter()
            .map(|line| {
                let mut line_div = line.split_whitespace();
                let cards = line_div.next().unwrap().chars().collect();
                let bet = line_div.next().unwrap().parse().unwrap();
                HandP2::new(cards, bet)
            })
            .collect();

        get_winnings(&mut hands);
    }
}
