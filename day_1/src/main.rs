use std::vec;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    part_one();
    part_two();
}

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_1/input.txt";

fn part_one() {
    let numbers = get_numbers();
    for i in numbers.iter() {
        for j in numbers.iter() {
            if i + j == 2020 {
                println!("PART ONE: The two numbers that sum to 2020 are {0} and {1}. Their product is: {2}", i, j, i * j);
                return;
            }
        }
    }
}

fn part_two() {
    let numbers = get_numbers();
    for i in numbers.iter() {
        for j in numbers.iter() {
            for k in numbers.iter() {
                if i + j + k == 2020 {
                    println!("PART TWO: The two numbers that sum to 2020 are {0}, {1} and {2}. Their product is: {3}", i, j, k, i * j * k);
                    return;
                }
            }
        }
    }
}

fn get_numbers() -> vec::Vec<u32> {
    let file = File::open(PATH).unwrap();
    io::BufReader::new(file).lines()
        .map(|x| x.unwrap().parse::<u32>().unwrap())
        .collect()
}
