use std::io::{self, BufRead};
use std::{fs::File, iter::Peekable, slice::Iter};

const PATH: &str = "C:/Users/mcali/Desktop/Repositories/advent_of_code_2020/day_18/input.txt";
const PT2_ADD_PRECEDENCE: u32 = 2;
const PT2_MULT_PRECEDENCE: u32 = 1;

fn main() {
    let sum_of_all_equations = sum_all_problems(false);
    println!("PART ONE: {}", sum_of_all_equations);

    let sum_of_backwardsey_nonsense = sum_all_problems(true);
    println!("PART TWO: {}", sum_of_backwardsey_nonsense);
}

fn sum_all_problems(part_two: bool) -> u64 {
    let file = File::open(PATH).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|x| {
            let line = x.unwrap().replace(" ", "");
            let tokenized: Vec<Token> = line
                .chars()
                .map(|c| match c {
                    '(' => Token::OpenParen,
                    ')' => Token::CloseParen,
                    '+' => Token::Add,
                    '*' => Token::Multiply,
                    val => Token::Number(val.to_string().parse().unwrap()),
                })
                .collect();
            if !part_two {
                recursive_calculate(&tokenized, 0).0
            } else {
                compute_expression(&mut tokenized.iter().peekable(), 1)
            }
        })
        .sum()
}

// part one implementation
fn recursive_calculate(equation: &[Token], index: usize) -> (u64, usize) {
    let mut index = index;
    let mut running_value = 0u64;
    let mut curr_operator = Token::Add;

    loop {
        if index >= equation.len() {
            break;
        }
        let token = equation.get(index).unwrap();
        match token {
            Token::Add => curr_operator = Token::Add,
            Token::Multiply => curr_operator = Token::Multiply,
            Token::Number(val) => {
                if curr_operator == Token::Add {
                    running_value += val;
                } else if curr_operator == Token::Multiply {
                    running_value *= val;
                }
            }
            Token::CloseParen => {
                return (running_value, index);
            }
            Token::OpenParen => {
                let (inner_val, new_index) = recursive_calculate(equation, index + 1);
                if curr_operator == Token::Add {
                    running_value += inner_val;
                } else if curr_operator == Token::Multiply {
                    running_value *= inner_val;
                }
                index = new_index;
            }
        }

        index += 1;
    }

    (running_value, 0)
}

// part two implementation
fn compute_expression(equation: &mut Peekable<Iter<Token>>, min_precedence: u32) -> u64 {
    let mut atom_lhs = compute_atom(equation);

    loop {
        let curr = equation.peek();
        // Stop looping if:
        //  a) this isn't a '+' or a '*', 
        //  b) the precedence of the operator is lower than the current min, or 
        //  c) we've read everything
        if let Some(val) = curr {
            if !matches!(*val, Token::Add | Token::Multiply)
                || get_pt2_precedence(**val) < min_precedence
            {
                break;
            }
        } else {
            break;
        }

        let operator = **curr.unwrap();
        let precedence = get_pt2_precedence(operator);
        let next_min_precedence = precedence + 1;

        equation.next(); // consume the token in preparation for the next call
        let atom_rhs = compute_expression(equation, next_min_precedence);
        atom_lhs = compute_operation(operator, atom_lhs, atom_rhs);
    }

    atom_lhs
}

fn compute_atom(equation: &mut Peekable<Iter<Token>>) -> u64 {
    let token = equation.peek().unwrap();

    match **token {
        Token::OpenParen => {
            equation.next(); // consume and swallow opening paren
            let val = compute_expression(equation, 1);
            equation.next(); // consume and swallow closing paren
            val
        }
        Token::Number(val) => {
            equation.next(); // actually consume the value we just peeked at
            val
        }
        _ => panic!("Oh no, invalid value in compute_atom()"),
    }
}

fn compute_operation(token: Token, lhs: u64, rhs: u64) -> u64 {
    match token {
        Token::Add => lhs + rhs,
        Token::Multiply => lhs * rhs,
        _ => panic!("Cannot compute_operation() using a non-operator token >:"),
    }
}

fn get_pt2_precedence(token: Token) -> u32 {
    match token {
        Token::Add => PT2_ADD_PRECEDENCE,
        Token::Multiply => PT2_MULT_PRECEDENCE,
        _ => panic!("Nothing else has a precedence, you dunce"),
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Token {
    OpenParen,
    CloseParen,
    Add,
    Multiply,
    Number(u64),
}