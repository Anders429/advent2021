
use util::read_input;
use std::str::FromStr;

#[derive(Clone)]
struct Line {
    bits: Vec<bool>,
}

impl Line {
    fn as_usize(&self) -> usize {
        let mut result = 0;
        for b in &self.bits {
            if *b {
                result |= 1;
            }
            result <<= 1;
        }
        result >> 1
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits = Vec::new();
        for c in s.chars() {
            match c {
                '0' => {
                    bits.push(false)
                }
                '1' => {
                    bits.push(true)
                }
                _ => {
                    return Err(())
                }
            }
        }
        Ok(
            Line {
                bits,
            }
        )
    }
}

#[derive(Debug)]
struct Lines {
    false_counts: Vec<usize>,
    true_counts: Vec<usize>,
}

impl Lines {
    fn new(lines: &[Line]) -> Self {
        let mut false_counts = vec![0; lines[0].bits.len()];
        let mut true_counts = vec![0; lines[0].bits.len()];

        for line in lines {
            for (i, b) in line.bits.iter().enumerate() {
                if *b {
                    true_counts[i] += 1;
                } else {
                    false_counts[i] += 1;
                }
            }
        }

        Self {
            false_counts,
            true_counts,
        }
    }

    fn gamma_rate(&self) -> usize {
        let mut gamma = 0;
        for (f, t) in self.false_counts.iter().zip(self.true_counts.iter()) {
            if t > f {
                gamma |= 1;
            }
            gamma <<= 1;
        }
        gamma >> 1
    }

    fn epsilon_rate(&self) -> usize {
        let mut gamma = 0;
        for (f, t) in self.false_counts.iter().zip(self.true_counts.iter()) {
            if t < f {
                gamma |= 1;
            }
            gamma <<= 1;
        }
        gamma >> 1
    }

    fn ogr(&self, lines: &[Line], i: usize) -> usize {
        let mut new_lines = Vec::new();

        if lines.len() == 1 {
            return lines[0].as_usize();
        }

        for line in lines.to_vec().drain(..) {
            if self.false_counts[i] > self.true_counts[i] {
                if !line.bits[i] {
                    new_lines.push(line);
                }
            } else {
                if line.bits[i] {
                    new_lines.push(line);
                }
            }
        }

        Lines::new(&new_lines).ogr(&new_lines, i + 1)
    }

    fn co2(&self, lines: &[Line], i: usize) -> usize {
        let mut new_lines = Vec::new();

        if lines.len() == 1 {
            return lines[0].as_usize();
        }

        for line in lines.to_vec().drain(..) {
            if self.true_counts[i] < self.false_counts[i] {
                if line.bits[i] {
                    new_lines.push(line);
                }
            } else {
                if !line.bits[i] {
                    new_lines.push(line);
                }
            }
        }

        Lines::new(&new_lines).co2(&new_lines, i + 1)
    }
}

fn solve_1(input: &[Line]) -> usize {
    let lines = Lines::new(input);

    lines.gamma_rate() * lines.epsilon_rate()
}

fn solve_2(input: &[Line]) -> usize {
    let lines = Lines::new(input);

    dbg!(lines.ogr(input, 0));
    dbg!(lines.co2(input, 0));

    lines.ogr(input, 0) * lines.co2(input, 0)
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
    fn example_1() {

    }

    #[test]
    fn example_2() {

    }
}
