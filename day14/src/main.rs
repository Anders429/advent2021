use std::collections::*;
use std::str::FromStr;
use util::read_input;

type Polymer = char;

fn parse_input(input: &[String]) -> (Vec<Polymer>, HashMap<(Polymer, Polymer), Polymer>) {
    // Parse polymer template.
    let mut template = Vec::new();
    for c in input[0].chars() {
        template.push(c);
    }

    // Parse rules.
    let mut rules = HashMap::new();
    for line in &input[2..] {
        let (key, value) = line.split_once(" -> ").unwrap();
        let mut key_chars = key.chars();
        rules.insert((key_chars.next().unwrap(), key_chars.next().unwrap()), value.chars().next().unwrap());
    }

    (template, rules)
}

fn solve_1(input: &[String]) -> usize {
    let (mut template, rules) = parse_input(input);

    for _ in 0..10 {
        let mut new_template = Vec::new();
        let mut prev_polymer = None;
        for polymer in template {
            if let Some(prev) = prev_polymer {
                new_template.push(prev);
                new_template.push(rules[&(prev, polymer)]);
            }
            prev_polymer = Some(polymer);
        }
        new_template.push(prev_polymer.unwrap());
        template = new_template;
    }

    let counts = template.iter().fold(HashMap::new(), |mut a, p| {*a.entry(p).or_insert(0) += 1; a});
    dbg!(counts);
    0
}

fn combine_maps(mut left: HashMap<Polymer, usize>, right: HashMap<Polymer, usize>) -> HashMap<Polymer, usize> {
    for (key, val) in right.into_iter() {
        *left.entry(key).or_insert(0) += val;
    }
    left
}

fn dp(left: Polymer, right: Polymer, step: usize, rules: &HashMap<(Polymer, Polymer), Polymer>, cache: &mut HashMap<(Polymer, Polymer, usize), HashMap<Polymer, usize>>) -> HashMap<Polymer, usize> {
    if let Some(cached) = cache.get(&(left, right, step)) {
        cached.clone()
    } else {
        let result = if step == 0 {
            let mut result = HashMap::new();
            result.insert(left, 1);
            *result.entry(right).or_insert(0) += 1;
            result
        } else {
            let mut result = combine_maps(dp(left, rules[&(left, right)], step - 1, rules, cache), dp(rules[&(left, right)], right, step - 1, rules, cache));
            *result.entry(rules[&(left, right)]).or_insert(1) -= 1;
            result
        };
        cache.insert((left, right, step), result.clone());
        result
    }
}

fn solve_2(input: &[String]) -> usize {
    let (template, rules) = parse_input(input);

    let mut cache = HashMap::new();

    let mut prev_polymer = None;
    let mut result = HashMap::new();
    for polymer in template.clone() {
        if let Some(prev) = prev_polymer {
            result = combine_maps(result, dp(prev, polymer, 40, &rules, &mut cache));
            *result.entry(prev).or_insert(1) -= 1;
        }
        prev_polymer = Some(polymer);
    }
    *result.entry(template[0]).or_insert(0) += 1;

    dbg!(result);
    todo!()
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
