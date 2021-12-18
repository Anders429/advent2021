use std::str::FromStr;
use util::read_input;

#[derive(Clone)]
enum Element {
    Number(usize),
    Pair(Pair),
}

impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next().unwrap() != '[' {
            Ok(Element::Number(s.parse::<usize>().unwrap()))
        } else {
            Ok(Element::Pair(Pair::from_str(s)?))
        }
    }
}

impl Element {
    fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    fn unwrap_number(&self) -> usize {
        match self {
            Self::Number(number) => *number,
            Self::Pair(_) => {
                panic!("not a number")
            }
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Self::Number(number) => {
                *number
            }
            Self::Pair(pair) => {
                3 * pair.x.magnitude() + 2 * pair.y.magnitude()
            }
        }
    }

    fn add_value_leftmost(&mut self, val: usize) {
        match self {
            Self::Number(number) => {
                *number += val;
            }
            Self::Pair(pair) => {
                pair.x.add_value_leftmost(val);
            }
        }
    }

    fn add_value_rightmost(&mut self, val: usize) {
        match self {
            Self::Number(number) => {
                *number += val;
            }
            Self::Pair(pair) => {
                pair.y.add_value_rightmost(val);
            }
        }
    }

    fn explode(&mut self, nesting_level: usize) -> Option<(Option<usize>, Option<usize>)> {
        match self {
            Self::Number(number) => None,
            Self::Pair(pair) => {
                if nesting_level == 3 && pair.x.is_number() && pair.y.is_number() {
                    let result = Some((Some(pair.x.unwrap_number()), Some(pair.y.unwrap_number())));
                    *self = Self::Number(0);
                    result
                } else {
                    if let Some((left, right)) = pair.x.explode(nesting_level + 1) {
                        if let Some(val) = right {
                            pair.y.add_value_leftmost(val);
                        }
                        Some((left, None))
                    } else {
                        if let Some((left, right)) = pair.y.explode(nesting_level + 1) {
                            if let Some(val) = left {
                                pair.x.add_value_rightmost(val);
                            }
                            Some((None, right))
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Number(number) => {
                if *number >= 10 {
                    *self = Self::Pair(Pair::new(
                        Element::Number(*number / 2),
                        Element::Number((*number + 1) / 2),
                    ));
                    true
                } else {
                    false
                }
            }
            Self::Pair(pair) => pair.x.split() || pair.y.split(),
        }
    }

    fn str_format(&self) -> String {
        match self {
            Self::Number(number) => {
                number.to_string()
            }
            Self::Pair(pair) => {
                pair.str_format()
            }
        }
    }
}

#[derive(Clone)]
struct Pair {
    x: Box<Element>,
    y: Box<Element>,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip prefix and suffix.
        let s = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();

        // Find split point.
        let mut char_indices = s.char_indices();
        let mut stack = 0;
        let mut split_point = None;
        for (i, c) in char_indices {
            if c == ',' && stack == 0 {
                split_point = Some(i);
                break;
            } else if c == '[' {
                stack += 1;
            } else if c == ']' {
                stack -= 1;
            }
        }
        let (x, y) = s.split_at(split_point.unwrap());
        let y = &y[1..];

        // Parse the elements.
        Ok(Self {
            x: Box::new(Element::from_str(x)?),
            y: Box::new(Element::from_str(y)?),
        })
    }
}

impl Pair {
    fn new(x: Element, y: Element) -> Self {
        Self {
            x: Box::new(x),
            y: Box::new(y),
        }
    }

    fn reduce(&mut self) {
        loop {
            if let Some((_, right)) = self.x.explode(0) {
                if let Some(val) = right {
                    self.y.add_value_leftmost(val);
                }
                continue;
            }
            if let Some((left, _)) = self.y.explode(0) {
                if let Some(val) = left {
                    self.x.add_value_rightmost(val);
                }
                continue;
            }
            if self.x.split() {
                continue;
            }
            if self.y.split() {
                continue;
            }
            break;
        }
    }

    fn add(self, other: Self) -> Self {
        let mut result = Self::new(Element::Pair(self), Element::Pair(other));
        result.reduce();
        result
    }

    fn str_format(&self) -> String {
        let mut result = String::new();
        result += "[";
        result += &self.x.str_format();
        result += ",";
        result += &self.y.str_format();
        result += "]";
        result
    }
}

fn solve_1(input: &[Pair]) -> usize {
    let mut combined: Option<Pair> = None;
    for pair in input.into_iter() {
        if let Some(prev) = combined {
            let added = prev.add(pair.clone());
            println!("{}", added.str_format());
            combined = Some(added);
        } else {
            combined = Some(pair.clone());
        }
    }
    Element::Pair(combined.unwrap()).magnitude()
}

fn solve_2(input: &[Pair]) -> usize {
    let mut result = 0;
    for (i, pair_a) in input.iter().enumerate() {
        for (j,pair_b) in input.iter().enumerate() {
            if i == j {
                continue;
            }
            result = std::cmp::max(Element::Pair(pair_a.clone().add(pair_b.clone())).magnitude(), result);
        }
    }
    result
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Pair>(&args[1]).collect::<Vec<_>>();

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
