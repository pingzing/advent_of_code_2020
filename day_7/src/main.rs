use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::str;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_7/input.txt";
lazy_static! {
    static ref RULE_SPLIT_REGEX: Regex =
        Regex::new(r"(.+ .+) bags contain ([0-9] .+ .+ bag[s]?|no other bag[s]?)").unwrap();
    static ref RULE_DETAIL_REGEX: Regex = Regex::new(r"([0-9]) ([a-z]+ [a-z]+) bag[s]?").unwrap();
}

fn main() {
    let bag_rules = parse_rules();
    let shiny_gold_count = count_bags("shiny gold", &bag_rules);
    println!(
        "PART ONE: Found {} bags that can (eventually) hold a shiny gold bag.",
        shiny_gold_count
    );

    let bag_inception_count = bag_inception(&bag_rules, bag_rules.get("shiny gold").unwrap());
    println!(
        "PART TWO: You need to pack {} other bags inside yours.",
        bag_inception_count
    );
}

fn parse_rules() -> HashMap<String, Option<Vec<(String, u32)>>> {
    let mut bag_rules: HashMap<String, Option<Vec<(String, u32)>>> = HashMap::new();
    let file = File::open(PATH).unwrap();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let split_rule = RULE_SPLIT_REGEX.captures(&line).unwrap();
        let bag_id = split_rule.get(1).unwrap().as_str().to_string();
        let rules_string = split_rule.get(2).unwrap().as_str();
        if rules_string == "no other bags" {
            bag_rules.insert(bag_id, None);
            continue;
        }

        let mut inner_rule_vec: Vec<(String, u32)> = vec![];
        for capture in RULE_DETAIL_REGEX.captures_iter(&rules_string) {
            let detail_bag_id = capture.get(2).unwrap().as_str().to_string();
            let detail_bag_count: u32 = capture.get(1).unwrap().as_str().parse().unwrap();
            inner_rule_vec.push((detail_bag_id, detail_bag_count));
        }

        bag_rules.insert(bag_id, Some(inner_rule_vec));
    }

    bag_rules
}

fn count_bags(target_bag_id: &str, rules: &HashMap<String, Option<Vec<(String, u32)>>>) -> u32 {
    rules
        .iter()
        .map(|(_, rule)| contains_bag_recursive(&rules, rule, target_bag_id) as u32)
        .sum()
}

fn contains_bag_recursive(
    bag_rules: &HashMap<String, Option<Vec<(String, u32)>>>,
    rule_opt: &Option<Vec<(String, u32)>>,
    target: &str,
) -> bool {
    if let Some(rule) = rule_opt {
        for bag in rule.iter() {
            if bag.0 == target
                || contains_bag_recursive(bag_rules, bag_rules.get(&bag.0).unwrap(), target)
            {
                return true;
            }
        }

        false
    } else {
        false
    }
}

fn bag_inception(
    bag_rules: &HashMap<String, Option<Vec<(String, u32)>>>,
    rule_opt: &Option<Vec<(String, u32)>>,
) -> u32 {
    if rule_opt.is_none() {
        return 0u32;
    }

    let mut count = 0u32;
    for rule in rule_opt.as_ref().unwrap() {
        count += rule.1 + rule.1 * bag_inception(bag_rules, bag_rules.get(&rule.0).unwrap());
    }

    return count;
}
