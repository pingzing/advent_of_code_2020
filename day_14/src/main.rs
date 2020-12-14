use bitvec::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufRead};
use std::vec::Vec;
use std::{collections::HashMap, fs::File};

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_14/input.txt";
lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new(r"mask = ([1|0|X]+)").unwrap();
    static ref INSTR_REGEX: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

fn main() {
    let instruction_blocks = read_instruction_blocks();
    let leftovers_sum = get_leftovers_sum_v1(&instruction_blocks);
    println!("PART ONE: {}", leftovers_sum);

    let address_mod_sum = get_leftovers_sum_v2(&instruction_blocks);
    println!("PART TWO: {}", address_mod_sum);
}

fn get_leftovers_sum_v1(instruction_blocks: &[InstructionBlock]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for block in instruction_blocks.iter() {
        let mask = &block.mask;
        for instruction in &block.instructions {
            let mut value = instruction.value;            
            let bits = &mut value.view_bits_mut::<Lsb0>()[0..36];
            let mask_bits = mask.chars().rev();
            // apply bitmask to value
            for (bit, mask_bit) in bits.iter_mut().zip(mask_bits) {
                match mask_bit {
                    'X' => {
                        continue;
                    }
                    '0' => {
                        bit.set(false);
                    }
                    '1' => {
                        bit.set(true);
                    }
                    _ => panic!("Unknown value in bitmask"),
                }
            }
            
            // Write modified value to memory
            memory.insert(instruction.address, value);
        }
    }

    memory
        .iter()
        .filter_map(|(_, v)| if *v != 0 { Some(*v) } else { None })
        .sum()
}

fn get_leftovers_sum_v2(instruction_blocks: &[InstructionBlock]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for block in instruction_blocks.iter() {
        let mask = &block.mask;        
        for instruction in &block.instructions {
            let immut_address = instruction.address;            
            let address_bits = &immut_address.view_bits::<Lsb0>()[0..36];            
            update_recursive(&mut memory, address_bits, 0, 0, mask, instruction.value);
        }
    }
    
    memory
        .iter()
        .filter_map(|(_, v)| if *v != 0 { Some(*v) } else { None })
        .sum()
}

fn update_recursive(
    memory: &mut HashMap<u64, u64>,
    original_address: &BitSlice<Lsb0, u64>,
    built_address: u64,
    index: usize,
    mask: &str,
    value: u64,
) {
    if index == mask.len() {                      
        memory.insert(built_address, value);
        return;
    }
            
    match mask.chars().rev().nth(index).unwrap() {
        'X' => {      
            let mut address_1 = built_address;
            let address_1_bits = address_1.view_bits_mut::<Lsb0>();                        
            address_1_bits.get_mut(index).unwrap().set(true);
            update_recursive(memory, original_address, BitField::load(address_1_bits), index + 1, mask, value);
            let mut address_2 = built_address;
            let address_2_bits = address_2.view_bits_mut::<Lsb0>();
            address_2_bits.get_mut(index).unwrap().set(false);
            update_recursive(memory, original_address, BitField::load(address_2_bits), index + 1, mask, value)
        },
        '0' => {            
            let mut address = built_address;            
            let address_bits = address.view_bits_mut::<Lsb0>();
            let original_value = *original_address.get(index).unwrap();
            address_bits.get_mut(index).unwrap().set(original_value);            
            update_recursive(memory, original_address, BitField::load(address_bits), index + 1, mask, value);
        },
        '1' => {
            let mut mutable_address = built_address;            
            let address_bits = mutable_address.view_bits_mut::<Lsb0>();
            address_bits.get_mut(index).unwrap().set(true);
            let new_address = BitField::load(address_bits);                        
            update_recursive(memory, original_address, new_address, index + 1, mask, value);

        },
        _ => panic!("recursive failed, weird stuff in mask, yo")
    }

}

fn read_instruction_blocks() -> Vec<InstructionBlock> {
    let mut blocks: Vec<InstructionBlock> = vec![];

    let mut curr_mask: Option<String> = None;
    let mut instructions: Vec<Instruction> = vec![];
    let file = File::open(PATH).unwrap();
    for read_line in io::BufReader::new(file).lines() {
        let line = read_line.unwrap();
        if let Some(caps) = MASK_REGEX.captures(&line) {
            if !instructions.is_empty() {
                let block = InstructionBlock {
                    instructions: instructions.clone(),
                    mask: curr_mask.unwrap().clone(),
                };
                blocks.push(block);
                instructions.clear();
            }

            curr_mask = Some(caps.get(1).unwrap().as_str().to_string());
        } else if let Some(caps) = INSTR_REGEX.captures(&line) {
            let instr = Instruction {
                address: caps.get(1).unwrap().as_str().parse().unwrap(),
                value: caps.get(2).unwrap().as_str().parse().unwrap(),
            };
            instructions.push(instr);
        }
    }
    // Read one last block from the buffer, because it has no final delimiter
    if !instructions.is_empty() {
        let block = InstructionBlock {
            instructions: instructions.clone(),
            mask: curr_mask.unwrap(),
        };
        blocks.push(block);
        instructions.clear();
    }

    blocks
}

#[derive(Clone)]
struct InstructionBlock {
    mask: String,
    instructions: Vec<Instruction>,
}

#[derive(Copy, Clone)]
struct Instruction {
    address: u64,
    value: u64,
}
