use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{hash_map::Entry, HashMap},
    io::{self, BufRead},
};
use std::{fs::File, io::Read};
use std::{ops::RangeInclusive, vec::Vec};

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_16/input.txt";
lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r"([a-z| ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

fn main() {
    let (rules, my_ticket, other_tickets) = parse_inputs();
    let error_rate = get_other_tickets_error_rate(&other_tickets, &rules);
    println!("PART ONE: The nearby ticket error rate is: {}", error_rate);

    let departure_values = get_departure_values(&rules, my_ticket, &other_tickets);
    println!("PART TWO: The multipled 'departure' vales on your ticket are: {}", departure_values);
}

fn get_other_tickets_error_rate(tickets: &[Ticket], rules: &[Rule]) -> u32 {
    tickets
        .iter()
        .map(|ticket| {
            ticket
                .fields
                .iter()
                .map(|val| {
                    if !rules.iter().any(|rule| is_value_valid(*val, rule)) {
                        *val
                    } else {
                        0u32
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

fn get_departure_values(rules: &[Rule], my_ticket: Ticket, other_tickets: &[Ticket]) -> u64 {
    // Remove invalid tickets
    let valid_tickets: Vec<&Ticket> = other_tickets
        .iter()
        .filter(|x| is_ticket_valid(*x, rules))
        .collect();

    // Begin process of elimination
    let mut candidates: HashMap<&str, Vec<u32>> = HashMap::new();
    for rule in rules.iter() {
        let vec = vec![
            0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ];
        candidates.insert(&rule.name, vec);
    }

    // Run through each rule once--one of these fields should only have one possible candidate
    let mut established_positions: HashMap<&str, u32> = HashMap::new();
    for rule in rules.iter() {
        for ticket in valid_tickets.iter() {
            for (idx, field) in ticket.fields.iter().enumerate() {
                if !is_value_valid(*field, rule) {
                    if let Entry::Occupied(mut occ) = candidates.entry(&rule.name) {
                        let vec = occ.get_mut();
                        if let Some(pos) = vec.iter().position(|x| *x == idx as u32) {
                            vec.remove(pos);
                        }
                    }
                }
            }
        }
    }

    loop {
        // Look for any candidates that are only valid for one field and:
        // - remove them from candidates,
        // - put them in established
        // - remove that position from all remaining candidate arrays

        // Remove from candidates, tracking which positions got removed
        let mut positions_to_remove: Vec<u32> = vec![];
        candidates.retain(|k, v| {
            if v.len() == 1 {
                let pos = *v.first().unwrap();
                positions_to_remove.push(pos);
                established_positions.insert(*k, pos);
                false
            } else {
                true
            }
        });

        // Remove that position from all candidate arrays
        if !positions_to_remove.is_empty() {
            for pos in positions_to_remove.iter() {
                for (_, v) in candidates.iter_mut() {
                    v.retain(|x| *x != *pos);
                }
            }
        }

        println!("Candidates list: ");
        for candidate in candidates.iter() {
            println!("{:?}", candidate);
        }
        println!("Establisheds list: ");
        for est in established_positions.iter() {
            println!("{:?}", est);
        }
        println!();

        if candidates.is_empty() {
            break;
        }
    }

    // Get the indices/positions of the values that represent 'departure ' something.
    let departure_indices: Vec<u32> = established_positions
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with("departure") {
                Some(*v)
            } else {
                None
            }
        })
        .collect();

    assert!(departure_indices.len() == 6, "Got too many indices that we think are departures.");    

    // Filter out everything that _isn't_ at one of those indices
    let departure_values: Vec<u64> = my_ticket.fields.iter().enumerate().filter_map(|(idx, value)| {
        if departure_indices.contains(&(idx as u32)) {
            Some(*value as u64)            
        } else {
            None
        }
    }).collect();
    
    // And multiply 'em all together
    departure_values.iter().product::<u64>()
}

fn is_ticket_valid(ticket: &Ticket, rules: &[Rule]) -> bool {
    ticket
        .fields
        .iter()
        .all(|x| rules.iter().any(|rule| is_value_valid(*x, rule)))
}

fn is_value_valid(value: u32, rule: &Rule) -> bool {
    rule.ranges.0.contains(&value) || rule.ranges.1.contains(&value)
}

fn parse_inputs() -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let file = File::open(PATH).unwrap();
    let mut reader = io::BufReader::new(&file);

    let rules: Vec<Rule> = reader
        .by_ref()
        .lines()
        .take(20)
        .map(|x| {
            let line = x.unwrap();
            let caps = RULE_REGEX.captures(&line).unwrap();
            let name = caps.get(1).unwrap().as_str().to_string();
            let range_1 = caps.get(2).unwrap().as_str().parse().unwrap()
                ..=caps.get(3).unwrap().as_str().parse().unwrap();
            let range_2 = caps.get(4).unwrap().as_str().parse().unwrap()
                ..=caps.get(5).unwrap().as_str().parse().unwrap();
            Rule {
                name,
                ranges: (range_1, range_2),
            }
        })
        .collect();

    let fields: Vec<u32> = reader
        .by_ref()
        .lines()
        .nth(2)
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let ticket = Ticket { fields };

    let tickets: Vec<Ticket> = reader
        .by_ref()
        .lines()
        .skip(2)
        .map(|x| {
            let line = x.unwrap();
            let fields: Vec<u32> = line.split(',').map(|y| y.parse().unwrap()).collect();
            Ticket { fields }
        })
        .collect();

    (rules, ticket, tickets)
}

struct Rule {
    name: String,
    ranges: (RangeInclusive<u32>, RangeInclusive<u32>),
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u32>,
}
