use std::fs::File;
use std::io::{self, BufRead};
use std::str;
use std::collections::HashMap;

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_8/input.txt";

fn main() {
    // Part one
    let instructions = parse_input();
    let value_before_loop = attempt_execution(&instructions);
    println!("PART ONE: The accumulator value just before the cycle is: {}", value_before_loop.unwrap_err());

    // Part two
    let mut self_modifying_code = instructions.clone();
    let program_len = self_modifying_code.len();

    // First, try all possible nop -> jmp replacements
    println!("Trying nop -> jmp replacements...");
    for i in 0..program_len {        
        let mut mutable_inst = self_modifying_code.get_mut(i).unwrap();
        let unmodified_instruction = mutable_inst.clone();
        if let Operation::Nop(val) = mutable_inst.operation {
            mutable_inst.operation = Operation::Jump(val);
        }

        match attempt_execution(&self_modifying_code) {
            Ok(acc) => println!("PART TWO: Found working instruction set! Accumulator value: {}", acc),
            Err(_) => {
                // If it didn't work, restore the old instruction
                self_modifying_code.get_mut(i).unwrap().operation = unmodified_instruction.operation;
            }
        }
    }    

    //If none of those work, let's try all possible jmp -> nop replacements
    println!("Trying jmp -> nop replacements...");
    for i in 0..program_len {
        let mut mutable_inst = self_modifying_code.get_mut(i).unwrap();
        let unmodified_instruction = mutable_inst.clone();
        if let Operation::Jump(val) = mutable_inst.operation {
            mutable_inst.operation = Operation::Nop(val);
        }

        match attempt_execution(&self_modifying_code) {
            Ok(acc) => println!("PART TWO: Found working instruction set! Accumulator value: {}", acc),
            Err(_) => {
                // If it didn't work, restore the old instruction
                self_modifying_code.get_mut(i).unwrap().operation = unmodified_instruction.operation;
            }
        }
    }
}

fn parse_input() -> Vec<Instruction> {
    let file = File::open(PATH).unwrap();
    io::BufReader::new(file).lines().enumerate().map(|(i, x)| {
        let line = x.unwrap();
        let bytes = line.as_bytes();
        let operation = str::from_utf8(&bytes[0..3]).unwrap();
        let operator: char = bytes[4] as char;
        let mut operand = str::from_utf8(&bytes[5..]).unwrap().parse().unwrap();
        if operator == '-' {
            operand = operand * - 1;
        }
        match operation {
            "nop" => Instruction { line: i as u32, operation: Operation::Nop(operand)},
            "acc" => Instruction { line: i as u32, operation: Operation::Accumulate(operand) },
            "jmp" => Instruction { line: i as u32, operation: Operation::Jump(operand) },
            _ => panic!("Unsupported value")
        }
    }).collect()
}

fn attempt_execution(instructions: &Vec<Instruction>) -> Result<i64, i64> {
    let mut seen_lines: HashMap<u32, bool> = HashMap::new();
    let mut state = State::new();
    
    loop {
        // Check to see if we've executed the last line
        if state.program_counter > instructions.len() as i32 - 1 {
            return Ok(state.accumulator);
        }

        // Fetch
        let instruction = instructions[state.program_counter as usize];        

        // (check for cycle)
        let new_instruction_line = instruction.line;
        if seen_lines.contains_key(&new_instruction_line) {
            return Err(state.accumulator);
        }

        // Add to list of seen lines
        seen_lines.insert(instruction.line, true);

        // Decode & Execute
        state.execute(&instruction);
    }         
}

struct State {
    program_counter: i32,
    accumulator: i64,
}

impl State {
    fn new() -> Self {
        State { program_counter: 0, accumulator: 0 }
    }

    fn execute(&mut self, instruction: &Instruction) {
        //println!("Line {}, running {:?}", instruction.line, instruction.operation);
        match instruction.operation {
            Operation::Nop(_) => {
                self.program_counter += 1;
            },
            Operation::Accumulate(acc) => {
                self.program_counter += 1;
                self.accumulator += acc as i64
            },
            Operation::Jump(jmp) => {
                self.program_counter += jmp
            },
        }
    }
}

#[derive(Copy, Clone)]
struct Instruction {
    line: u32,
    operation: Operation,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Accumulate(i32),
    Jump(i32),
    Nop(i32)
}