use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_13/input.txt";

fn main() {
    let (timestamp, lines) = parse_notes_part_one();
    let bus_dep_hash = part_one(timestamp, &lines);
    println!("PART ONE: {}", bus_dep_hash);

    let bus_lines_with_times = parse_notes_part_two();
    let timestamp = part_two(&bus_lines_with_times);
    println!("PART TWO: {}", timestamp);
}

fn part_one(earliest_departure: u32, lines: &[u32]) -> u32 {
    for id in lines {
        if earliest_departure % id == 0 {
            return 0; // hey, it could happen
        }
    }

    let mut best_id: u32 = 0;
    let mut smallest_wait_time = u32::MAX;
    for id in lines {
        let last_dep: u32 = (earliest_departure / id) * id;
        let next_dep: u32 = last_dep + id;
        let wait_time = next_dep - earliest_departure;
        println!(
            "Last departure for {} is: {} and next is: {} for a wait time of {}",
            id, last_dep, next_dep, wait_time
        );
        if wait_time < smallest_wait_time {
            smallest_wait_time = wait_time;
            best_id = *id;
        }
    }

    best_id * smallest_wait_time
}

fn part_two(lines: &[BusSchedule]) -> u64 {
    let bus_ids_sum = lines.iter().fold(1, |acc, bus| acc * bus.id);
    lines
        .iter()
        .skip(1)
        .map(|bus| {
            let bus_q_without_r = bus_ids_sum / bus.id;
            (bus.time * euclid_inverse(bus_q_without_r as i64, bus.id as i64) * bus_q_without_r)
                % bus_ids_sum
        })
        .sum::<u64>()
        % bus_ids_sum
}

fn parse_notes_part_one() -> (u32, Vec<u32>) {
    let file = File::open(PATH).unwrap();
    let mut lines_iter = io::BufReader::new(file).lines();
    let arrival_timestamp: u32 = lines_iter.next().unwrap().unwrap().parse().unwrap();
    let bus_lines: Vec<u32> = lines_iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse().unwrap())
        .collect();

    (arrival_timestamp, bus_lines)
}

fn parse_notes_part_two() -> Vec<BusSchedule> {    
    let file = File::open(PATH).unwrap();
    let mut lines_iter = io::BufReader::new(file).lines();
    let bus_lines: Vec<BusSchedule> = lines_iter
        .nth(1)
        .unwrap()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, entry)| *entry != "x" )
        .map(|(i, entry)| {
            let val = entry.parse().unwrap();
            BusSchedule {
                id: val,
                time: val - (i as u64 % val)
            }
        })    
        .collect();

    bus_lines
}

fn euclid_inverse(mut a: i64, mut b: i64) -> u64 {
    if b == 1 {
        return 1;
    }

    let b0 = b;
    let mut t;
    let mut q;
    let mut x0 = 0;
    let mut x1 = 1;
    while a > 1 {
        q = a / b;
        t = b;
        b = a % b;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 {
        x1 += b0;
    }

    x1 as u64
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct BusSchedule {
    id: u64,
    time: u64,
}
