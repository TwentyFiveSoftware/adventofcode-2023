use std::collections::{HashMap, HashSet};
use regex::{Regex};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/04.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> i32 {
    lines
        .iter()
        .map(|line| Scratchcard::from_str(line).unwrap())
        .map(|scratchcard| {
            match scratchcard.match_count() {
                0 => 0,
                match_count => 2i32.pow(match_count as u32 - 1),
            }
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> u32 {
    let scratchcards: HashMap<u32, Scratchcard> = lines
        .iter()
        .map(|line| Scratchcard::from_str(line).unwrap())
        .fold(HashMap::new(), |mut scratchcards, scratchcard| {
            scratchcards.insert(scratchcard.card_number, scratchcard);
            scratchcards
        });

    let mut cache: HashMap<u32, u32> = HashMap::new();

    let total_scratchcards: u32 = scratchcards.values()
        .map(|scratchcard| {
            calculate_total_scratchcards(scratchcard.card_number, &scratchcards, &mut cache) + 1
        })
        .sum();

    total_scratchcards
}

fn calculate_total_scratchcards(
    card_number: u32,
    scratchcard_lookup: &HashMap<u32, Scratchcard>,
    cache: &mut HashMap<u32, u32>,
) -> u32 {
    let match_count = scratchcard_lookup.get(&card_number).unwrap().match_count() as u32;

    let mut total: u32 = match_count;

    for won_card_number in (card_number + 1)..(card_number + match_count + 1) {
        total += match cache.get(&won_card_number) {
            None => {
                let won_card_total = calculate_total_scratchcards(won_card_number, scratchcard_lookup, cache);
                cache.insert(won_card_number, won_card_total);
                won_card_total
            }
            Some(won_card_total) => *won_card_total
        }
    }

    total
}

struct Scratchcard {
    card_number: u32,
    numbers: HashSet<u8>,
    winning_numbers: HashSet<u8>,
}

impl Scratchcard {
    fn match_count(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }
}

impl FromStr for Scratchcard {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line_regex = Regex::new(r"^Card +([0-9]+): (.*) \| (.*)$").unwrap();
        let groups = line_regex.captures(line).unwrap();

        let card_number = groups.get(1).unwrap().as_str().parse::<u32>().unwrap();

        let winning_numbers: HashSet<u8> = HashSet::from_iter(
            groups.get(2).unwrap().as_str()
                .split(" ")
                .filter(|item| item.len() > 0)
                .map(|number| number.trim().parse::<u8>().unwrap())
        );

        let numbers: HashSet<u8> = HashSet::from_iter(
            groups.get(3).unwrap().as_str()
                .split(" ")
                .filter(|item| item.len() > 0)
                .map(|number| number.trim().parse::<u8>().unwrap())
        );

        Ok(Scratchcard { card_number, numbers, winning_numbers })
    }
}
