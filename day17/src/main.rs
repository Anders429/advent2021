use std::collections::*;
use std::ops::RangeInclusive;
use std::str::FromStr;
use util::read_input;

#[derive(Debug)]
struct Target {
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
}

impl FromStr for Target {
    type Err = ();

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        s = s.strip_prefix("target area: x=").unwrap();
        let (x_str, y_str) = s.split_once(", y=").unwrap();
        let (x_1, x_2) = x_str.split_once("..").unwrap();
        let (y_1, y_2) = y_str.split_once("..").unwrap();
        Ok(Self {
            x_range: (x_1.parse::<isize>().unwrap())..=(x_2.parse::<isize>().unwrap()),
            y_range: (y_1.parse::<isize>().unwrap())..=(y_2.parse::<isize>().unwrap()),
        })
    }
}

fn solve_1(input: &[Target]) -> isize {
    (0..=((input[0].y_range.start() + 1) * -1)).sum()
}

fn solve_2(input: &[Target]) -> usize {
    let ys = ((*input[0].y_range.start())..=0)
        .filter_map(|v| {
            let mut y = 0;
            let mut v2 = v;
            loop {
                if y < *input[0].y_range.start() {
                    return None;
                }
                if y <= *input[0].y_range.end() {
                    return Some(v);
                }
                y += v2;
                v2 -= 1;
            }
        })
        .fold(Vec::new(), |mut a, v| {
            if v == 0 {
                a.push(v);
            } else if v == -1 {
                a.push(v);
            } else {
                a.push(v * -1 - 1);
                a.push(v);
            }
            a
        });

    let mut result = 0;
    for x in 0..=*input[0].x_range.end() {
        for y in &ys {
            let mut x_vel = x;
            let mut y_vel = *y;
            let mut x_loc = 0;
            let mut y_loc = 0;
            loop {
                if x_loc > *input[0].x_range.end() {
                    break;
                }
                if y_loc < *input[0].y_range.start() {
                    break;
                }
                if x_loc >= *input[0].x_range.start() && y_loc <= *input[0].y_range.end() {
                    result += 1;
                    break;
                }
                x_loc += x_vel;
                y_loc += y_vel;
                if x_vel > 0 {
                    x_vel -= 1;
                }
                y_vel -= 1;
            }
        }
    }

    result
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Target>(&args[1]).collect::<Vec<_>>();

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
