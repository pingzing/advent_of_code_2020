use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_9/input.txt";

fn main() {
    let data_stream = read_numbers();
    let first_non_summing_value = first_non_sum(&data_stream);
    println!("PART ONE: First number that doesn't sum over the previous 25 is: {}", first_non_summing_value.unwrap());

    let (smallest, largest) = find_contiguous_sum(&data_stream, first_non_summing_value.unwrap());
    println!("PART TWO: The smallest and largest values of the contiguous values are {} and {} which add to {}", smallest, largest, smallest + largest);
}

fn read_numbers() -> Vec<u64> {
    let file = File::open(PATH).unwrap();
    io::BufReader::new(file).lines().map(|x| {
        let string = x.unwrap();
        return string.trim().parse().unwrap();
    }).collect()
}

fn first_non_sum(data_stream: &Vec<u64>) -> Option<u64> {
    for (i, value) in data_stream.iter().enumerate().skip(25) {
        let previous_25 = &data_stream[i-25..i];
        let mut found: bool = false;
        for (outer_idx, outer) in previous_25.iter().enumerate() {            
            for (inner_idx, inner) in previous_25.iter().enumerate() {
                if outer_idx == inner_idx { // can't add a number up to itself--that's not a pair
                    continue;
                }
                if outer + inner == *value {
                    found = true;
                    break;
                }
            }
            if found == true {
                break;
            }
        }
        if found == false {
            return Some(*value);
        }
    }
    return None;
}

fn find_contiguous_sum(data_stream: &Vec<u64>, first_invalid: u64) -> (u64, u64) {
    for (upper_idx, _) in data_stream.iter().enumerate() {
        for lower_idx in upper_idx + 1..data_stream.len() {
            let contiguous_sum: u64 = data_stream[upper_idx..=lower_idx].iter().sum();
            if contiguous_sum == first_invalid {
                let smallest = data_stream[upper_idx..=lower_idx].iter().min().unwrap();
                let biggest = data_stream[upper_idx..=lower_idx].iter().max().unwrap();
                return (*smallest, *biggest);
            }

            if contiguous_sum > first_invalid {
                break;
            }
        }       
    }

    return (0, 0);
}
