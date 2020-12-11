use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_10/input.txt";

fn main() {
    let joltages = get_joltages();
    let product = construct_adapter_chain(&joltages);
    println!("PART ONE: The product of the number of one-joltages and number of three-joltages is: {}", product);

    let total_permutations = distinct_permutations(&joltages);
    println!("PART TWO: The total possible permutations of adapters is {}", total_permutations);
}

fn get_joltages() -> Vec<u32> {
    let file = File::open(PATH).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect()
}

fn construct_adapter_chain(joltages: &Vec<u32>) -> u32 {    
    let device_joltage = joltages.iter().max().unwrap() + 3u32;
    let mut sorted_joltages = joltages.clone();
    sorted_joltages.push(device_joltage);
    sorted_joltages.sort();
    
    let mut latest_joltage: u32 = 0;
    let mut one_jolts: u32 = 0;
    let mut three_jolts: u32 = 0;
    for next in sorted_joltages {        
        match next - latest_joltage {
            1 => one_jolts += 1,
            3 => three_jolts += 1,
            _ => {}
        }
        latest_joltage = next;
    }    
    
    one_jolts * three_jolts
}

fn distinct_permutations(joltages: &Vec<u32>) -> u64 {
    let device_joltage = joltages.iter().max().unwrap() + 3u32;
    let mut sorted_joltages = joltages.clone();
    sorted_joltages.push(0); // source joltage
    sorted_joltages.push(device_joltage); // device joltage
    sorted_joltages.sort();
    
    let mut memoizations: HashMap<usize, u64> = HashMap::new();    
    permutations_recursive(0, &sorted_joltages, &mut memoizations)
}

fn permutations_recursive(index: usize, joltages: &Vec<u32>, memoizations: &mut HashMap<usize, u64>) -> u64 {    
    let mut total_count = 0u64;
    if index == joltages.len() - 1 {
        return 1; //if we've made it to the end, we've found exactly one unique permutation
    }
    
    for next in index + 1..=cmp::min(index + 3, joltages.len() - 1) { // check the values at the next 3 indices
        let diff = joltages[next] - joltages[index]; 
        if diff >= 1 && diff <= 3 { // if the difference between the current value and any of the next values is between 1 and 3, it's valid
            if memoizations.contains_key(&next) { // if we've already memoized the number of permutations for how to reach this index, add it to the total
                total_count += memoizations[&next];
            } else { // otherwise, go calculate the number, store it, and add it to the total
                let permutation_count = permutations_recursive(next, joltages, memoizations);
                memoizations.insert(next, permutation_count);                
                total_count += permutation_count;
            }
        }
    }

    total_count
}