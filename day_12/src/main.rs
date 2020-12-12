use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_12/input.txt";

fn main() {
    let directions = get_directions();
    let manhattan_distance = get_manhattan_distance(&directions);
    println!("PART ONE: The man(e)hattan distance is: {}", manhattan_distance);

    let waypoint_manhattan_distance = get_manhattan_distance_via_waypoint(&directions);
    println!("PART TWO: The man(e)hattan distance by traveling via waypoint is {}", waypoint_manhattan_distance);
}

fn get_manhattan_distance(directions: &[Direction]) -> u32 {
    let mut ship_y = 0i32; // negative is south, positive is north
    let mut ship_x = 0i32; // negative is west, positive is east
    let mut facing_deg = 90u32;

    for direction in directions.iter() {        
        match direction {
            Direction::North(val) => { ship_y += *val as i32; },
            Direction::South(val) => { ship_y -= *val as i32; },
            Direction::East(val) => { ship_x += *val as i32; },
            Direction::West(val) => { ship_x -= *val as i32; },
            Direction::Left(val) => { facing_deg = add_degrees(facing_deg, -(*val as i32)); },
            Direction::Right(val) => { facing_deg = add_degrees(facing_deg, *val as i32); },
            Direction::Forward(val) => {
                match facing_deg {
                    0 => { ship_y += *val as i32 },
                    90 => { ship_x += *val as i32 },
                    180 => { ship_y -= *val as i32 },
                    270 => { ship_x -= *val as i32 },
                    _ => panic!("Unexpected facing degree when trying to move forward")
                }
            }
        }        
    }

    ship_y.abs() as u32 + ship_x.abs() as u32
}

fn get_manhattan_distance_via_waypoint(directions: &[Direction]) -> u32 {
    let mut waypoint_y = 1i32; // aka north/south
    let mut waypoint_x = 10i32; // aka east/west

    let mut ship_y = 0i32; // negative is south, positive is north
    let mut ship_x = 0i32; // negative is west, positive is east    

    for direction in directions.iter() {
        match direction {
            Direction::North(val) => { waypoint_y += *val as i32; },
            Direction::South(val) => { waypoint_y -= *val as i32; },
            Direction::East(val) => { waypoint_x += *val as i32; },
            Direction::West(val) => { waypoint_x -= *val as i32; },
            Direction::Left(val) => { 
                let (new_x, new_y) = rotate_point(waypoint_x, waypoint_y, -(*val as i32)); 
                waypoint_x = new_x;
                waypoint_y = new_y;
            },
            Direction::Right(val) => { 
                let (new_x, new_y) = rotate_point(waypoint_x, waypoint_y, *val as i32);
                waypoint_x = new_x;
                waypoint_y = new_y;
            },
            Direction::Forward(val) => {
                ship_x += waypoint_x * (*val as i32);
                ship_y += waypoint_y * (*val as i32);                                
            }
        }                
    }
    

    ship_y.abs() as u32 + ship_x.abs() as u32
}

fn get_directions() -> Vec<Direction> {
    let file = File::open(PATH).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|x| {
            let line = x.unwrap();
            let direc_char = line.chars().next().unwrap();
            let direc_val = line.chars().skip(1).collect::<String>().parse().unwrap();
            match direc_char {
                'N' => Direction::North(direc_val),
                'S' => Direction::South(direc_val),
                'E' => Direction::East(direc_val),
                'W' => Direction::West(direc_val),
                'L' => Direction::Left(direc_val),
                'R' => Direction::Right(direc_val),
                'F' => Direction::Forward(direc_val),
                _ => panic!("Unsupported direction!"),
            }
        })
        .collect()
}

fn rotate_point(x: i32, y: i32, angle: i32) -> (i32, i32) {
    match angle {        
        -90 | 270 => { (-y, x) },
        -180 | 180 => { (-x, -y) },
        -270 | 90 => { (y, -x) },
        _ => panic!("Angle that wasn't a multiple of 90!")
    }        
}

fn add_degrees(degrees: u32, operand: i32) -> u32 {
    let degrees = degrees as i32;
    let mut new_degrees = degrees + operand;
    if new_degrees < 0 {
        new_degrees += 360;
    }

    new_degrees as u32 % 360
}

#[derive(Debug)]
enum Direction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}
