use lazy_static::lazy_static;
use std::vec::Vec;
use std::{collections::HashSet, fs::File};
use std::{
    io::{self, BufRead},
    ops::Add,
};

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
    static ref NEIGHBOR_OFFSETS_4D: Vec<(i32, i32, i32, i32)> = vec![
        (-1, -1, -1, -1),
        (0, -1, -1, -1),
        (1, -1, -1, -1),
        (-1, 0, -1, -1),
        (0, 0, -1, -1),
        (1, 0, -1, -1),
        (-1, 1, -1, -1),
        (0, 1, -1, -1),
        (1, 1, -1, -1),
        (-1, -1, 0, -1),
        (0, -1, 0, -1),
        (1, -1, 0, -1),
        (-1, 0, 0, -1),
        (0, 0, 0, -1),
        (1, 0, 0, -1),
        (-1, 1, 0, -1),
        (0, 1, 0, -1),
        (1, 1, 0, -1),
        (-1, -1, 1, -1),
        (0, -1, 1, -1),
        (1, -1, 1, -1),
        (-1, 0, 1, -1),
        (0, 0, 1, -1),
        (1, 0, 1, -1),
        (-1, 1, 1, -1),
        (0, 1, 1, -1),
        (1, 1, 1, -1),
        (-1, -1, -1, 0),
        (0, -1, -1, 0),
        (1, -1, -1, 0),
        (-1, 0, -1, 0),
        (0, 0, -1, 0),
        (1, 0, -1, 0),
        (-1, 1, -1, 0),
        (0, 1, -1, 0),
        (1, 1, -1, 0),
        (-1, -1, 0, 0),
        (0, -1, 0, 0),
        (1, -1, 0, 0),
        (-1, 0, 0, 0),    
        (1, 0, 0, 0),
        (-1, 1, 0, 0),
        (0, 1, 0, 0),
        (1, 1, 0, 0),
        (-1, -1, 1, 0),
        (0, -1, 1, 0),
        (1, -1, 1, 0),
        (-1, 0, 1, 0),
        (0, 0, 1, 0),
        (1, 0, 1, 0),
        (-1, 1, 1, 0),
        (0, 1, 1, 0),
        (1, 1, 1, 0),
        (-1, -1, -1, 1),
        (0, -1, -1, 1),
        (1, -1, -1, 1),
        (-1, 0, -1, 1),
        (0, 0, -1, 1),
        (1, 0, -1, 1),
        (-1, 1, -1, 1),
        (0, 1, -1, 1),
        (1, 1, -1, 1),
        (-1, -1, 0, 1),
        (0, -1, 0, 1),
        (1, -1, 0, 1),
        (-1, 0, 0, 1),
        (0, 0, 0, 1),
        (1, 0, 0, 1),
        (-1, 1, 0, 1),
        (0, 1, 0, 1),
        (1, 1, 0, 1),
        (-1, -1, 1, 1),
        (0, -1, 1, 1),
        (1, -1, 1, 1),
        (-1, 0, 1, 1),
        (0, 0, 1, 1),
        (1, 0, 1, 1),
        (-1, 1, 1, 1),
        (0, 1, 1, 1),
        (1, 1, 1, 1),
    ];
}

fn main() {
    // Offset generators
    // for i in 0..27 {
    //     println!("({}, {}, {}),", i % 3-1, i / 3 % 3 -1, i / 9 -1 );
    // }
    // for i in 0..81 {
    //     println!("({}, {}, {}, {}),", i % 3 - 1, i / 3 % 3 - 1, i / 9 % 3 - 1, i / 27 - 1, );
    // }

    let starting_state = parse_starting_state();
    let in_3d = simulate(&starting_state, 6, Dimensions::Three);
    println!("PART ONE: Active cubes after 6 iterations: {}", in_3d);

    let in_4d = simulate(&starting_state, 6, Dimensions::Four);
    println!("PART TWO: Active cubes after 6 iterations in 4D: {}", in_4d);
}

fn simulate(initial_state: &HashSet<Cube>, iterations: u32, dimensions: Dimensions) -> u32 {
    let mut current_state = initial_state.clone();
    for _ in 0..iterations {
        let survivors: Vec<Cube> = current_state
            .iter()
            .filter(|x| {
                matches!(
                    get_active_neighbor_count(x, &current_state, dimensions),
                    2 | 3
                )
            })
            .copied()
            .collect();

        let mut revival_candidates: Vec<Cube> = vec![];
        for cube in current_state.iter() {
            // Get all active cubes' neighbors
            if dimensions == Dimensions::Three {
                for offset in NEIGHBOR_OFFSETS_3D.iter() {
                    let candidate = *cube + *offset;
                    // Only add them if they aren't already in the active list
                    if !current_state.contains(&candidate) {
                        revival_candidates.push(candidate);
                    }
                }
            } else if dimensions == Dimensions::Four {
                for offset in NEIGHBOR_OFFSETS_4D.iter() {
                    let candidate = *cube + *offset;
                    // Only add them if they aren't already in the active list
                    if !current_state.contains(&candidate) {
                        revival_candidates.push(candidate);
                    }
                }
            }
        }

        let reviveds: Vec<Cube> = revival_candidates
            .iter()
            .filter(|x| matches!(get_active_neighbor_count(x, &current_state, dimensions), 3))
            .copied()
            .collect();

        current_state = survivors.iter().chain(reviveds.iter()).copied().collect();
    }

    current_state.len() as u32
}

fn get_active_neighbor_count(
    cube: &Cube,
    dimension: &HashSet<Cube>,
    dimensions: Dimensions,
) -> u32 {
    let mut active_count = 0u32;
    if dimensions == Dimensions::Three {
        for offset in NEIGHBOR_OFFSETS_3D.iter() {
            let neighbor = *cube + *offset;
            if dimension.contains(&neighbor) {
                active_count += 1;
            }
        }
    } else if dimensions == Dimensions::Four {
        for offset in NEIGHBOR_OFFSETS_4D.iter() {
            let neighbor = *cube + *offset;
            if dimension.contains(&neighbor) {
                active_count += 1;
            }
        }
    }

    active_count
}

fn parse_starting_state() -> HashSet<Cube> {
    let file = File::open(PATH).unwrap();
    let mut dimension = HashSet::new();
    for (y, row) in io::BufReader::new(file).lines().enumerate() {
        for (x, val) in row.unwrap().chars().enumerate() {
            if val == '#' {
                dimension.insert(Cube {
                    x: x as i32,
                    y: y as i32,
                    z: 0,
                    w: 0,
                });
            }
        }
    }

    dimension
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Dimensions {
    Three,
    Four,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Add for Cube {
    type Output = Cube;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl std::ops::Add<(i32, i32, i32)> for Cube {
    type Output = Cube;

    fn add(self, rhs: (i32, i32, i32)) -> Cube {
        Cube {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
            w: self.w,
        }
    }
}

impl std::ops::Add<(i32, i32, i32, i32)> for Cube {
    type Output = Cube;

    fn add(self, rhs: (i32, i32, i32, i32)) -> Cube {
        Cube {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
            w: self.w + rhs.3,
        }
    }
}
