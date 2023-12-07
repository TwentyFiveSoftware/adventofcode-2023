use std::cmp::Ordering;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/07.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut hands = lines
        .iter()
        .map(|line| Hand::from_str(line).unwrap())
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.compare(b));

    hands.iter()
        .enumerate()
        .map(|(i, hand)| (i as i32 + 1) * hand.bid as i32)
        .sum()
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut hands = lines
        .iter()
        .map(|line| line.replace("J", "*"))
        .map(|line| Hand::from_str(&line).unwrap())
        .collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.compare(b));

    hands.iter()
        .enumerate()
        .map(|(i, hand)| (i as i32 + 1) * hand.bid as i32)
        .sum()
}

struct Hand {
    cards: Vec<u8>,
    bid: u32,
}

impl Hand {
    fn get_hand_type(&self) -> u8 {
        let mut initial_card_count_per_type = self.cards.iter().
            fold(HashMap::new(), |mut map, card| {
                map.entry(card).and_modify(|count| *count += 1).or_insert(1);
                map
            });

        if !initial_card_count_per_type.contains_key(&0) || initial_card_count_per_type.len() == 1 {
            return Hand::get_hand_type_from_cards(&initial_card_count_per_type);
        }

        let joker_count = *initial_card_count_per_type.get(&0).unwrap();
        initial_card_count_per_type.remove(&0);

        initial_card_count_per_type.keys()
            .map(|card_type| {
                let mut card_counts_per_type = initial_card_count_per_type.clone();
                card_counts_per_type.entry(card_type).and_modify(|count| *count += joker_count);
                Hand::get_hand_type_from_cards(&card_counts_per_type)
            })
            .max()
            .unwrap()
    }

    fn get_hand_type_from_cards(card_count_per_type: &HashMap<&u8, i32>) -> u8 {
        // five of a kind
        if card_count_per_type.values().any(|count| *count == 5) {
            return 6;
        }

        // four of a kind
        if card_count_per_type.values().any(|count| *count == 4) {
            return 5;
        }

        // full house
        if card_count_per_type.values().all(|count| *count == 3 || *count == 2) {
            return 4;
        }

        // three of a kind
        if card_count_per_type.values().any(|count| *count == 3) {
            return 3;
        }

        // two pair
        if card_count_per_type.len() == 3 && card_count_per_type.values().all(|count| *count == 2 || *count == 1) {
            return 2;
        }

        // one pair
        if card_count_per_type.values().any(|count| *count == 2) {
            return 1;
        }

        // high card
        return 0;
    }


    fn compare(&self, other: &Hand) -> Ordering {
        let type_delta = (self.get_hand_type() as i8) - (other.get_hand_type() as i8);

        if type_delta > 0 {
            return Ordering::Greater;
        }
        if type_delta < 0 {
            return Ordering::Less;
        }

        zip(&self.cards, &other.cards)
            .find_map(|(a, b)| {
                if a.cmp(b) != Ordering::Equal {
                    return Some(a.cmp(b));
                }

                None
            }).unwrap()
    }
}

const CARD_RANKS: &'static [char] = &['*', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = &s.split(" ").collect::<Vec<&str>>()[..];

        let bid = split.get(1).unwrap().parse::<u32>().unwrap();

        let cards = split.get(0).unwrap().chars()
            .map(|card| CARD_RANKS.iter().position(|c| *c == card).unwrap() as u8)
            .collect();

        Ok(Hand { cards, bid })
    }
}
