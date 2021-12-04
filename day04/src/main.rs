
use util::read_input;
use std::str::FromStr;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

#[derive(Clone, Copy, Debug)]
enum Number {
    Unmarked(usize),
    Marked(usize),
}

impl Default for Number {
    fn default() -> Self {
        Self::Unmarked(usize::default())
    }
}

struct Board {
    numbers: [[Number; WIDTH]; HEIGHT],
}

impl Board {
    fn new(lines: &[String]) -> Self {
        let mut numbers = [[Number::default(); WIDTH]; HEIGHT];

        for (j, line) in lines.iter().enumerate() {
            for (i, val) in line.split_whitespace().map(|s| s.parse::<usize>().unwrap()).enumerate() {
                numbers[j][i] = Number::Unmarked(val);
            }
        }

        Self {
            numbers,
        }
    }

    fn mark(&mut self, pull: usize) {
        for row in self.numbers.iter_mut() {
            for num in row.iter_mut() {
                if let Number::Unmarked(val) = num {
                    if *val == pull {
                    *num = Number::Marked(pull);
                    }
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        // Check rows
        for row in self.numbers {
            let mut winner = true;
            for num in row {
                match num {
                    Number::Unmarked(_) => {
                        winner = false;
                        break;
                    }
                    _ => {}
                }
            }
            if winner {
                return true;
            }
        }
        // Chec columns
        for i in 0..5 {
            let mut winner = true;
            for row in self.numbers {
                match row[i] {
                    Number::Unmarked(_) => {
                        winner = false;
                        break;
                    }
                    _ => {}
                }
            }
            if winner {
                return true;
            }
        }
        // Check diagonals
        let mut winner = true;
        for i in 0..5 {
            for j in 0..5 {
                match self.numbers[i][j] {
                    Number::Unmarked(_) => {
                        winner = false;
                        break;
                    }
                    _ => {}
                }
            }
        }
        if winner {
            return true;
        }

        let mut winner = true;
        for i in 0..5 {
            for j in 0..5 {
                match self.numbers[4-i][j] {
                    Number::Unmarked(_) => {
                        winner = false;
                        break;
                    }
                    _ => {}
                }
            }
        }
        if winner {
            return true;
        }

        false
    }

    fn score(&self) -> usize {
        dbg!(self.numbers);
        let mut result = 0;
        for row in self.numbers {
            for num in row {
                if let Number::Unmarked(val) = num {
                    result += val;
                }
            }   
        }
        result
    }
}

fn solve_1(input: &[String]) -> usize {
    let mut lines = input.to_vec().into_iter();

    let pulls_raw = lines.next().unwrap();
    let pulls = pulls_raw.split(',').map(|s| s.parse::<usize>().unwrap());
    lines.next();  // SKip hte empty line.

    let mut boards = Vec::new();
    loop {
        if lines.len() == 0 {
            break;
        }
        let mut board_lines = Vec::new();
        for _ in 0..5 {
            board_lines.push(lines.next().unwrap());
        }
        boards.push(Board::new(&board_lines));
        lines.next();  // Skip the empty line.
    }

    for pull in pulls {
        for board in boards.iter_mut() {
            board.mark(pull);
            if board.is_winner() {
                return pull * board.score()
            }
        }
    }

    unreachable!()
}

fn solve_2(input: &[String]) -> usize {
    let mut lines = input.to_vec().into_iter();

    let pulls_raw = lines.next().unwrap();
    let pulls = pulls_raw.split(',').map(|s| s.parse::<usize>().unwrap());
    lines.next();  // SKip hte empty line.

    let mut boards = Vec::new();
    loop {
        if lines.len() == 0 {
            break;
        }
        let mut board_lines = Vec::new();
        for _ in 0..5 {
            board_lines.push(lines.next().unwrap());
        }
        boards.push(Board::new(&board_lines));
        lines.next();  // Skip the empty line.
    }


    for pull in pulls {
        let mut new_boards = Vec::new();
        let boards_len = boards.len();
        for mut board in boards.into_iter() {
            board.mark(pull);
            if !board.is_winner() {
                new_boards.push(board);
            } else if boards_len == 1 {
                return pull * board.score()
            }
            
        }
        boards = new_boards;
    }

    unreachable!()
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
    fn example_1() {

    }

    #[test]
    fn example_2() {

    }
}
