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
#[derive(PartialEq)]
struct Hand {
    cards: Vec<char>,
    bet: u64,
    kind: HandType
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if  
    }
}

fn get_kind(cards: &Vec<char>) {
    let dups: Vec<uszie> = if let Some((i, &first_char)) = chars.iter().enumerate().find(|(i, &c)| chars[..*i].contains(&c)) {
        chars.iter().enumerate()
            .filter(|&(i, &c)| i != index && c == first_char)
            .map(|(i, _)| i)
            .collect()
    } else {
        Vec::new()
    };
}

fn main() {
    let (line, _part) = get_input_lines();
    let hands: Vec<Hand> = line
        .iter()
        .map(|line| {
            let mut line_div = line.split_whitespace();
            Hand {
                cards: line_div.next().unwrap().chars().collect(),
                bet: line_div.next().unwrap().parse().unwrap()
            }
        })
        .collect();

    println!("{:?}", hands);
    

}
