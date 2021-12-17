use std::collections::*;
use std::str::FromStr;
use util::read_input;

fn read_bits(mut n: usize, bytes: &mut impl Iterator<Item = u8>, buffer: &mut usize, buffer_len: &mut usize) -> usize {
    while *buffer_len < n {
        *buffer <<= 4;
        *buffer |= bytes.next().unwrap() as usize;
        *buffer_len += 4;
    }
    let result = *buffer >> (*buffer_len - n);
    *buffer_len -= n;
    *buffer &= 2_usize.pow(*buffer_len as u32) - 1;
    result
}

fn read_operator(bytes: &mut (impl Iterator<Item = u8> + ExactSizeIterator), buffer: &mut usize, buffer_len: &mut usize) -> Vec<Packet> {
    // Get most significant bit.
    let msb = read_bits(1, bytes, buffer, buffer_len);

    // Interpret bits based on msb.
    let mut packets = Vec::new();
    if msb == 0 {
        let mut length = read_bits(15, bytes, buffer, buffer_len);
        while length > 0 {
            let prev_byte_len = bytes.len();
            let prev_buffer_len = *buffer_len;
            packets.push(Packet::read_from(bytes, buffer, buffer_len));
            length = if *buffer_len < prev_buffer_len {
                length.saturating_sub(prev_buffer_len - *buffer_len)
            } else {
                length + (*buffer_len - prev_buffer_len)  
            };
            length = length.saturating_sub((prev_byte_len - bytes.len()) * 4);
        }
    } else {
        let length = read_bits(11, bytes, buffer, buffer_len);
        for _ in 0..length {
            packets.push(Packet::read_from(bytes, buffer, buffer_len));
        }
    }

    packets
}

#[derive(Debug)]
enum Type {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(usize),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    r#type: Type,
}

impl Packet {
    fn read_from(bytes: &mut (impl Iterator<Item = u8> + ExactSizeIterator), buffer: &mut usize, buffer_len: &mut usize) -> Self {
        // Read the header.
        let version = read_bits(3, bytes, buffer, buffer_len);
        let type_id = read_bits(3, bytes, buffer, buffer_len);

        // Match on type id.
        let r#type = match type_id {
            0 => {
                Type::Sum(read_operator(bytes, buffer, buffer_len))
            }
            1 => {
                Type::Product(read_operator(bytes, buffer, buffer_len))
            }
            2 => {
                Type::Minimum(read_operator(bytes, buffer, buffer_len))
            }
            3 => {
                Type::Maximum(read_operator(bytes, buffer, buffer_len))
            }
            4 => {
                // Read 5 bits at a time.
                let mut value = 0;
                loop {
                    let msb = read_bits(1, bytes, buffer, buffer_len);
                    value <<= 4;
                    value |= read_bits(4, bytes, buffer, buffer_len);
                    // Check the most significant bit.
                    if msb == 0 {
                        break;
                    }
                }
                Type::Literal(value)
            }
            5 => {
                Type::GreaterThan(read_operator(bytes, buffer, buffer_len))
            }
            6 => {
                Type::LessThan(read_operator(bytes, buffer, buffer_len))
            }
            7 => {
                Type::EqualTo(read_operator(bytes, buffer, buffer_len))
            }
            _ => {
                unimplemented!()
            }
        };

        Self {
            version: version as u8,
            r#type,
        }
    }

    fn version_sum(&self) -> usize {
        match &self.r#type {
            Type::Literal(_) => self.version as usize,
            Type::Sum(packets) | Type::Product(packets) | Type::Minimum(packets) | Type::Maximum(packets) | Type::GreaterThan(packets) | Type::LessThan(packets) | Type::EqualTo(packets) => {
                packets.iter().map(|p| p.version_sum()).sum::<usize>() + (self.version as usize)
            }
        }
    }

    fn evaluate(&self) -> usize {
        match &self.r#type {
            Type::Literal(val) => *val,
            Type::Sum(packets) => packets.iter().map(|p| p.evaluate()).sum(),
            Type::Product(packets) => packets.iter().map(|p| p.evaluate()).product(),
            Type::Minimum(packets) => packets.iter().map(|p| p.evaluate()).min().unwrap(),
            Type::Maximum(packets) => packets.iter().map(|p| p.evaluate()).max().unwrap(),
            Type::GreaterThan(packets) => {
                // Assume 2 packets.
                if packets[0].evaluate() > packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Type::LessThan(packets) => {
                //dbg!(packets);
                // Assume 2 packets.
                if packets[0].evaluate() < packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
            Type::EqualTo(packets) => {
                // Assume 2 packets.
                if packets[0].evaluate() == packets[1].evaluate() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse_packet(line: &str) -> Packet {
    Packet::read_from(&mut hex::decode(line).unwrap().into_iter().fold(Vec::new(), |mut a, byte| {a.push(byte >> 4); a.push(byte & 15); a}).into_iter(), &mut 0, &mut 0)
}

fn solve_1(input: &[String]) -> usize {
    let packet = parse_packet(&input[0]);
    packet.version_sum()
}

fn solve_2(input: &[String]) -> usize {
    let packet = parse_packet(&input[0]);
    dbg!(&packet);
    packet.evaluate()
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
