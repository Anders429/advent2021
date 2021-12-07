#![feature(int_abs_diff)]

use std::str::FromStr;
use util::read_input;

fn solve_1(input: &[String]) -> usize {
    let mut crabs = input[0].split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();

    crabs.sort();
    let median = crabs[crabs.len() / 2];
    crabs.into_iter().fold(0, |mut a, c| {a += c.abs_diff(median); a})
}

fn solve_2(input: &[String]) -> usize {
    let crabs = input[0].split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let low_mean = crabs.iter().sum::<usize>() / crabs.len();
    let high_mean = (crabs.iter().sum::<usize>() + (crabs.len() - 1)) / crabs.len();
    
    std::cmp::min(crabs.iter().fold(0, |mut a, c| {
        let mut diff = c.abs_diff(low_mean);
        while diff != 0 {
            a += diff;
            diff -= 1;
        }
        a
    }), crabs.iter().fold(0, |mut a, c| {
        let mut diff = c.abs_diff(high_mean);
        while diff != 0 {
            a += diff;
            diff -= 1;
        }
        a
    }))
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
