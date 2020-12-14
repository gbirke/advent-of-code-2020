use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Eq, PartialEq, Debug, Clone)]
struct Bitmask {
    and_mask: u64,
    or_mask: u64,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Memory {
    location: u64,
    number: u64,
    bitmask: Bitmask,
}

impl Memory {
    fn apply_bitmask(&self) -> u64 {
        (self.number & self.bitmask.and_mask) | self.bitmask.or_mask
    }
}

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> Vec<Memory> {
    let mut result: Vec<Memory> = Vec::new();
    let mut bm = "";
    for l in input.lines() {
        let mut parts = l.split(" = ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();
        if &first[0..4] == "mask" {
            bm = second;
            continue;
        } else if &first[0..3] == "mem" {
            result.push(Memory {
                location: first[4..first.len() - 1].parse::<u64>().unwrap(),
                number: second.parse::<u64>().unwrap(),
                bitmask: parse_bitmask(bm),
            })
        }
    }
    result
}

fn parse_bitmask(pattern: &str) -> Bitmask {
    let and_mask = u64::from_str_radix(&pattern.replace("X", "1"), 2).unwrap();
    let or_mask = u64::from_str_radix(&pattern.replace("X", "0"), 2).unwrap();
    Bitmask { and_mask, or_mask }
}

#[aoc(day14, part1)]
fn part_one(input: &Vec<Memory>) -> u64 {
    let unique_memory: HashMap<u64, Memory> =
        input.iter().map(|m| (m.location, m.clone())).collect();
    unique_memory.values().map(|m| m.apply_bitmask()).sum()
}

fn get_floating_masks(input: &str) -> Vec<usize> {
    let floating_bits: Vec<usize> = input.chars().rev().enumerate().filter_map(|(i,c) | match c {
        'X' => Some(i), _ => None
    }).collect();
    
    let powers:Vec<u64> = floating_bits.iter().map(|n| {
        2u64.pow(u32::try_from(*n).unwrap())
    }).collect();

    // TODO combine each power with [1, 0] using https://docs.rs/itertools/0.7.8/itertools/trait.Itertools.html#method.combinations
    // TODO Create numbers from all the combinations, setting the bits

    // TODO replace with real vec
    Vec::new()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_applies_bitmask() {
        let bm = Bitmask {
            and_mask: u64::MAX & 0b111111111111111111111111111111111101,
            or_mask: 0b1000000,
        };
        let memory1 = Memory {
            location: 0,
            number: 11,
            bitmask: bm.clone(),
        };
        let memory2 = Memory {
            location: 0,
            number: 101,
            bitmask: bm.clone(),
        };
        let memory3 = Memory {
            location: 0,
            number: 0,
            bitmask: bm.clone(),
        };

        assert_eq!(memory1.apply_bitmask(), 73);
        assert_eq!(memory2.apply_bitmask(), 101);
        assert_eq!(memory3.apply_bitmask(), 64);
    }

    #[test]
    fn it_parses_bitmask() {
        let bm = parse_bitmask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

        assert_eq!(
            bm,
            Bitmask {
                and_mask: u64::MAX & 0b111111111111111111111111111111111101,
                or_mask: 0b1000000
            }
        )
    }

    #[test]
    fn it_parses_input() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11";
        let result = parse_input(input);

        assert_eq!(
            result[0],
            Memory {
                location: 8,
                number: 11,
                bitmask: Bitmask {
                    and_mask: u64::MAX & 0b111111111111111111111111111111111101,
                    or_mask: 0b1000000
                }
            }
        )
    }

    #[test]
    fn it_solves_part_one() {
        let raw_input =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";
        let input = parse_input(raw_input);
        println!("Input was {:?}", input);

        assert_eq!(part_one(&input), 165)
    }
}
