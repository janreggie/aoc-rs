use anyhow::{anyhow, bail, Context, Result};
use std::cmp::Ordering;

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
        result *= 2; // first attempt is a no-op
        if b {
            result += 1;
        }
    }

    result
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
                // Extra padding
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
    fn new(bits: &[bool]) -> Result<Packet> {
        let iter = bits.iter();
        let (packet, rest) = Packet::it(iter);
        if rest.len() != 0 {
            bail!("could not create a Packet with input of len 0")
        }
        packet
    }

    fn it(mut bits: std::slice::Iter<bool>) -> (Result<Packet>, std::slice::Iter<bool>) {
        if bits.len() == 0 {
            return (
                Err(anyhow!("could not create a Packet with input of len 0")),
                bits,
            );
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
            let length_type_id = *bits.next().unwrap_or(&false);
            let mut subpackets = Vec::new();

            if length_type_id {
                // Get the next 11 bits
                let mut subpacket_count = vec![];
                for _ in 0..11 {
                    subpacket_count.push(*bits.next().unwrap_or(&false));
                }
                let subpacket_count = bits_to_literal(&subpacket_count) as usize;

                while subpackets.len() < subpacket_count {
                    let (subpacket, bb) = Packet::it(bits);
                    match subpacket {
                        Ok(s) => subpackets.push(s),
                        Err(e) => {
                            return (
                                Err(e).context(format!(
                                    "could not insert subpacket {} of {}",
                                    subpackets.len() + 1,
                                    subpacket_count
                                )),
                                bb,
                            )
                        }
                    }
                    bits = bb;
                }
            } else {
                // Get the next 15 bits
                let mut to_consume = vec![];
                for _ in 0..15 {
                    to_consume.push(*bits.next().unwrap_or(&false));
                }
                let to_consume = bits_to_literal(&to_consume) as usize;

                let initial_length = bits.len();
                while bits.len() != 0 {
                    let bb = bits;
                    let (subpacket, bb) = Packet::it(bb);
                    match subpacket {
                        Ok(s) => subpackets.push(s),
                        Err(e) => {
                            return (
                                Err(e).context(format!(
                                    "could not insert subpacket {}",
                                    subpackets.len() + 1,
                                )),
                                bb,
                            )
                        }
                    }
                    let consumed = initial_length - bb.len();
                    if consumed == to_consume {
                        bits = bb;
                        break;
                    } else if consumed > to_consume {
                        return (
                            Err(anyhow!(
                                "overconsumed: expected to consume {} bits, got {}",
                                to_consume,
                                consumed
                            )),
                            bb,
                        );
                    }
                    bits = bb;
                }
            }
            data = PacketData::Subpackets(subpackets);
        }

        let packet = Packet {
            version,
            type_id,
            data,
        };
        (Ok(packet), bits)
    }

    fn get_version_sum(&self) -> u128 {
        self.version
            + match &self.data {
                PacketData::Literal(_) => 0,
                PacketData::Subpackets(vv) => vv.iter().map(|p| p.get_version_sum()).sum(),
            }
    }

    fn value(&self) -> Result<u128> {
        match &self.data {
            PacketData::Literal(v) => Ok(*v),
            PacketData::Subpackets(vv) => {
                let vv: Result<Vec<u128>> = vv.iter().map(|p| p.value()).collect();
                let vv = vv.context("could not evaluate subpackets")?;

                let expect = |cmp: Ordering| {
                    if vv.len() != 2 {
                        bail!("expect input to be of length 2")
                    }

                    let (v1, v2) = (vv[0], vv[1]);
                    Ok(if v1.cmp(&v2) == cmp { 1 } else { 0 })
                };

                let vv = vv.iter();
                match self.type_id {
                    0 => Ok(vv.sum()),
                    1 => Ok(vv.product()),
                    2 => Ok(*vv.min().context("zero subpackets")?),
                    3 => Ok(*vv.max().context("zero subpackets")?),
                    4 => Err(anyhow!("PacketData should be Literal for type id 4")),
                    5 => expect(Ordering::Greater),
                    6 => expect(Ordering::Less),
                    7 => expect(Ordering::Equal),
                    _ => Err(anyhow!("unknown type id {}", self.type_id)),
                }
            }
            .context(format!(
                "could not evaluate value of Packet ID: {}, Version: {}",
                self.type_id, self.version
            )),
        }
    }
}

enum PacketData {
    Literal(u128),
    Subpackets(Vec<Packet>),
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("expected only 1 line, got {} instead", lines.len())
    }
    let mut lines = lines;
    let input = lines.pop().unwrap();
    let bits = input_to_bits(&input).context("could not parse input")?;
    let packet = Packet::new(&bits).context("could not create packet from input")?;

    let ans1 = packet.get_version_sum();
    let ans2 = packet.value().context("could not get value")?;

    Ok((ans1.to_string(), ans2.to_string()))
}
