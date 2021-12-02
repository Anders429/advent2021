use util::read_input;

fn solve(input: &[usize]) -> usize {
    let mut prev_input = None;
    let mut count = 0;

    for i in input {
        match prev_input {
            Some(p) => {
                if i > p {
                    count += 1;
                }
            }
            None => {

            }
        }
        prev_input = Some(i)
    }

    count
}

fn solve_2(input: &[usize]) -> usize {
    enum Sum {
        None,
        Single(usize),
        Double(usize, usize),
        Triple(usize, usize, usize),
    }

    let mut sum = Sum::None;
    let mut count = 0;

    for i in input {
        match sum {
            Sum::None => {
                sum = Sum::Single(*i);
            }
            Sum::Single(p) => {
                sum = Sum::Double(p, *i);
            }
            Sum::Double(p1, p2) => {
                sum = Sum::Triple(p1, p2, *i);
            }
            Sum::Triple(p1, p2, p3) => {
                if p2 + p3 + i > p1 + p2 + p3 {
                    count += 1;
                }
                sum = Sum::Triple(p2, p3, *i);
            }
        }
    }

    count
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<usize>(&args[1]).collect::<Vec<usize>>();

    // First star.
    println!("{}", solve(&input));
    println!("{}", solve_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{solve_2};

    #[test]
    fn example_2() {
        let input = [
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];

        assert_eq!(solve_2(&input), 7);
    }
}
