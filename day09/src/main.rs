use std::collections::*;
use std::str::FromStr;
use util::read_input;

struct Row {
    points: Vec<usize>,
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = Vec::new();
        for c in s.chars() {
            points.push(c.to_string().parse::<usize>().map_err(|_ |())?);
        }
        Ok(Self {
            points,
        })
    }
}

fn solve_1(input: &[Row]) -> usize {
    let mut lowest = Vec::new();
    // Find lowest points.
    for (i, row) in input.iter().enumerate() {
        for (j, point) in row.points.iter().enumerate() {
            // Has to be lower than all up down left and right
            let point = *point;
            if j != 0 && point >= row.points[j - 1] {
                continue;
            }
            if j != row.points.len() - 1 && point >= row.points[j + 1] {
                continue;
            }
            if i != 0 && point >= input[i - 1].points[j] {
                continue;
            }
            if i != input.len() - 1 && point >= input[i + 1].points[j] {
                continue;
            }
            lowest.push(point);
        }
    }
    lowest.into_iter().map(|i| i + 1).sum()
}

fn solve_2(input: &[Row]) -> usize {
    let mut lowest = Vec::new();
    // Find lowest points.
    for (i, row) in input.iter().enumerate() {
        for (j, point) in row.points.iter().enumerate() {
            // Has to be lower than all up down left and right
            let point = *point;
            if j != 0 && point >= row.points[j - 1] {
                continue;
            }
            if j != row.points.len() - 1 && point >= row.points[j + 1] {
                continue;
            }
            if i != 0 && point >= input[i - 1].points[j] {
                continue;
            }
            if i != input.len() - 1 && point >= input[i + 1].points[j] {
                continue;
            }
            lowest.push((point, i, j));
        }
    }

    // Basins
    let mut basins = Vec::new();
    // Find the basins by traveling starting at the lowest points.
    for low in lowest.into_iter() {
        let mut stack = vec![low];
        let mut visited = HashSet::new();
        let mut count = 0;
        while !stack.is_empty() {
            let (point, i, j) = stack.pop().unwrap();
            if visited.contains(&(i, j)) {
                continue;
            }
            visited.insert((i, j));
            count += 1;
            if j != 0 && point < input[i].points[j - 1] && input[i].points[j - 1] != 9 {
                stack.push((point, i, j - 1));
            }
            if j != input[i].points.len() - 1 && point < input[i].points[j + 1] && input[i].points[j + 1] != 9{
                stack.push((point, i, j + 1));
            }
            if i != 0 && point < input[i - 1].points[j] && input[i - 1].points[j] != 9 {
                stack.push((point, i - 1, j));
            }
            if i != input.len() - 1 && point < input[i + 1].points[j]  && input[i + 1].points[j] != 9 {
                stack.push((point, i + 1, j));
            }
        }
        basins.push(count);
    }

    basins.sort();
    basins.iter().rev().take(3).fold(1, |mut a, b| { a *= b; a})
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Row>(&args[1]).collect::<Vec<Row>>();

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
