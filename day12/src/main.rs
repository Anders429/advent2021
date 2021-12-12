use std::collections::*;
use std::str::FromStr;
use util::read_input;

struct Cave {
    label: String,
    paths: Vec<String>,
}

impl Cave {
    fn new(label: String) -> Self {
        Self {
            label,
            paths: Vec::new(),
        }
    }

    fn add_path(&mut self, path: String) {
        self.paths.push(path);
    }

    fn is_small(&self) -> bool {
        self.label.chars().all(char::is_lowercase)
    }

    fn is_start(&self) -> bool {
        self.label == "start"
    }

    fn is_end(&self) -> bool {
        self.label == "end"
    }
}

struct Edge {
    cave_a: String,
    cave_b: String,
}

impl FromStr for Edge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cave_a, cave_b) = s.split_once('-').ok_or(())?;

        Ok(Self {
            cave_a: cave_a.to_owned(),
            cave_b: cave_b.to_owned(),
        })
    }
}

fn create_caves(input: &[Edge]) -> HashMap<String, Cave> {
    let mut caves = HashMap::new();

    for edge in input {
        caves.entry(edge.cave_a.clone()).or_insert(Cave::new(edge.cave_a.clone())).add_path(edge.cave_b.clone());
        caves.entry(edge.cave_b.clone()).or_insert(Cave::new(edge.cave_b.clone())).add_path(edge.cave_a.clone());
    }

    caves
}

fn solve_1(input: &[Edge]) -> usize {
    let caves = create_caves(input);

    // DFS
    let mut stack = vec![vec!["start"]];
    let mut finished = Vec::new();
    while let Some(paths) = stack.pop() {
        let current = paths.last().unwrap();
        for cave in &caves[*current].paths {
            if caves[cave].is_small() {
                if paths.contains(&cave.as_str()) {
                    continue;
                }
            }
            let mut new_paths = paths.clone();
            new_paths.push(&cave);
            if caves[cave].is_end() {
                finished.push(new_paths);
            }
            else {
                stack.push(new_paths);
            }
        }
    }

    finished.len()
}

fn solve_2(input: &[Edge]) -> usize {
    let caves = create_caves(input);

    // DFS
    let mut stack = vec![(vec!["start"], false)];
    let mut finished = Vec::new();
    while let Some((paths, doubled)) = stack.pop() {
        let current = paths.last().unwrap();
        for cave in &caves[*current].paths {
            let mut has_doubled = doubled;
            if caves[cave].is_start() {
                continue;
            }
            if caves[cave].is_small() {
                if paths.contains(&cave.as_str()) {
                    if !has_doubled {
                        has_doubled = true;
                    } else {
                        continue;
                    }
                }
            }
            let mut new_paths = paths.clone();
            new_paths.push(&cave);
            if caves[cave].is_end() {
                finished.push(new_paths);
            }
            else {
                stack.push((new_paths, has_doubled));
            }
        }
    }

    finished.len()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Edge>(&args[1]).collect::<Vec<Edge>>();

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
