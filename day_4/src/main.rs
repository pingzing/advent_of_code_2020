use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_4/input.txt";
lazy_static! {
    static ref DOCUMENT_REGEX: Regex = Regex::new(r"([a-z]+):([^\s]+)").unwrap();
    static ref HEIGHT_REGEX: Regex = Regex::new(r"(\d+.)(cm|in)$").unwrap();
    static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"#([0-9a-z]){6}").unwrap();
}

fn main() {
    // Part one
    let documents = read_file();
    let valid_count = documents
        .iter()
        .filter(|x| validate_document(x, false))
        .count();
    println!(
        "PART ONE: Of {} documents, {} are valid passports.",
        documents.len(),
        valid_count
    );

    // Part two
    let strict_valid_count = documents
        .iter()
        .filter(|x| validate_document(x, true))
        .count();
    println!(
        "PART TWO: Of {} documents, {} are valid passports",
        documents.len(),
        strict_valid_count
    );
}

fn read_file() -> Vec<HashMap<String, String>> {
    let file = File::open(PATH).unwrap();
    let mut documents: Vec<HashMap<String, String>> = vec![];
    let lines = io::BufReader::new(file).lines();
    let mut scratch = String::new();
    for line in lines {
        let read_line = line.unwrap();        
        if read_line == "" {
            // Take our built-up record, trasnform it into a hashmap
            documents.push(parse_record(scratch.as_str()));
            scratch = String::new();
        }
        // Otherwise, keep reading
        scratch.push(' '); //add a space at the end to keep things consistent
        scratch.push_str(read_line.as_str());
    }
    // If we're done reading, but there's still something in the buffer, try to read it anyway
    documents.push(parse_record(scratch.as_str()));

    documents
}

fn parse_record(document_string: &str) -> HashMap<String, String> {    
    DOCUMENT_REGEX
        .captures_iter(document_string)
        .map(|x| {
            (
                x.get(1).unwrap().as_str().to_string(),
                x.get(2).unwrap().as_str().to_string(),
            )
        })
        .collect::<HashMap<String, String>>()
}

fn validate_document(document: &HashMap<String, String>, strict: bool) -> bool {    
    // First, check to see if everything exists
    let existence = document.contains_key("byr")
        && document.contains_key("iyr")
        && document.contains_key("eyr")
        && document.contains_key("hgt")
        && document.contains_key("hcl")
        && document.contains_key("ecl")
        && document.contains_key("pid");
    // and we don't care about cid

    if !strict || !existence {
        return existence;
    }

    let byr_valid = match document["byr"].parse().unwrap_or(-1) {
        1920..=2002 => true,
        _ => false,
    };
    let iyr_valid = match document["iyr"].parse().unwrap_or(-1) {
        2010..=2020 => true,
        _ => false,
    };
    let eyr_valid = match document["eyr"].parse().unwrap_or(-1) {
        2020..=2030 => true,
        _ => false,
    };
    let hgt_valid = validate_height(&document["hgt"]);
    let hcl_valid = HAIR_COLOR_REGEX.is_match(&document["hcl"]);
    let ecl_valid = match document["ecl"].as_str() {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    };
    // Needs to be nine digits, and also actually be digits
    let pid_valid =
        document["pid"].len() == 9 && document["pid"].parse().map_or(false, |_: u32| true);

    return existence
        && byr_valid
        && iyr_valid
        && eyr_valid
        && hgt_valid
        && hcl_valid
        && ecl_valid
        && pid_valid;
}

fn validate_height(height_string: &str) -> bool {
    if let Some(height_caps) = HEIGHT_REGEX.captures(height_string) {
        let height_unit = height_caps.get(2).unwrap().as_str();
        let height_val: u32 = height_caps.get(1).unwrap().as_str().parse().unwrap();
        match height_unit {
            "cm" => match height_val {
                150..=193 => true,
                _ => false,
            },
            "in" => match height_val {
                59..=76 => true,
                _ => false,
            },
            _ => false,
        }
    } else {
        false
    }
}
