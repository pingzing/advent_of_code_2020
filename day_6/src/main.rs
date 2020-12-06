use std::{collections::HashMap, str::Split, fs::{read_to_string}};

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_6/input.txt";

fn main() {    

    // part one
    let input_str = read_to_string(PATH).unwrap();    
    let yes_sum: u32 = part_one(input_str.split("\r\n\r\n"));
    println!("PART ONE: The number of yesses is {}", yes_sum);

    // part two
    let all_yes_sum = part_two(input_str.split("\r\n\r\n"));
    println!("PART TWO: The sum of yesses for everyone on a card is {}", all_yes_sum);
}

fn part_one(inputs: Split<&str>) -> u32 {
    let mut unique_letters: HashMap<u8, bool> = HashMap::new();
    inputs.map(|x| {
        unique_letters.clear();
        let filtered_string: String = x.chars().filter(|c| !c.is_whitespace()).collect();                
        for byte in filtered_string.bytes() {
            unique_letters.insert(byte, true);
        }
        return unique_letters.keys().count() as u32;
    }).sum()
}

fn part_two(inputs: Split<&str>) -> u32 {
    inputs.map(|x| {        
        let mut yes_counts: HashMap<char, u32> = HashMap::new();        
        for line in x.lines() {
            for byte in line.bytes() {       
                let yes_count = yes_counts.entry(byte as char).or_insert(0);
                *yes_count += 1;
            }
        }

        let count = yes_counts.iter().filter(|kv| *kv.1 == x.lines().count() as u32).count() as u32;        
        return count;
    }).sum()
}