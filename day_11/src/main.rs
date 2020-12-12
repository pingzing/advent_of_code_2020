#![allow(clippy::clippy::ptr_arg, clippy::clippy::needless_range_loop)]

use std::cmp;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_11/input.txt";
type FerryMap = Vec<Vec<SpotState>>;

fn main() {
    let ferry_map = parse_file();
    let occupied_count = count_stable_occupieds(ferry_map.clone(), true);
    println!(
        "PART ONE: Once stabilized, {} seats are occupied.",
        occupied_count
    );

    let occupied_count_mk_2 = count_stable_occupieds(ferry_map, false);
    println!(
        "PART TWO: Once stablizied, {} seats using line-of-sight are occupied.",
        occupied_count_mk_2
    );
}

fn parse_file() -> FerryMap {
    let file = File::open(PATH).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|x| {
            let string = x.unwrap();
            string.chars().map(|y| y.into()).collect()
        })
        .collect::<FerryMap>()
}

fn count_stable_occupieds(map: FerryMap, is_part_one: bool) -> u32 {
    let mut previous_map = map;
    loop {
        let new_map = if is_part_one {
            apply_rules_part_one(&previous_map)
        } else {
            apply_rules_part_two(&previous_map)
        };

        if ferry_maps_equal(&previous_map, &new_map) {
            // we're done! count up and return
            return new_map
                .iter()
                .flatten()
                .filter(|x| matches!(**x, SpotState::Occupied))
                .count() as u32;
        }

        previous_map = new_map;
    }
}

fn apply_rules_part_one(map: &FerryMap) -> FerryMap {
    let row_len = map[0].len();
    let mut new_map = map.clone();
    for r in 0..new_map.len() {
        for c in 0..row_len {
            let seat = &mut new_map[r][c];
            match seat {
                SpotState::Floor => continue,
                SpotState::Empty => {
                    let adjacents = get_direct_adjacents(r, c, row_len, map);
                    let occupied_count = adjacents
                        .iter()
                        .filter(|x| ***x == SpotState::Occupied)
                        .count() as u32;
                    if occupied_count == 0 {
                        *seat = SpotState::Occupied;
                    }
                }
                SpotState::Occupied => {
                    let adjacents = get_direct_adjacents(r, c, row_len, map);
                    let occupied_count = adjacents
                        .iter()
                        .filter(|x| ***x == SpotState::Occupied)
                        .count() as u32;
                    if occupied_count >= 4 {
                        *seat = SpotState::Empty;
                    }
                }
            }
        }
    }

    new_map
}

fn get_direct_adjacents(row: usize, col: usize, row_len: usize, map: &FerryMap) -> Vec<&SpotState> {
    let mut adjacents: Vec<&SpotState> = Vec::with_capacity(8); // max of 8

    // Above
    if row > 0 {
        // Left
        if col > 0 {
            adjacents.push(&map[row - 1][col - 1]);
        }
        // Directly above
        adjacents.push(&map[row - 1][col]);
        // Right
        if col < row_len - 1 {
            adjacents.push(&map[row - 1][col + 1]);
        }
    }

    // Same row, left
    if col > 0 {
        adjacents.push(&map[row][col - 1]);
    }
    // Same row, right
    if col < row_len - 1 {
        adjacents.push(&map[row][col + 1]);
    }

    // Below
    if row < map.len() - 1 {
        // Left
        if col > 0 {
            adjacents.push(&map[row + 1][col - 1]);
        }
        // Directly below
        adjacents.push(&map[row + 1][col]);
        // Right
        if col < row_len - 1 {
            adjacents.push(&map[row + 1][col + 1]);
        }
    }

    adjacents
}

fn apply_rules_part_two(map: &FerryMap) -> FerryMap {
    let row_len = map[0].len();
    let mut new_map = map.clone();
    for r in 0..new_map.len() {
        for c in 0..row_len {
            let seat = &mut new_map[r][c];
            match seat {
                SpotState::Floor => continue,
                SpotState::Empty => {
                    let adjacents = get_line_of_sight_adjacents(r, c, row_len, map);
                    let occupied_count = adjacents
                        .iter()
                        .filter(|x| ***x == SpotState::Occupied)
                        .count() as u32;
                    if occupied_count == 0 {
                        *seat = SpotState::Occupied;
                    }
                }
                SpotState::Occupied => {
                    let adjacents = get_line_of_sight_adjacents(r, c, row_len, map);
                    let occupied_count = adjacents
                        .iter()
                        .filter(|x| ***x == SpotState::Occupied)
                        .count() as u32;
                    if occupied_count >= 5 {
                        *seat = SpotState::Empty;
                    }
                }
            }
        }
    }

    new_map
}

fn get_line_of_sight_adjacents(
    row: usize,
    col: usize,
    row_len: usize,
    map: &FerryMap,
) -> Vec<&SpotState> {
    let mut adjacents: Vec<&SpotState> = Vec::with_capacity(8); // max of 8
    let col_len = map.len();

    // Up-left
    let max_ul_offset = cmp::min(row, col);
    if max_ul_offset > 0 {
        for offset in 1..=max_ul_offset {
            match map[row - offset][col - offset] {
                SpotState::Floor => continue,
                SpotState::Occupied | SpotState::Empty => {
                    adjacents.push(&map[row - offset][col - offset]);
                    break;
                }
            }
        }
    }

    // Up
    if row > 0 {
        for offset in 1..=row {
            match map[row - offset][col] {
                SpotState::Floor => continue,
                SpotState::Occupied | SpotState::Empty => {
                    adjacents.push(&map[row - offset][col]);
                    break;
                }
            }
        }
    }

    // Up-right
    let max_ur_offset = cmp::min(row, row_len - col - 1);
    if max_ur_offset > 0 {
        for offset in 1..=max_ur_offset {
            match map[row - offset][col + offset] {
                SpotState::Floor => continue,
                SpotState::Occupied | SpotState::Empty => {
                    adjacents.push(&map[row - offset][col + offset]);
                    break;
                }
            }
        }
    }

    // Directly left
    if col > 0 {
        for offset in 1..=col {
            match map[row][col - offset] {
                SpotState::Floor => continue,
                SpotState::Occupied | SpotState::Empty => {
                    adjacents.push(&map[row][col - offset]);
                    break;
                }
            }
        }
    }

    // Directly right
    if row_len - col - 1 > 0 {
        for offset in 1..=row_len - col - 1 {
            match map[row][col + offset] {
                SpotState::Floor => continue,
                SpotState::Occupied | SpotState::Empty => {
                    adjacents.push(&map[row][col + offset]);
                    break;
                }
            }
        }
    }

    // Down-left
    let max_dl_offset = cmp::min(col_len - row - 1, col);
    if max_dl_offset > 0 {
        for offset in 1..=max_dl_offset {
            match map[row + offset][col - offset] {
                SpotState::Floor => continue,
                SpotState::Occupied | SpotState::Empty => {
                    adjacents.push(&map[row + offset][col - offset]);
                    break;
                }
            }
        }
    }

    //Down
    if col_len - row - 1 > 0 {
        for offset in 1..=col_len - row - 1 {
            match map[row + offset][col] {
                SpotState::Floor => continue,
                SpotState::Occupied | SpotState::Empty => {
                    adjacents.push(&map[row + offset][col]);
                    break;
                }
            }
        }
    }

    // Down-right
    let max_dr_offset = cmp::min(col_len - row - 1, row_len - col - 1);
    if max_dr_offset > 0 {
        for offset in 1..=max_dr_offset {
            match map[row + offset][col + offset] {
                SpotState::Floor => continue,
                SpotState::Occupied | SpotState::Empty => {
                    adjacents.push(&map[row + offset][col + offset]);
                    break;
                }
            }
        }
    }

    adjacents
}

fn ferry_maps_equal(map_a: &FerryMap, map_b: &FerryMap) -> bool {
    let equal_length: bool = map_a.len() == map_b.len();
    if !equal_length {
        return false;
    }

    for (row_a, row_b) in map_a.iter().zip(map_b.iter()) {
        if row_a.len() != row_b.len() {
            return false;
        }
        for (col_a, col_b) in row_a.iter().zip(row_b.iter()) {
            if col_a != col_b {
                return false;
            }
        }
    }

    true
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SpotState {
    Floor,
    Occupied,
    Empty,
}

impl Display for SpotState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chr: char = Into::<char>::into(*self);
        write!(f, "{}", chr)
    }
}

impl From<char> for SpotState {
    fn from(chr: char) -> SpotState {
        match chr {
            '.' => SpotState::Floor,
            '#' => SpotState::Occupied,
            'L' => SpotState::Empty,
            _ => panic!("Unknown value."),
        }
    }
}

impl From<SpotState> for char {
    fn from(s: SpotState) -> char {
        match s {
            SpotState::Floor => '.',
            SpotState::Occupied => '#',
            SpotState::Empty => 'L',
        }
    }
}
