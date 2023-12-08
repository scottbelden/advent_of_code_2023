use counter::Counter;
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
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
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct Hand {
    hand_type: HandType,
    original_cards: Vec<Card>,
    bid: usize,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        } else if self.hand_type > other.hand_type {
            return Ordering::Greater;
        } else {
            return self.original_cards.cmp(&other.original_cards);
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.original_cards == other.original_cards
    }
}

impl Eq for Hand {}

fn get_hand_type(cards: &str) -> HandType {
    let char_counts = cards.chars().collect::<Counter<_>>();
    let mut char_values = char_counts.most_common().into_iter();
    let (_, most_common) = char_values.next().unwrap();
    return match most_common {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            let (_, next_common) = char_values.next().unwrap();
            let hand_type;
            if next_common == 2 {
                hand_type = HandType::FullHouse;
            } else {
                hand_type = HandType::ThreeOfAKind;
            }
            return hand_type;
        }
        2 => {
            let (_, next_common) = char_values.next().unwrap();
            let hand_type;
            if next_common == 2 {
                hand_type = HandType::TwoPair;
            } else {
                hand_type = HandType::OnePair;
            }
            return hand_type;
        }
        _ => HandType::HighCard,
    };
}

fn get_cards(cards: &str) -> Vec<Card> {
    cards
        .chars()
        .map(|char| match char {
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
            _ => Card::Ace,
        })
        .collect()
}

fn main() {
    let file_path = "src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut hands = contents
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let hand = Hand {
                hand_type: get_hand_type(cards),
                original_cards: get_cards(cards),
                bid: bid.parse::<usize>().unwrap(),
            };
            return hand;
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    let product = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .fold(0, |acc, elem| acc + elem);

    println!("Part 1: {product}");

    // println!("Part 2: {count}");
}
