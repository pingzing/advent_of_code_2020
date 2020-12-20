use regex::Regex;
use std::io::{self, BufRead};
use std::{collections::HashMap, fs::File, io::Read};

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_19/input_pt1.txt";
const PATH2: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_19/input_pt2.txt";

fn main() {
    let (rules, strings) = parse_input(PATH);
    let part_one = matches_rule_zero(&rules, &strings);
    println!("PART ONE: Matches: {}", part_one);

    let (rules2, strings2) = parse_input(PATH2);
    let part_two = matches_rule_zero_loopily(&rules2, &strings2);
    println!("PART TWO: Matches: {}", part_two);
}

fn matches_rule_zero(rules: &HashMap<usize, Vec<Token>>, strings: &[String]) -> u32 {
    let rule_zero = rules.get(&0usize).unwrap();
    let mut rule_string = compile_rule(rules, &rule_zero);
    rule_string.insert_str(0, "(?m)^");
    rule_string.push_str("\\b");
    let rule_zero_regex = Regex::new(&rule_string).unwrap();
    println!("Rule zero string is: {:?}", rule_string);

    strings
        .iter()
        .map(|x| if rule_zero_regex.is_match(x) { 1 } else { 0 })
        .sum()
}

fn matches_rule_zero_loopily(rules: &HashMap<usize, Vec<Token>>, strings: &[String]) -> u32 {
    let rule_42 = rules.get(&42usize).unwrap();
    let rule_31 = rules.get(&31).unwrap();

    let rule_42_string = "(".to_owned() + &compile_rule(rules, &rule_42) + ")";
    let rule_31_string = "(".to_owned() + &compile_rule(rules, &rule_31) + ")";
    let rule_8_string = format!("{rule_42}+", rule_42 = rule_42_string);
    let mut rule_11_string = format!(
        r"({rule_42}{{}}{rule_31}|{rule_42}{rule_31})", // with a {} hole that acts as a stand in for rule 11. we'll inflate it in a moment
        rule_42 = rule_42_string,
        rule_31 = rule_31_string
    );
    // Spend a couple of loops inflating rule 11 string to simulate recursiveness
    for _ in 0..2 {
        rule_11_string = rule_11_string.replace("{}", &rule_11_string);
    }
    rule_11_string = rule_11_string.replace("{}", ""); // remove the stand-in so we have a valid regex string

    let rule_0_string = "(?m)^".to_owned() + &rule_8_string + &rule_11_string + "\\b";
    let rule_0_regex = Regex::new(&rule_0_string).unwrap();

    strings
        .iter()
        .map(|x| if rule_0_regex.is_match(x) { 1 } else { 0 })
        .sum()
}

fn compile_rule(rules: &HashMap<usize, Vec<Token>>, rule_string: &[Token]) -> String {
    let mut built_string = String::new();

    for token in rule_string {
        match token {
            Token::Terminal(val) => built_string.push_str(val),
            Token::Or => built_string.push('|'),
            Token::RuleNum(rule) => {
                let discovered_rule_string = rules.get(rule).unwrap();
                let inner_rule = compile_rule(rules, discovered_rule_string);
                if inner_rule.len() > 1 {
                    built_string.push('(');
                    built_string.push_str(inner_rule.as_str());
                    built_string.push(')');
                } else {
                    built_string.push_str(inner_rule.as_str());
                }
            }
        }
    }

    built_string
}

fn parse_input(path: &str) -> (HashMap<usize, Vec<Token>>, Vec<String>) {
    let mut rules_map = HashMap::new();
    let file = File::open(path).unwrap();
    let mut reader = io::BufReader::new(file);

    for line in reader.by_ref().lines().take(133) {
        let line = line.unwrap();
        let rule_num = line
            .chars()
            .take_while(|x| *x != ':')
            .collect::<String>()
            .parse()
            .unwrap();
        let other_tokens: Vec<Token> = line
            .split(' ')
            .skip(1)
            .map(|x| {
                if x == "|" {
                    Token::Or
                } else if let Ok(rule) = x.parse() {
                    Token::RuleNum(rule)
                } else {
                    // If it's not numeric, or a pipe, it must be a terminal
                    Token::Terminal(x.replace("\"", ""))
                }
            })
            .collect();
        rules_map.insert(rule_num, other_tokens);
    }

    let strings = reader
        .by_ref()
        .lines()
        .filter_map(|x| {
            let line = x.unwrap();
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .collect();

    (rules_map, strings)
}

#[derive(Debug)]
enum Token {
    RuleNum(usize),
    Or,
    Terminal(String),
}
