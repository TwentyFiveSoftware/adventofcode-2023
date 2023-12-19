use std::cmp::{max, min};
use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input_file = File::open("inputs/19.txt").unwrap();

    let lines = BufReader::new(input_file)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let input_split = lines.split(|line| line.is_empty()).collect::<Vec<&[String]>>();

    let workflows = input_split.get(0).unwrap().iter()
        .map(|line| Workflow::from_str(line).unwrap())
        .fold(HashMap::new(), |mut map, workflow| {
            map.insert(workflow.name.clone(), workflow);
            map
        });

    let parts = input_split.get(1).unwrap().iter()
        .map(|line| Part::from_str(line).unwrap())
        .collect::<Vec<Part>>();

    println!("PART 1: {}", part1(&workflows, &parts));
    println!("PART 2: {}", part2(&workflows));
}

fn part1(workflows: &HashMap<String, Workflow>, parts: &Vec<Part>) -> u32 {
    parts
        .iter()
        .filter(|part| {
            let mut current_workflow = workflows.get("in").unwrap();

            loop {
                match current_workflow.get_action_for_part(&part) {
                    RuleAction::ACCEPT => {
                        return true;
                    }
                    RuleAction::REJECT => {
                        return false;
                    }
                    RuleAction::JUMP(workflow_name) => {
                        current_workflow = workflows.get(workflow_name.as_str()).unwrap();
                    }
                }
            }
        })
        .map(Part::rating_sum)
        .sum()
}

fn part2(workflows: &HashMap<String, Workflow>) -> u64 {
    find_all_accepting_ranges(&workflows, "in", RatingRange::new())
        .iter()
        .filter(|range| range.is_possible())
        .map(|range| range.ranges
            .values()
            .map(|(lower, upper)| (upper - lower + 1) as u64)
            .fold(1, |acc, x| acc * x)
        )
        .sum()
}

fn find_all_accepting_ranges(
    workflows: &HashMap<String, Workflow>,
    current_workflow_name: &str,
    mut current_range: RatingRange,
) -> Vec<RatingRange> {
    let workflow = workflows.get(current_workflow_name).unwrap();

    let mut ranges = vec![];

    for rule in &workflow.rules {
        match &rule.action {
            RuleAction::ACCEPT => {
                ranges.push(current_range.with_rule_applied(rule));
            }
            RuleAction::REJECT => {}
            RuleAction::JUMP(workflow_name) => {
                ranges.append(&mut find_all_accepting_ranges(workflows, workflow_name, current_range.with_rule_applied(rule)));
            }
        }

        current_range = current_range.with_inverted_rule_applied(rule);
    }

    match &workflow.default_action {
        RuleAction::ACCEPT => {
            ranges.push(current_range);
        }
        RuleAction::REJECT => {}
        RuleAction::JUMP(workflow_name) => {
            ranges.append(&mut find_all_accepting_ranges(workflows, workflow_name, current_range));
        }
    }

    ranges
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_action: RuleAction,
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^([a-z]+)\{(.*)}$").unwrap();
        let groups = regex.captures(s).unwrap();

        let name = groups.get(1).unwrap().as_str().to_string();
        let raw_rules = groups.get(2).unwrap().as_str().split(",").collect::<Vec<&str>>();

        let rules = raw_rules.iter().take(raw_rules.len() - 1)
            .map(|s| Rule::from_str(s).unwrap()).collect::<Vec<Rule>>();

        Ok(Workflow {
            name: name.to_string(),
            rules,
            default_action: RuleAction::from_str(raw_rules.last().unwrap()).unwrap(),
        })
    }
}

impl Workflow {
    fn get_action_for_part(&self, part: &Part) -> RuleAction {
        self.rules.iter()
            .find_map(|rule| rule.apply_for_part(&part))
            .unwrap_or(self.default_action.clone())
    }
}

#[derive(Clone)]
struct Rule {
    rating_type: RatingType,
    condition: RuleCondition,
    value: u32,
    action: RuleAction,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^([xmas])([><])([0-9]+):([a-zA-Z]+)$").unwrap();
        let groups = regex.captures(s).unwrap();

        let rating_type = RatingType::from_str(groups.get(1).unwrap().as_str()).unwrap();
        let condition = RuleCondition::from_str(groups.get(2).unwrap().as_str()).unwrap();
        let value = groups.get(3).unwrap().as_str().parse::<u32>().unwrap();
        let action = RuleAction::from_str(groups.get(4).unwrap().as_str()).unwrap();

        Ok(Rule { rating_type, condition, value, action })
    }
}

impl Rule {
    fn apply_for_part(&self, part: &Part) -> Option<RuleAction> {
        let rating = *part.ratings.get(&self.rating_type).unwrap();

        let matches = match self.condition {
            RuleCondition::LESS => rating < self.value,
            RuleCondition::GREATER => rating > self.value,
        };

        if !matches {
            return None;
        }

        Some(self.action.clone())
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
enum RatingType {
    X,
    M,
    A,
    S,
}

impl FromStr for RatingType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(RatingType::X),
            "m" => Ok(RatingType::M),
            "a" => Ok(RatingType::A),
            "s" => Ok(RatingType::S),
            &_ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
enum RuleCondition {
    LESS,
    GREATER,
}

impl FromStr for RuleCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(RuleCondition::GREATER),
            "<" => Ok(RuleCondition::LESS),
            &_ => Err(()),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
enum RuleAction {
    ACCEPT,
    REJECT,
    JUMP(String),
}

impl FromStr for RuleAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RuleAction::ACCEPT),
            "R" => Ok(RuleAction::REJECT),
            &_ => Ok(RuleAction::JUMP(s.to_string())),
        }
    }
}

struct Part {
    ratings: HashMap<RatingType, u32>,
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)}$").unwrap();
        let groups = regex.captures(s).unwrap();

        let mut ratings = HashMap::new();
        ratings.insert(RatingType::X, groups.get(1).unwrap().as_str().parse::<u32>().unwrap());
        ratings.insert(RatingType::M, groups.get(2).unwrap().as_str().parse::<u32>().unwrap());
        ratings.insert(RatingType::A, groups.get(3).unwrap().as_str().parse::<u32>().unwrap());
        ratings.insert(RatingType::S, groups.get(4).unwrap().as_str().parse::<u32>().unwrap());

        Ok(Part { ratings })
    }
}

impl Part {
    fn rating_sum(&self) -> u32 {
        self.ratings.values().sum()
    }
}

#[derive(Clone)]
struct RatingRange {
    ranges: HashMap<RatingType, (u32, u32)>,
}

impl RatingRange {
    fn new() -> RatingRange {
        RatingRange {
            ranges: vec![RatingType::X, RatingType::M, RatingType::A, RatingType::S].iter()
                .fold(HashMap::new(), |mut map, rating_type| {
                    map.insert(*rating_type, (1, 4000));
                    map
                })
        }
    }

    fn with_rule_applied(&self, rule: &Rule) -> RatingRange {
        let mut ranges = self.ranges.clone();

        let (lower, upper) = ranges.get(&rule.rating_type).unwrap();

        match rule.condition {
            RuleCondition::LESS => {
                ranges.insert(rule.rating_type, (*lower, min(*upper, rule.value - 1)));
            }
            RuleCondition::GREATER => {
                ranges.insert(rule.rating_type, (max(*lower, rule.value + 1), *upper));
            }
        }

        RatingRange { ranges }
    }

    fn with_inverted_rule_applied(&self, rule: &Rule) -> RatingRange {
        let mut ranges = self.ranges.clone();

        let (lower, upper) = ranges.get(&rule.rating_type).unwrap();

        match rule.condition {
            RuleCondition::LESS => {
                ranges.insert(rule.rating_type, (max(*lower, rule.value), *upper));
            }
            RuleCondition::GREATER => {
                ranges.insert(rule.rating_type, (*lower, min(*upper, rule.value)));
            }
        }

        RatingRange { ranges }
    }

    fn is_possible(&self) -> bool {
        self.ranges.values().all(|(lower, upper)| lower <= upper)
    }
}
