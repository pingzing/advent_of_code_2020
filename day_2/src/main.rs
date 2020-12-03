use std::fmt;
use std::str;
use std::vec;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_2/input.txt";
const REGEX: &str = r"([0-9]+)-([0-9]+) (.): (.+)";

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let entries = get_password_entries();
    let entries_len = entries.iter().len();

    let invalid_entries: Vec::<PasswordAndRule> = entries.into_iter().filter(is_invalid_part_one).collect();
    for entry in invalid_entries.iter() {
        println!("{}", entry);
    }
    println!("PART ONE: Total invalid entries: {}. Total valid is: {}", invalid_entries.len(), entries_len - invalid_entries.len());    
}

fn part_two() {
    let entries = get_password_entries();    

    let valid_entries: Vec::<PasswordAndRule> = entries.into_iter().filter(is_valid_part_two).collect();
    println!("PART TWO: Total valid entries: {}", valid_entries.len());
}

fn get_password_entries() -> vec::Vec<PasswordAndRule> {
    let file = File::open(PATH).unwrap();
    let regex = Regex::new(REGEX).unwrap();
    io::BufReader::new(file).lines()
        .map(|x| {
                let line = x.unwrap();                
                let caps = regex.captures(&line).unwrap();
                PasswordAndRule {
                    first_num: caps.get(1).unwrap().as_str().parse().unwrap(),
                    second_num: caps.get(2).unwrap().as_str().parse().unwrap(),
                    rule: caps.get(3).unwrap().as_str().to_string(),
                    password: caps.get(4).unwrap().as_str().to_string()
                }
            })
        .collect()
}

fn is_invalid_part_one(entry: &PasswordAndRule) -> bool {
    let letter_count = entry.password.matches(&entry.rule).count() as u32;
    println!("Letter count for {} is: {}", entry, letter_count);
    if letter_count < entry.first_num || letter_count > entry.second_num {
        return true;
    }

    false
}

fn is_valid_part_two(entry: &PasswordAndRule) -> bool {
    let rule_char = entry.rule.as_bytes()[0] as char;
    let indexable_password = entry.password.as_bytes();
    let first_pos_contains: bool = (indexable_password[(entry.first_num - 1) as usize] as char) == rule_char;
    let second_pos_contains: bool = (indexable_password[(entry.second_num - 1) as usize] as char) == rule_char;

    first_pos_contains != second_pos_contains
}

struct PasswordAndRule {
    rule: String,
    first_num: u32,
    second_num: u32,
    password: String,
}

impl fmt::Display for PasswordAndRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{} {}: {}", self.first_num, self.second_num, self.rule, self.password)
    }
}