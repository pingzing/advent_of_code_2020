use lazy_static::lazy_static;
use std::io::{self, BufRead};
use std::vec::Vec;
use std::{collections::HashSet, fs::File};

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_17/input.txt";
lazy_static! {
    static ref NEIGHBOR_OFFSETS_3D: Vec<(i32, i32, i32)> = vec![
        (-1, -1, -1),
        (0, -1, -1),
        (1, -1, -1),
        (-1, 0, -1),
        (0, 0, -1),
        (1, 0, -1),
        (-1, 1, -1),
        (0, 1, -1),
        (1, 1, -1),
        (-1, -1, 0),
        (0, -1, 0),
        (1, -1, 0),
        (-1, 0, 0),
        (1, 0, 0),
        (-1, 1, 0),
        (0, 1, 0),
        (1, 1, 0),
        (-1, -1, 1),
        (0, -1, 1),
        (1, -1, 1),
        (-1, 0, 1),
        (0, 0, 1),
        (1, 0, 1),
        (-1, 1, 1),
        (0, 1, 1),
        (1, 1, 1),
    ];
}

fn main() {
    // Offset generators
    // for i in 0..27 {
    //     println!("({}, {}, {}),", i % 3-1, i / 3 % 3 -1, i / 9 -1 );
    // }
    // for i in 0..81 {
    //     println!("({}, {}, {}, {}),", i % 3-1, i / 3 % 3-1, i / 9 % 3-1, i / 27-1,);
    // }

    let starting_state = parse_starting_state();
    let active_after_6 = simulate(&starting_state, 6);
    println!(
        "PART ONE: Active cubes after 6 iterations: {}",
        active_after_6
    );
}

fn simulate(initial_state: &HashSet<(i32, i32, i32)>, iterations: u32) -> u32 {
    let mut current_state = initial_state.clone();
    for i in 0..iterations {

        let survivors: Vec<(i32, i32, i32)> = current_state
            .iter()
            .filter(|x| matches!(get_active_neighbor_count(x, &current_state), 2 | 3))
            .map(|x| (x.0, x.1, x.2)) //force-deref
            .collect();

        let mut revival_candidates: Vec<(i32, i32, i32)> = vec![];
        for cube in current_state.iter() {
            // Get all active cubes' neighbors
            for offset in NEIGHBOR_OFFSETS_3D.iter() {
                let candidate = (cube.0 + offset.0, cube.1 + offset.1, cube.2 + offset.2);
                // Only add them if they aren't already in the active list
                if !current_state.contains(&candidate) {
                    revival_candidates.push(candidate);
                }
            }
        }

        let reviveds: Vec<(i32, i32, i32)> = revival_candidates
            .iter()
            .filter(|x| matches!(get_active_neighbor_count(x, &current_state), 2 | 3))
            .map(|x| (x.0, x.1, x.2)) //force-deref
            .collect();

        current_state = survivors
            .iter()
            .chain(reviveds.iter())
            .map(|x| (x.0, x.1, x.2)) //force-deref
            .collect();
            
    }

    current_state.len() as u32
}

fn get_active_neighbor_count(cube: &(i32, i32, i32), dimension: &HashSet<(i32, i32, i32)>) -> u32 {
    NEIGHBOR_OFFSETS_3D
        .iter()
        .map(|offset| {
            let neighbor = (cube.0 + offset.0, cube.1 + offset.1, cube.2 + offset.2);
            match dimension.contains(&neighbor) {
                true => 1,
                false => 0,
            }
        })
        .sum()
}

fn parse_starting_state() -> HashSet<(i32, i32, i32)> {
    let file = File::open(PATH).unwrap();
    let mut dimension = HashSet::new();
    for (y, row) in io::BufReader::new(file).lines().enumerate() {
        for (x, val) in row.unwrap().chars().enumerate() {
            if val == '#' {
                dimension.insert((x as i32, y as i32, 0));
            }
        }
    }

    dimension
}
