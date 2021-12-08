use std::collections::*;
use std::str::FromStr;
use util::read_input;

// 0 => abc-efg
// 1 => --c--f-
// 2 => a-cde-g
// 3 => a-cd-fg
// 4 => -bcd-f-
// 5 => ab-d-fg
// 6 => ab-defg
// 7 => a-c--f-
// 8 => abcdefg
// 9 => abcd-fg

// 6 and 9 are length 6.
// 5, 3, and 2 are length 5.
// 0, 6 and 9 will tell us which segments are C, D and E.
// B is in 5, but not 3 or 2.

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl FromStr for Segment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "e" => Ok(Self::E),
            "f" => Ok(Self::F),
            "g" => Ok(Self::G),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default)]
struct Signal {
    segments: Vec<Segment>,
}

impl Signal {
    fn len(&self) -> usize {
        self.segments.len()
    }

    fn contains(&self, v: &Segment) -> bool {
        self.segments.contains(v)
    }

    fn interpret(&self, mapping: &HashMap<Segment, Segment>) -> usize {
        //dbg!(mapping);
        let mut mapped_segments = HashSet::new();
        for segment in &self.segments {
            mapped_segments.insert(mapping[&segment]);
        }
        if mapped_segments.contains(&Segment::A) {
            if mapped_segments.contains(&Segment::B) {
                if mapped_segments.contains(&Segment::C) {
                    if mapped_segments.contains(&Segment::D) {
                        if mapped_segments.contains(&Segment::E) {
                            8
                        } else {
                            9
                        }
                    } else {
                        0
                    }
                } else {
                    if mapped_segments.contains(&Segment::E) {
                        6
                    } else {
                        5
                    }
                }
            } else {
                if mapped_segments.contains(&Segment::D) {
                    if mapped_segments.contains(&Segment::E) {
                        2
                    } else {
                        3
                    }
                } else {
                    7
                }
            }
        } else {
            if mapped_segments.contains(&Segment::B) {
                4
            } else {
                1
            }
        }
    }
}

impl FromStr for Signal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = Vec::with_capacity(7);
        for c in s.chars() {
            segments.push(Segment::from_str(&c.to_string())?);
        }
        Ok(Self { segments })
    }
}

struct Signals {
    patterns: Vec<Signal>,
    outputs: Vec<Signal>,
}

impl Signals {
    fn interpret_output(&self, mapping: &HashMap<Segment, Segment>) -> usize {
        let mut result = 0;
        for signal in &self.outputs {
            result *= 10;
            result += signal.interpret(mapping);
        }
        result
    }

    fn deduce_and_interpret(&self) -> usize {
        let mut mapping = HashMap::new();
        mapping.insert(
            Segment::A,
            vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
        );
        mapping.insert(
            Segment::B,
            vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
        );
        mapping.insert(
            Segment::C,
            vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
        );
        mapping.insert(
            Segment::D,
            vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
        );
        mapping.insert(
            Segment::E,
            vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
        );
        mapping.insert(
            Segment::F,
            vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
        );
        mapping.insert(
            Segment::G,
            vec![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
        );

        // Filter out 1s, 4s, 7s, and 8s.
        let mut one_segments = &Signal::default();
        let mut four_segments = &Signal::default();
        let mut seven_segments = &Signal::default();
        let mut eight_segments = &Signal::default();
        for segments in &self.patterns {
            if segments.len() == 2 {
                one_segments = segments;
            } else if segments.len() == 3 {
                seven_segments = segments;
            } else if segments.len() == 4 {
                four_segments = segments;
            } else if segments.len() == 7 {
                eight_segments = segments;
            }
        }
        // 1 and 7
        // We know A is the only segment not in 1 but in 7.
        for segment in &seven_segments.segments {
            if one_segments.contains(&segment) {
                *mapping.get_mut(&segment).unwrap() = vec![Segment::C, Segment::F];
            } else {
                *mapping.get_mut(&segment).unwrap() = vec![Segment::A];
            }
        }
        // 4 and 7
        // We know the other segments are B and D.
        for segment in &four_segments.segments {
            if !seven_segments.contains(&segment) {
                *mapping.get_mut(&segment).unwrap() = vec![Segment::B, Segment::D];
            }
        }
        // 7, 4 and 8
        // We know the other segments can't be A, B, C, D, or F.
        for segment in &eight_segments.segments {
            if !four_segments.contains(&segment) && !seven_segments.contains(&segment) {
                *mapping.get_mut(&segment).unwrap() = vec![Segment::E, Segment::G];
            }
        }
        // Now we have each segment narrowed down to 2, with A figured out completely.
        // 2,3,5
        // These can tell us which is B and which is D.
        let mut c = Vec::new();
        for (k, v) in &mapping {
            if v.contains(&Segment::B) {
                c.push(k.clone());
            }
        }
        let mut d_segment = Segment::A;
        'outer: for segments in &self.patterns {
            if segments.len() != 5 {
                continue;
            }
            for segment in &c {
                if !segments.contains(&segment) {
                    // This is segment B.
                    *mapping.get_mut(&segment).unwrap() = vec![Segment::B];
                    for (key, val) in mapping.iter_mut() {
                        if key == segment {
                            continue;
                        }
                        if val.contains(&Segment::D) {
                            *val = vec![Segment::D];
                            d_segment = *key;
                            break;
                        }
                    }
                    break 'outer;
                }
            }
        }
        // 6 and 9.
        //dbg!(&mapping);
        for segments in &self.patterns {
            if segments.len() != 6 {
                continue;
            }
            // We know which one is 0, so we skip it.
            //dbg!(d_segment);
            if !segments.contains(&d_segment) {
                continue;
            }
            for segment in &[
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ] {
                if !segments.contains(&segment) {
                    //dbg!(&segments);
                    //dbg!(&segment);
                    // Either C or E.
                    if mapping[segment].contains(&Segment::C) {
                        *mapping.get_mut(&segment).unwrap() = vec![Segment::C];
                        for (key, val) in mapping.iter_mut() {
                            if key == segment {
                                continue;
                            }
                            if val.contains(&Segment::F) {
                                *val = vec![Segment::F];
                                break;
                            }
                        }
                        break;
                    } else if mapping[segment].contains(&Segment::E) {
                        *mapping.get_mut(&segment).unwrap() = vec![Segment::E];
                        for (key, val) in mapping.iter_mut() {
                            if key == segment {
                                continue;
                            }
                            if val.contains(&Segment::G) {
                                *val = vec![Segment::G];
                                break;
                            }
                        }
                        break;
                    }
                }
            }
        }

        let mut final_mapping = HashMap::new();
        for (key, v) in &mapping {
            // Should only be one left.
            final_mapping.insert(*key, v[0]);
        }
        let output = self.interpret_output(&final_mapping);
        //dbg!(output);
        output
    }
}

impl FromStr for Signals {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_patterns, raw_outputs) = s.split_once(" | ").ok_or(())?;

        let mut patterns = Vec::with_capacity(10);
        for (i, pattern) in raw_patterns.split_whitespace().enumerate() {
            patterns.push(Signal::from_str(pattern)?);
        }

        let mut outputs = Vec::with_capacity(4);
        for (i, output) in raw_outputs.split_whitespace().enumerate() {
            outputs.push(Signal::from_str(output)?);
        }

        Ok(Self { outputs, patterns })
    }
}

fn solve_1(input: &[Signals]) -> usize {
    input
        .iter()
        .map(|s| &s.outputs)
        .flatten()
        .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count()
}

fn solve_2(input: &[Signals]) -> usize {
    input.iter().map(|s| s.deduce_and_interpret()).sum()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Signals>(&args[1]).collect::<Vec<Signals>>();

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
