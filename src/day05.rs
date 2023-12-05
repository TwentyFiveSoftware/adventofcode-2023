use std::cmp::{max, min};
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
    println!("PART 2: {}", part2(&lines));
}

fn part1(lines: &Vec<String>) -> u64 {
    let almanac = parse_almanac_from_lines(lines);

    almanac.seeds.iter()
        .map(|seed| almanac.map_seed_to_location(*seed))
        .min()
        .unwrap()
}

fn part2(lines: &Vec<String>) -> u64 {
    let almanac = parse_almanac_from_lines(lines);
    let seed_ranges = almanac.get_seed_ranges();

    let mut seed_locations_ranges: Vec<MappingRange> = almanac.mappings.last().unwrap().ranges.to_vec();
    seed_locations_ranges.sort_by_key(|range| range.destination_range_start);

    let mut lowest_seed_locations: Vec<u64> = Vec::new();

    for seed_location_range in &seed_locations_ranges[..seed_locations_ranges.len() - 1] {
        let inputs = almanac.build_input_for_seed_location_range(*seed_location_range);

        for input in &inputs {
            let matching_seed_ranges = seed_ranges.iter()
                .filter(|seed_range| input.source_range_start <= seed_range.start + seed_range.length && seed_range.start <= input.source_range_start + input.range_length)
                .map(|seed_range| {
                    let start = max(seed_range.start, input.source_range_start);
                    let end = min(seed_range.start + seed_range.length, input.source_range_start + input.range_length);
                    SeedRange { start, length: end - start }
                })
                .collect::<Vec<SeedRange>>();

            for matching_seed_range in matching_seed_ranges {
                lowest_seed_locations.push(almanac.map_seed_to_location(matching_seed_range.start));
            }
        }
    }

    *lowest_seed_locations.iter().min().unwrap()
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

        ranges.sort_by_key(|range| range.source_range_start);

        // fill empty ranges
        let first_range = ranges.first().unwrap();
        if first_range.source_range_start > 0 {
            ranges.insert(0, MappingRange {
                source_range_start: 0,
                destination_range_start: 0,
                range_length: first_range.source_range_start,
            })
        }

        let last_range = ranges.last().unwrap();
        ranges.push(MappingRange {
            source_range_start: last_range.source_range_start + last_range.range_length,
            destination_range_start: last_range.source_range_start + last_range.range_length,
            range_length: u64::MAX - (last_range.source_range_start + last_range.range_length),
        });

        for j in 1..(ranges.len() - 1) {
            let current_range = ranges.get(j).unwrap();
            let next_range = ranges.get(j + 1).unwrap();

            let intermediary_range_start = current_range.source_range_start + current_range.range_length;
            let intermediary_range_length = next_range.source_range_start - intermediary_range_start;

            if intermediary_range_length == 0 {
                continue;
            }

            ranges.insert(j + 1, MappingRange {
                source_range_start: intermediary_range_start,
                destination_range_start: intermediary_range_start,
                range_length: intermediary_range_length,
            });
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

#[derive(Copy, Clone)]
struct MappingRange {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

#[derive(Copy, Clone)]
struct SeedRange {
    start: u64,
    length: u64,
}

impl Almanac {
    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let mut number = seed;

        for mapping in &self.mappings {
            number = mapping.map_number(number)
        }

        number
    }

    fn build_input_for_seed_location_range(&self, seed_location_range: MappingRange) -> Vec<MappingRange> {
        let mut inputs: Vec<MappingRange> = vec![seed_location_range];

        for mapping in self.mappings.iter().rev() {
            inputs = mapping.build_input_for_outputs(&inputs);
        }

        inputs
    }

    fn get_seed_ranges(&self) -> Vec<SeedRange> {
        let mut seed_ranges: Vec<SeedRange> = vec![];

        for i in (0..self.seeds.len()).step_by(2) {
            seed_ranges.push(SeedRange {
                start: *self.seeds.get(i).unwrap(),
                length: *self.seeds.get(i + 1).unwrap(),
            });
        }

        seed_ranges
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

    fn build_input_for_outputs(&self, output_ranges: &Vec<MappingRange>) -> Vec<MappingRange> {
        let mut input_ranges: Vec<MappingRange> = vec![];

        for output_range in output_ranges {
            input_ranges.append(&mut self.build_input_for_output(output_range));
        }

        return input_ranges;
    }

    fn build_input_for_output(&self, output_range: &MappingRange) -> Vec<MappingRange> {
        let output_range_start = output_range.source_range_start;
        let output_range_end = output_range_start + output_range.range_length;

        let mut input_ranges: Vec<MappingRange> = vec![];

        let mut current_start = output_range_start;

        while current_start < output_range_end {
            let range = &self.ranges.iter()
                .find(|r| r.destination_range_start <= current_start && r.destination_range_start + r.range_length > current_start)
                .unwrap();

            let non_intersecting_range_length = current_start - range.destination_range_start;
            let mut intersection_range_length = range.range_length - non_intersecting_range_length;

            if current_start + intersection_range_length > output_range_end {
                intersection_range_length = output_range_end - current_start;
            }

            input_ranges.push(MappingRange {
                source_range_start: range.source_range_start + non_intersecting_range_length,
                destination_range_start: range.destination_range_start + non_intersecting_range_length,
                range_length: intersection_range_length,
            });

            current_start += intersection_range_length;
        }

        input_ranges
    }
}
