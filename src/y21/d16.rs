use anyhow::{bail, Context, Result};
// use std::collections::VecDeque;

/// Returns the bit sequence of the input into its binary form.
/// For example:
/// `literal_to_bits(11) == [True,False,True,True]`
fn literal_to_bits(input: u128) -> Vec<bool> {
    let mut result = Vec::new();
    let mut remaining = input;
    while remaining > 0 {
        result.push(remaining % 2 != 0);
        remaining /= 2;
    }

    result.reverse();
    result
}

#[cfg(test)]
#[test]
fn test_literal_to_bits() {
    let (t, f) = (true, false);
    let testcases: Vec<(u128, Vec<bool>)> =
        vec![(3, vec![t, t]), (4, vec![t, f, f]), (2, vec![t, f])];
    for (lit, bits) in testcases {
        assert_eq!(bits, literal_to_bits(lit));
    }
}

/// Converts bit sequence into decimal.
/// For example:
/// `bool_arr_to_u32([True,True,False,True]) == 11`.
fn bits_to_literal(input: &[bool]) -> u128 {
    let mut result = 0;
    for &b in input {
        if b {
            result += 1;
        }
        result *= 2;
    }

    result /= 2; // We overmultiplied
    result
}

fn print_bits(input: &[bool]) {
    for &b in input {
        if b {
            eprint!("#")
        } else {
            eprint!(".")
        }
    }
    eprintln!("")
}

fn input_to_bits(input: &str) -> Result<Vec<bool>> {
    fn lookup(c: char) -> Option<Vec<bool>> {
        Some(literal_to_bits(c.to_digit(16)? as u128))
    }

    let mut result = Vec::new();
    for c in input.chars() {
        match lookup(c) {
            None => bail!("could not interpret character `{}`", c),
            Some(v) => {
                for _ in 0..(4 - v.len()) {
                    result.push(false);
                }
                for b in v {
                    result.push(b);
                }
            }
        }
    }

    // Remove extra 0's at the end
    while !*result.last().unwrap_or(&true) {
        result.pop();
    }

    Ok(result)
}

struct Packet {
    version: u128,
    type_id: u128,
    data: PacketData,
}

impl Packet {
    fn new(bits: &[bool]) -> Option<Packet> {
        let iter = bits.iter();
        let (packet, rest) = Packet::it(iter);
        if rest.len() != 0 {
            None
        } else {
            packet
        }
    }

    fn it(mut bits: std::slice::Iter<bool>) -> (Option<Packet>, std::slice::Iter<bool>) {
        if bits.len() == 0 {
            return (None, bits);
        }

        let mut version = Vec::new();
        for _ in 0..3 {
            version.push(*bits.next().unwrap_or(&false));
        }
        let version = bits_to_literal(&version);
        let mut type_id = Vec::new();
        for _ in 0..3 {
            type_id.push(*bits.next().unwrap_or(&false));
        }
        let type_id = bits_to_literal(&type_id);
        eprintln!("version: {}, type_id: {}", version, type_id);

        let data: PacketData;
        if type_id == 4 {
            let mut literal = Vec::new();
            loop {
                let prefix = *bits.next().unwrap_or(&false);
                for _ in 0..4 {
                    literal.push(*bits.next().unwrap_or(&false));
                }
                if !prefix {
                    break;
                }
            }
            let literal = bits_to_literal(&literal);
            data = PacketData::Literal(literal);
        } else {
            // TODO: Fix this part. This is wrong!!
            let length_type_id = *bits.next().unwrap_or(&false);
            let bb = bits;
            let (p1, bb) = Packet::it(bb);
            let (p2, bb) = Packet::it(bb);
            if p1.is_none() || p2.is_none() {
                return (None, bb);
            }

            bits = bb;
            data = PacketData::Subpacket(Box::new(p1.unwrap()), Box::new(p2.unwrap()));
        }

        let packet = Packet {
            version,
            type_id,
            data,
        };
        (Some(packet), bits)
    }

    fn get_version_sum(&self) -> u128 {
        self.version
            + match &self.data {
                PacketData::Literal(_) => 0,
                PacketData::Subpacket(p1, p2) => p1.get_version_sum() + p2.get_version_sum(),
            }
    }
}

enum PacketData {
    Literal(u128),
    Subpacket(Box<Packet>, Box<Packet>),
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("expected only 1 line, got {} instead", lines.len())
    }
    let mut lines = lines;
    let input = lines.pop().unwrap();
    let bits = input_to_bits(&input).context("could not parse input")?;
    print_bits(&bits);

    // Part 1: I don't know what I'm doing
    let packet = Packet::new(&bits).context("could not create packet from input")?;
    let ans1 = packet.get_version_sum();

    Ok((ans1.to_string(), String::from("unimplemented")))
}
