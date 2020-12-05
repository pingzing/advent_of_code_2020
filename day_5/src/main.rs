use std::fmt;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec::Vec;
use std::str;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_5/input.txt";

fn main() {
    // Part one
    let split_strings = parse_file();
    let mut boarding_passes = parse_boarding_passes(split_strings);
    boarding_passes.sort();
    println!("The highest seat ID on a boarding pass is {}", boarding_passes.iter().max_by(|x, y| x.id.cmp(&y.id)).unwrap().id);

    // Part two
    println!("All seats in order:");
    for pass in &boarding_passes {
        println!("{}", pass);
    }

    print_seat_candidates(&boarding_passes);
}

fn parse_file() -> Vec<(String, String)> {
    let file = File::open(PATH).unwrap();
    io::BufReader::new(file).lines().map(|x| {
        let line = x.unwrap();
        let ascii_line = line.as_bytes();
        (str::from_utf8(&ascii_line[0..7]).unwrap().to_string(), str::from_utf8(&ascii_line[7..10]).unwrap().to_string())
    }).collect()
}

fn parse_boarding_passes(boarding_passes: Vec<(String, String)>) -> Vec<BoardingPass> {
    boarding_passes.iter().map(|x| {        
        let row_num = subdivide_recursively(x.0.as_bytes(), 0 as usize, 0, 127, 'F', 'B');
        let seat_num = subdivide_recursively(x.1.as_bytes(), 0 as usize, 0, 7, 'L', 'R');
        let seat_id = row_num * 8 + seat_num;        
        return BoardingPass { row: row_num, seat: seat_num, id: seat_id };
    }).collect()
}

fn subdivide_recursively(list: &[u8], current_index: usize, current_min: u32, current_max: u32, go_smaller: char, go_bigger: char) -> u32 {
    // Base case
    if current_index > list.len() - 1 {
        return current_max;
    }        

    // Recursive logic
    let range = current_min..=current_max;    
    if list[current_index] as char == go_smaller {        
        let new_max = range.clone().nth_back(range.count() / 2).unwrap();
        return subdivide_recursively(list, current_index + 1, current_min, new_max, go_smaller, go_bigger);
    }
    else if list[current_index] as char == go_bigger {        
        let new_min = range.clone().nth(range.count() / 2).unwrap();
        return subdivide_recursively(list, current_index + 1, new_min, current_max, go_smaller, go_bigger);
    }

    panic!("Unsupported char: {}", list[current_index]);
}

fn print_seat_candidates(boarding_passes: &Vec<BoardingPass>) {
    let mut peekable_iter= boarding_passes.iter().peekable();
    while let Some(curr) = peekable_iter.next() {
        if let Some(next) = peekable_iter.peek() {
            match curr.seat {                
                 0 | 1 | 2 | 3 | 4| 5 | 6 => {
                    let expected_seat = curr.seat + 1;
                    if next.seat != expected_seat {
                        println!("Possible seat after: {} with ID {}", curr, curr.row * 8 + curr.seat + 1);
                    }
                },
                7 => {
                    let expected_seat = 0;
                    if next.seat != expected_seat {
                        println!("Possible seat after: {} with ID {}", curr, curr.row + 1 * 8 + 0);
                    }
                }
                _ => panic!("Unsupported seat number.")
            }
        } else {
            break;
        }
    }    
}

#[derive(Eq)]
struct BoardingPass {
    row: u32,
    seat: u32,
    id: u32
}

impl fmt::Display for BoardingPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Row {}, Seat {}, ID: {}", self.row, self.seat, self.id)
    }
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        let row_order = self.row.cmp(&other.row);
        if row_order == Ordering::Equal {
            return self.seat.cmp(&other.seat);
        }

        return row_order;
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
        && self.seat == other.seat
        && self.id == other.id
    }
}
