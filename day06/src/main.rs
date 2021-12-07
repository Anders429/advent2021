use std::collections::HashMap;
use std::str::FromStr;
use util::read_input;

struct Fish {
    timer: usize,
}

impl Fish {
    fn new(timer: usize) -> Self {
        Self {
            timer,
        }
    }

    // How many descendants will this fish have over days
    fn descendants(&self, mut days: usize) -> usize {
        if days > self.timer {
            Fish::new(6).descendants(days - ( self.timer + 1)) + Fish::new(8).descendants(days - ( self.timer + 1))
        } else {
            1
        }

        // let mut timer = self.timer;
        // // dbg!("FISH");
        // // dbg!(self.timer);
        // let mut result = 0;
        // while days >= timer + 1 {
        //     // dbg!(days);
        //     // dbg!(result);
        //     // Count the next fish.
        //     result += 1;
        //     // Count all of that fish's descendants.
        //     result += Fish::new(8).descendants(days - (timer + 1));
        //     // Move to the next week.
        //     days = days.saturating_sub((timer + 1));
        //     timer = 6;
        // }
        // // dbg!("END FISH");
        // result

    }
}

fn descendants(day: usize, age: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if cache.contains_key(&(day, age)) {
        cache[&(day, age)]
    } else {
        if day <= age {
            1
        } else {
            let value = descendants(day - age - 1, 6, cache) + descendants(day - age - 1, 8, cache);
            cache.insert((day, age), value);
            value
        }
    }
}

fn solve_1(input: &[String]) -> usize {
    let fishes = input[0].split(',').map(|t| Fish::new(t.parse::<usize>().unwrap())).collect::<Vec<_>>();

    fishes.iter().fold(0, |mut acc, fish| {acc += fish.descendants(80); acc})
}

fn solve_2(input: &[String]) -> usize {
    let mut cache = HashMap::new();

    input[0].split(',').map(|t| descendants(256, t.parse::<usize>().unwrap(), &mut cache)).fold(0, |mut acc, count| {acc += count; acc})
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
