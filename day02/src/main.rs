use util::read_input;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, i) = s.split_once(' ').unwrap();

        match c {
            "forward" => {
                Ok(Instruction::Forward(i.parse::<usize>().unwrap()))
            }
            "down" => {
                Ok(Instruction::Down(i.parse::<usize>().unwrap()))
            }
            "up" => {
                Ok(Instruction::Up(i.parse::<usize>().unwrap()))
            }
            _ => {
                Err("unknown".to_string())
            }
        }
    }
}

#[derive(Default)]
struct Submarine {
    horizontal_position: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {
    fn instruct(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(i) => {
                self.horizontal_position += i;
                self.depth += self.aim * i;
            }
            Instruction::Down(i) => {
                self.aim += i;
            }
            Instruction::Up(i) => {
                self.aim -= i;
            }
        }
    }
}

fn solve_1(input: &[Instruction]) -> usize {
    let mut sub = Submarine::default();

    for i in input {
        sub.instruct(*i);
    }

    sub.horizontal_position * sub.depth
}

fn solve_2(input: &[usize]) -> usize {
    todo!()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Instruction>(&args[1]).collect::<Vec<Instruction>>();

    // First star.
    println!("{}", solve_1(&input));
    // println!("{}", solve_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{solve_1, solve_2};

    #[test]
    fn example_1() {

    }

    #[test]
    fn example_2() {

    }
}
