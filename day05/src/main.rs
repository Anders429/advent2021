use std::collections::HashMap;
use std::str::FromStr;
use util::read_input;

#[derive(Debug, Eq, PartialEq)]
struct Line {
    x1: usize,
    y1: usize,

    x2: usize,
    y2: usize,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn points(&self) -> Vec<(usize, usize)> {
        if self.is_horizontal() {
            (std::cmp::min(self.x1, self.x2)..=std::cmp::max(self.x1, self.x2)).map(|x| (x, self.y1)).collect::<Vec<_>>()
        } else if self.is_vertical() {
            (std::cmp::min(self.y1, self.y2)..=std::cmp::max(self.y1, self.y2)).map(|y| (self.x1, y)).collect::<Vec<_>>()
        } else if self.x1 < self.x2 && self.y1 < self.y2 {
            (self.x1..=self.x2).zip(self.y1..=self.y2).collect::<Vec<_>>()
        } else if self.x1 < self.x2 && self.y1 > self.y2 {
            (self.x1..=self.x2).zip((self.y2..=self.y1).rev()).collect::<Vec<_>>()
        }else if self.x1 > self.x2 && self.y1 < self.y2 {
            (self.x2..=self.x1).rev().zip(self.y1..=self.y2).collect::<Vec<_>>()
        }else {
            (self.x2..=self.x1).rev().zip((self.y2..=self.y1).rev()).collect::<Vec<_>>()
        }
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x1, s) = s.split_once(',').ok_or(())?;
        let (y1, s) = s.split_once(" -> ").ok_or(())?;

        let (x2, y2) = s.split_once(',').ok_or(())?;

        let x1 = x1.parse::<usize>().map_err(|_| ())?;
        let y1 = y1.parse::<usize>().map_err(|_| ())?;
        let x2 = x2.parse::<usize>().map_err(|_| ())?;
        let y2 = y2.parse::<usize>().map_err(|_| ())?;

        Ok(Self { x1, y1, x2, y2 })
    }
}

fn solve_1(input: &[Line]) -> usize {
    let mut points = HashMap::new();

    for line in input {
        if !(line.is_horizontal() || line.is_vertical()) {
            continue;
        }
        for point in line.points() {
            *points.entry(point).or_insert(0) += 1;
        }
    }

    points.iter().filter(|(_, count)| **count > 1).count()
}

fn solve_2(input: &[Line]) -> usize {
    let mut points = HashMap::new();

    for line in input {
        for point in line.points() {
            *points.entry(point).or_insert(0) += 1;
        }
    }

    points.iter().filter(|(_, count)| **count > 1).count()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Line>(&args[1]).collect::<Vec<Line>>();

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
