use std::collections::*;
use std::str::FromStr;
use util::read_input;

#[derive(Eq, PartialEq, Hash, Clone)]
enum Distance {
    Finite(usize),
    Infinite,
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Node {
    value: usize,
    visited: bool,
    distance: Distance,
}

impl Node {
    fn new(value: usize) -> Self {
        Self {
            value,
            visited: false,
            distance: Distance::Infinite,
        }
    }

    fn increment(&mut self) {
        if self.value == 9 {
            self.value = 1;
        } else {
            self.value += 1;
        }
    }

    fn attempt(&mut self, distance: usize) {
        if let Distance::Finite(self_distance) = self.distance {
            if self_distance <= self.value + distance {
                return;
            }
        }
        self.distance = Distance::Finite(self.value + distance);
    }

    fn priority(&self) -> usize {
        if let Distance::Finite(distance) = self.distance {
            usize::MAX - distance
        } else {
            0
        }
    }
}

fn parse_input(input: &[String]) -> Vec<Vec<Node>> {
    let mut result = Vec::new();
    for line in input {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Node::new(c.to_string().parse::<usize>().unwrap()));
        }
        result.push(row);
    }
    result
}

fn dijsktra(mut grid: Vec<Vec<Node>>) -> usize {
    grid[0][0].distance = Distance::Finite(0);

    let mut queue = priority_queue::PriorityQueue::new();
    let mut current = (0, 0);
    loop {
        if current.0 == grid.len() - 1 && current.1 == grid[0].len() - 1 {
            if let Distance::Finite(result) = grid[current.0][current.1].distance {
                return result;
            } else {
                panic!("oh no");
            }
        }

        if let Distance::Finite(raw_distance) = grid[current.0][current.1].distance {
        if current.0 != 0 {
            grid[current.0 - 1][current.1].attempt(raw_distance);
            if !grid[current.0 - 1][current.1].visited {
                queue.push_increase((current.0 - 1, current.1), grid[current.0 - 1][current.1].priority());
            }
        }
        if current.1 != 0 {
            grid[current.0][current.1 - 1].attempt(raw_distance);
            if !grid[current.0][current.1 - 1].visited {
                queue.push_increase((current.0, current.1 - 1), grid[current.0][current.1 - 1].priority());
            }
        }
        if current.0 != grid.len() - 1 {
            grid[current.0 + 1][current.1].attempt(raw_distance);
            if !grid[current.0 + 1][current.1].visited {
                queue.push_increase((current.0 + 1, current.1), grid[current.0 + 1][current.1].priority());
            }
        }
        if current.1 != grid[0].len() - 1 {
            grid[current.0][current.1 + 1].attempt(raw_distance);
            if !grid[current.0][current.1 + 1].visited {
                queue.push_increase((current.0, current.1 + 1), grid[current.0][current.1 + 1].priority());
            }
        }
        } else {
            panic!("grr");
        }
        grid[current.0][current.1].visited = true;

        // Pick next.
        while let Some(((a, b), _)) = queue.pop() {
            if !grid[a][b].visited {
                current = (a, b);
                break;
            } else {
                dbg!("UHOH");
            }
        }
    }
}

fn solve_1(input: &[String]) -> usize {
    let grid = parse_input(input);
    dijsktra(grid)
}

fn increment_grid(grid: &Vec<Vec<Node>>) -> Vec<Vec<Node>> {
    let mut new_grid = Vec::new();
    for line in grid {
        new_grid.push(line.iter().cloned().map(|mut n| {n.increment(); n}).collect::<Vec<_>>());
    }
    new_grid
}

fn multiply_grid(mut grid: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
    let len = grid.len();
    let row_len = grid[0].len();

    let mut grids = Vec::new();
    for _ in 0..9 {
        let mut new_grid = increment_grid(&grid);
        grids.push(grid);
        grid = new_grid;
    }

    let mut result = Vec::new();
    for a in 0..5 {
        for i in 0..len {
            let mut row = Vec::new();
            for b in 0..5 {
                row.extend(grids[a + b][i].clone());
            }
            result.push(row);
        }
    }
    result
}

fn solve_2(input: &[String]) -> usize {
    let grid = multiply_grid(parse_input(input));
    dijsktra(grid)
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
