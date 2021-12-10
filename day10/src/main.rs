use std::collections::*;
use std::str::FromStr;
use util::read_input;

enum Chunk {
    Parenthesis,
    Bracket,
    Curly,
    Carrot,
}

fn solve_1(input: &[String]) -> usize {

    let mut result = 0;

    for s in input {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(Chunk::Parenthesis);
                }
                '[' => {
                    stack.push(Chunk::Bracket);
                }
                '{' => {

                    stack.push(Chunk::Curly);
                }
                '<' => {

                    stack.push(Chunk::Carrot);
                }
                ')' => {
                    if let Some(chunk) = stack.pop() {
                        if !matches!(chunk, Chunk::Parenthesis) {
                            result += 3;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                ']' => {
                    if let Some(chunk) = stack.pop() {
                        if !matches!(chunk, Chunk::Bracket) {
                            result += 57;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                '}' => {
                    if let Some(chunk) = stack.pop() {
                        if !matches!(chunk, Chunk::Curly) {
                            result += 1197;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                '>' => {
                    if let Some(chunk) = stack.pop() {
                        if !matches!(chunk, Chunk::Carrot) {
                            result += 25137;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ => {panic!("oh no");}
            }
        }
    }

    result
}

fn solve_2(input: &[String]) -> usize {
    let mut results = Vec::new();

    for s in input {
        let mut stack = Vec::new();
        let mut corrupted = false;
        for c in s.chars() {
            match c {
                '(' => {
                    stack.push(Chunk::Parenthesis);
                }
                '[' => {
                    stack.push(Chunk::Bracket);
                }
                '{' => {

                    stack.push(Chunk::Curly);
                }
                '<' => {

                    stack.push(Chunk::Carrot);
                }
                ')' => {
                    if let Some(chunk) = stack.pop() {
                        if !matches!(chunk, Chunk::Parenthesis) {
                            corrupted = true;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                ']' => {
                    if let Some(chunk) = stack.pop() {
                        if !matches!(chunk, Chunk::Bracket) {
                            corrupted = true;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                '}' => {
                    if let Some(chunk) = stack.pop() {
                        if !matches!(chunk, Chunk::Curly) {
                            corrupted = true;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                '>' => {
                    if let Some(chunk) = stack.pop() {
                        if !matches!(chunk, Chunk::Carrot) {
                            corrupted = true;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ => {panic!("oh no");}
            }
        }
        if !corrupted {
        let result = stack.iter().rev().fold(0, |mut a, c| {
            a *= 5;
            match c {
            Chunk::Parenthesis => {
                a += 1;
            }
            Chunk::Bracket => {
                a += 2;
            }
            Chunk::Curly => {
                a += 3;
            }
            Chunk::Carrot => {
                a += 4;
            }
            }
            a
        });
        results.push(result);
        }
    }

    results.sort();
    dbg!(&results);
    results[results.len() / 2]
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
