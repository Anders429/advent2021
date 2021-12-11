use std::collections::*;
use std::str::FromStr;
use util::read_input;

struct Octopus {
    energy: usize,
    flashing: bool,
    flashed: bool,
}

impl Octopus {
    fn new(energy: usize) -> Self {
        Self {
            energy,
            flashing: false,
            flashed: false,
        }
    }

    fn increment(&mut self) {
        self.energy += 1;
        if self.energy > 9 {
            self.flashing = true;
        }
    }

    fn flash(&mut self) {
        if self.flashing {
            self.flashed = true;
        }
    }

    fn reset(&mut self) {
        if self.flashing {
            self.energy = 0;
            self.flashing = false;
            self.flashed = false;
        }
    }
}

fn solve_1(input: &[String]) -> usize {
    let mut octopuses = Vec::new();
    for line in input {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Octopus::new(c.to_string().parse::<usize>().unwrap()));
        }
        octopuses.push(row);
    }

    let octopuses_len = octopuses.len();
    let row_len = octopuses[0].len();

    let mut count = 0;
    for _ in 0..100 {
        for row in octopuses.iter_mut() {
            for octopus in row.iter_mut() {
                octopus.increment();
            }
        }
        let mut state_changed = true;
        let mut flashing = Vec::new();
        while state_changed {
            state_changed = false;
            for (i, row) in octopuses.iter_mut().enumerate() {
                for (j, octopus) in row.iter_mut().enumerate() {
                    if octopus.flashing && !octopus.flashed {
                        count += 1;
                        state_changed = true;
                        octopus.flash();
                        if i != 0 {
                            if j != 0 {
                                flashing.push((i - 1, j - 1));
                            }
                            flashing.push((i - 1, j));
                            if j != row_len - 1 {
                                flashing.push((i - 1, j + 1));
                            }
                        }
                        if j != 0 {
                            flashing.push((i, j - 1));
                        }
                        if j != row_len - 1 {
                            flashing.push((i, j + 1));
                        }
                        if i != octopuses_len - 1 {
                            if j != 0 {
                                flashing.push((i + 1, j - 1));
                            }
                            flashing.push((i + 1, j));
                            if j != row_len - 1 {
                                flashing.push((i + 1, j + 1));
                            }
                        }
                    }
                }
            }
            for (i, j) in flashing.drain(..) {
                octopuses[i][j].increment();
            }
        }
        for row in octopuses.iter_mut() {
            for octopus in row.iter_mut() {
                octopus.reset();
            }
        }
    }

    count
}

fn solve_2(input: &[String]) -> usize {
    let mut octopuses = Vec::new();
    for line in input {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Octopus::new(c.to_string().parse::<usize>().unwrap()));
        }
        octopuses.push(row);
    }

    let octopuses_len = octopuses.len();
    let row_len = octopuses[0].len();

    let mut iteration = 0;
    loop {
        for row in octopuses.iter_mut() {
            for octopus in row.iter_mut() {
                octopus.increment();
            }
        }
        let mut state_changed = true;
        let mut flashing = Vec::new();
        while state_changed {
            state_changed = false;
            for (i, row) in octopuses.iter_mut().enumerate() {
                for (j, octopus) in row.iter_mut().enumerate() {
                    if octopus.flashing && !octopus.flashed {
                        state_changed = true;
                        octopus.flash();
                        if i != 0 {
                            if j != 0 {
                                flashing.push((i - 1, j - 1));
                            }
                            flashing.push((i - 1, j));
                            if j != row_len - 1 {
                                flashing.push((i - 1, j + 1));
                            }
                        }
                        if j != 0 {
                            flashing.push((i, j - 1));
                        }
                        if j != row_len - 1 {
                            flashing.push((i, j + 1));
                        }
                        if i != octopuses_len - 1 {
                            if j != 0 {
                                flashing.push((i + 1, j - 1));
                            }
                            flashing.push((i + 1, j));
                            if j != row_len - 1 {
                                flashing.push((i + 1, j + 1));
                            }
                        }
                    }
                }
            }
            for (i, j) in flashing.drain(..) {
                octopuses[i][j].increment();
            }
        }
        let mut syncronized = true;
        for row in octopuses.iter_mut() {
            for octopus in row.iter_mut() {
                if !octopus.flashed {
                    syncronized = false;
                }
                octopus.reset();
            }
        }
        if syncronized {
            return iteration;
        }
        iteration += 1;
    }
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
