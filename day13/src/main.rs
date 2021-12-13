use std::collections::*;
use std::str::FromStr;
use util::read_input;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Dot {
    x: usize,
    y: usize,
}

impl Dot {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl FromStr for Dot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(())?;
        Ok(Self {
            x: x.parse::<usize>().map_err(|_| ())?,
            y: y.parse::<usize>().map_err(|_| ())?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    X(usize),
    Y(usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.strip_prefix("fold along ").ok_or(())?;
        let (instruction, value) = s.split_once('=').ok_or(())?;
        let value = value.parse::<usize>().map_err(|_| ())?;
        match instruction {
            "x" => Ok(Self::X(value)),
            "y" => Ok(Self::Y(value)),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &[String]) -> (HashSet<Dot>, Vec<Instruction>) {
    let mut iter = input.into_iter();

    let mut dots = HashSet::new();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        dots.insert(Dot::from_str(line).unwrap());
    }

    let mut instructions = Vec::new();
    while let Some(line) = iter.next() {
        instructions.push(Instruction::from_str(line).unwrap());
    }  
    
    (dots, instructions)
}

fn process_instruction(dots: &HashSet<Dot>, instruction: Instruction) -> HashSet<Dot> {
    let mut result = HashSet::with_capacity(dots.len());

    for dot in dots {
        let mut new_dot = dot.clone();
        match instruction {
            Instruction::X(value) => {
                if new_dot.x > value {
                    new_dot.x = value - (new_dot.x - value);
                }
            }
            Instruction::Y(value) => {
                if new_dot.y > value {
                    new_dot.y = value - (new_dot.y - value);
                }
            }
        }
        result.insert(new_dot);
    }
    result
}

fn solve_1(input: &[String]) -> usize {
    let (dots, instructions) = parse_input(input);

    process_instruction(&dots, instructions[0]).len()
}

fn solve_2(input: &[String]) -> usize {
    let (mut dots, instructions) = parse_input(input);

    for instruction in instructions {
        dots = process_instruction(&dots, instruction);
    }

    dbg!(dots.len());
    dbg!(dots.iter().map(|d| d.x).max());
    dbg!(dots.iter().map(|d| d.y).max());
    for j in 0..6 {
        'line: for i in 0..40 {
            for dot in &dots {
                if dot.x == i && dot.y == j {
                    print!("#");
                    continue 'line;
                }
            }
            print!(".");
        }
        println!("");
    }
    0
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    // First star.
    println!("{}", solve_1(&input));
    println!("{}", solve_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{solve_1, solve_2};

    #[test]
    fn example_1() {}

    #[test]
    fn example_2() {}
}
