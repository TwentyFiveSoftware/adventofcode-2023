use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/05.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("PART 1: {}", part1(&lines));
}

fn part1(lines: &Vec<String>) -> u64 {
    let almanac = parse_almanac_from_lines(lines);

    almanac.seeds.iter()
        .map(|seed| almanac.map_seed_to_location(*seed))
        .min()
        .unwrap()
}

fn parse_almanac_from_lines(lines: &Vec<String>) -> Almanac {
    let seeds = (&lines.get(0).unwrap()[7..])
        .split(" ")
        .map(|number| number.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mapping_start_indices = vec![
        lines.iter().position(|line| line == "seed-to-soil map:").unwrap(),
        lines.iter().position(|line| line == "soil-to-fertilizer map:").unwrap(),
        lines.iter().position(|line| line == "fertilizer-to-water map:").unwrap(),
        lines.iter().position(|line| line == "water-to-light map:").unwrap(),
        lines.iter().position(|line| line == "light-to-temperature map:").unwrap(),
        lines.iter().position(|line| line == "temperature-to-humidity map:").unwrap(),
        lines.iter().position(|line| line == "humidity-to-location map:").unwrap(),
    ];


    let mut mappings: Vec<Mapping> = vec![];
    for (i, start_index) in mapping_start_indices.iter().enumerate() {
        let mut ranges: Vec<MappingRange> = vec![];

        let end_index = if i + 1 < mapping_start_indices.len() {
            mapping_start_indices[i + 1] - 1
        } else {
            lines.len()
        };

        for line in &lines[(*start_index + 1)..end_index] {
            let numbers = line
                .split(" ")
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            ranges.push(MappingRange {
                destination_range_start: *numbers.get(0).unwrap(),
                source_range_start: *numbers.get(1).unwrap(),
                range_length: *numbers.get(2).unwrap(),
            })
        }

        mappings.push(Mapping { ranges });
    }

    Almanac { seeds, mappings }
}


struct Almanac {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

struct Mapping {
    ranges: Vec<MappingRange>,
}

struct MappingRange {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

impl Almanac {
    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let mut number = seed;

        for mapping in &self.mappings {
            number = mapping.map_number(number)
        }

        number
    }
}

impl Mapping {
    fn map_number(&self, number: u64) -> u64 {
        for range in &self.ranges {
            if number >= range.source_range_start && number < range.source_range_start + range.range_length {
                return range.destination_range_start + (number - range.source_range_start);
            }
        }

        return number;
    }
}
