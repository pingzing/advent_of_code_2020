use std::vec::{Vec};
use std::fs::File;
use std::io::{self, BufRead};

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_3/input.txt";

fn main() {
    // Part one
    let part_1 = toboggan(3, 1);
    println!("PART ONE: Hit {} trees.", part_1);

    // Part two
    let run_1 = toboggan(1, 1);
    let run_2 = toboggan(3, 1);
    let run_3 = toboggan(5, 1);
    let run_4 = toboggan(7, 1);    
    let run_5 = toboggan(1, 2);
    println!("PART TWO: Hit multiplied together {} trees.", run_1 * run_2 * run_3 * run_4 * run_5);
}

fn toboggan(right_movement: u32, down_movement: u32) -> u32 {
    let (terrain_map, line_width) = parse_terrain_file();
    let num_lines = terrain_map.len() as u32 / line_width;

    let mut trees_hit = 0u32;
    let mut column_counter = 0u32;
    let mut line_num = 0;

    while line_num < num_lines {            
        let column_index = column_counter % line_width;
        let i = (line_width * line_num) + column_index;        
        let current_terrain = terrain_map[i as usize];
        if current_terrain == Terrain::Tree {
            trees_hit += 1;
        }

        // Move rightward and down in preparation for the next loop
        column_counter += right_movement;
        line_num += down_movement;
    }

    println!("Made it to the bottom! Hit {} trees.", trees_hit);
    return trees_hit;
}

fn parse_terrain_file() -> (Vec<Terrain>, u32) {
    let file = File::open(PATH).unwrap();
    let mut row_width: u32 = 0;

    let terrain_map = io::BufReader::new(file).lines()
        .flat_map(|x| {
            let line = x.unwrap();
            if row_width == 0 {
                row_width = line.chars().count() as u32;
            }
            line.chars().map(|y| {
                match y {
                    '#' => Terrain::Tree,
                    _ => Terrain::Open,
                }
            }).collect::<Vec<Terrain>>()
        }).collect::<Vec<Terrain>>();    

    (terrain_map, row_width)
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Terrain {
    Open,
    Tree
}

impl std::fmt::Display for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Terrain::Open => write!(f, "."),
            Terrain::Tree => write!(f, "#")
        }
    }
}