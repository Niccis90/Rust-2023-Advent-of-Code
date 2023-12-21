use core::num;
use std::time::Instant;

mod data;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Hand {
    cards: [u8; 5],
    bid: u16,
    Type: HandType,
}

impl Hand {
    fn determine_type(&self) -> HandType {
        if self.cards.contains(&1) {
            let mut cards_count = [0; 13];
            let mut num_jokers = 0;

            for &card in &self.cards {
                match card {
                    1 => num_jokers += 1,
                    2..=14 => cards_count[card as usize - 2] += 1,
                    _ => eprintln!("Invalid card type: {}", card),
                }
            }

            let (most_freq_card, _) = cards_count
                .iter()
                .enumerate()
                .max_by_key(|&(i, &count)| (count, i))
                .unwrap_or((0, &0));

            if num_jokers > 0 {
                cards_count[most_freq_card] += num_jokers;
            }

            let pairs = cards_count.iter().filter(|&&count| count == 2).count();
            let three_of_a_kind = cards_count.iter().any(|&count| count == 3);
            let four_of_a_kind = cards_count.iter().any(|&count| count == 4);
            let five_of_a_kind = cards_count.iter().any(|&count| count == 5);

            match () {
                _ if five_of_a_kind => HandType::Five_of_a_kind,
                _ if four_of_a_kind => HandType::Four_of_a_kind,
                _ if three_of_a_kind && pairs == 1 => HandType::Full_house,
                _ if three_of_a_kind => HandType::Three_of_a_kind,
                _ if pairs == 2 => HandType::Two_pair,
                _ if pairs == 1 => HandType::One_pair,
                _ => HandType::High_card,
            }
        } else {
            let mut cards_count = [0; 13];
            for &card in &self.cards {
                if card >= 2 && card <= 14 {
                    cards_count[(card - 2) as usize] += 1;
                } else {
                    eprintln!("invalid card type:{}", card);
                }
            }
            let pairs = cards_count.iter().filter(|&&count| count == 2).count();
            let three_of_a_kind = cards_count.iter().any(|&count| count == 3);
            let four_of_a_kind = cards_count.iter().any(|&count| count == 4);
            let five_of_a_kind = cards_count.iter().any(|&count| count == 5);

            if five_of_a_kind {
                HandType::Five_of_a_kind
            } else if four_of_a_kind {
                HandType::Four_of_a_kind
            } else if three_of_a_kind && pairs == 1 {
                HandType::Full_house
            } else if three_of_a_kind {
                HandType::Three_of_a_kind
            } else if pairs == 2 {
                HandType::Two_pair
            } else if pairs == 1 {
                HandType::One_pair
            } else {
                HandType::High_card
            }
        }
    }
}
#[derive(PartialEq, Debug, Clone, Copy)]
enum HandType {
    High_card,
    One_pair,
    Two_pair,
    Three_of_a_kind,
    Full_house,
    Four_of_a_kind,
    Five_of_a_kind,
}

impl HandType {
    fn to_numeric(&self) -> u8 {
        match self {
            HandType::High_card => 1,
            HandType::One_pair => 2,
            HandType::Two_pair => 3,
            HandType::Three_of_a_kind => 4,
            HandType::Full_house => 5,
            HandType::Four_of_a_kind => 6,
            HandType::Five_of_a_kind => 7,
        }
    }
}
#[derive(PartialEq, Debug)]
enum Value {
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

impl Value {
    fn from_str(value: &str) -> Option<Value> {
        match value.to_uppercase().as_str() {
            "2" => Some(Value::Two),
            "3" => Some(Value::Three),
            "4" => Some(Value::Four),
            "5" => Some(Value::Five),
            "6" => Some(Value::Six),
            "7" => Some(Value::Seven),
            "8" => Some(Value::Eight),
            "9" => Some(Value::Nine),
            "T" => Some(Value::Ten),
            "J" => Some(Value::Jack),
            "Q" => Some(Value::Queen),
            "K" => Some(Value::King),
            "A" => Some(Value::Ace),
            _ => None,
        }
    }
    fn get_value(&self) -> u8 {
        match self {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 1, // change this between 1 or 11 depending on part to solve.
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }
}

fn get_hands(data: &str) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in data.lines() {
        let mut cards_array = [0; 5];
        let mut splitted = line.split_whitespace();
        let cards = splitted.next().unwrap();
        let bid: u16 = splitted.next().unwrap().parse().unwrap();
        for (idx, card) in cards.chars().enumerate() {
            let card_str = card.to_string();
            if let Some(card_value) = Value::from_str(&card_str) {
                let numeric_val = card_value.get_value();
                cards_array[idx] = numeric_val;
            }
        }
        let mut hand = Hand {
            cards: cards_array,
            bid: bid,
            Type: HandType::High_card,
        };
        hand.Type = hand.determine_type();
        hands.push(hand);
    }
    hands
}

fn faceoff(hand1: &Hand, hand2: &Hand) -> (Hand, Hand) {
    let mut winner: &Hand = &hand2;
    let mut loser: &Hand = &hand1;
    let h1_type = &hand1.Type;
    let h2_type = &hand2.Type;
    if h1_type.to_numeric() > h2_type.to_numeric() {
        winner = hand1
    } else if h1_type.to_numeric() == h2_type.to_numeric() {
        let mut h1_nums = hand1.cards;
        let mut h2_nums = hand2.cards;
        for (idx1, h1) in h1_nums.iter().enumerate() {
            if h1 > &h2_nums[idx1] {
                winner = hand1;
                break;
            } else if h1 == &h2_nums[idx1] {
                continue;
            } else {
                winner = hand2;
                break;
            }
        }
    } else {
        winner = hand2
    }
    if winner == hand1 {
        loser = hand2
    } else {
        loser = hand1
    }

    (winner.clone(), loser.clone())
}

fn part1_2(data: &str) -> u64 {
    let mut hands = get_hands(data);
    let mut sorted: Vec<Hand> = Vec::new();

    while !hands.is_empty() {
        let mut idx1 = 0;
        while idx1 < hands.len() {
            let mut lost_once = false;
            let hand1 = hands[idx1];

            for idx2 in 0..hands.len() {
                if idx1 != idx2 {
                    let hand2 = hands[idx2];
                    let (winner, _) = faceoff(&hand1, &hand2);
                    if winner != hand1 {
                        lost_once = true;
                        break;
                    }
                }
            }

            if !lost_once {
                sorted.push(hand1);
                hands.remove(idx1);
            } else {
                idx1 += 1;
            }
        }
    }
    let mut sum: u64 = 0;
    for (rank, hand) in sorted.iter().rev().enumerate() {
        sum += hand.bid as u64 * (rank as u64 + 1) as u64;
    }

    sum
}

fn main() {
    let time = Instant::now();
    let test = data::TEST;
    let data = data::DATA;
    let part1_2 = part1_2(data);
    println!("{:?}", part1_2);

    println!("{:?}", time.elapsed());
}
// answer1 = 249390788 5.9895ms
// asnwer2 = 248750248 5.3412ms
