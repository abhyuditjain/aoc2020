use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Mask {
    or_pattern: u64,
    and_pattern: u64,
}

impl Default for Mask {
    fn default() -> Self {
        Mask {
            or_pattern: 0,
            and_pattern: u64::MAX,
        }
    }
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        value & self.and_pattern | self.or_pattern
    }
}

#[derive(Debug, Copy, Clone)]
struct FloatMask {
    overwrite_pattern: u64,
    float_pattern: u64,
}

impl Default for FloatMask {
    fn default() -> Self {
        FloatMask {
            overwrite_pattern: 0,
            float_pattern: 0,
        }
    }
}

impl FloatMask {
    fn apply(&self, location: u64) -> Vec<u64> {
        let mut result = Vec::with_capacity(1 << self.float_pattern.count_ones());
        let loc = location & !self.float_pattern | self.overwrite_pattern;
        result.push(loc);

        for bi in 0..64 {
            if ((self.float_pattern >> bi) & 1) == 1 {
                let half = result.iter().map(|n| n | (1 << bi)).collect::<Vec<_>>();
                result = [&result[..], &half[..]].concat();
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_float_mask_test() {
        let mask = FloatMask {
            overwrite_pattern: 0,
            float_pattern: 0b11,
        };
        assert_eq!(mask.apply(0b000), [0b00, 0b01, 0b10, 0b11]);
    }

    #[test]
    fn float_mask_test() {
        let mask = FloatMask {
            overwrite_pattern: 0,
            float_pattern: 0b101,
        };
        assert_eq!(mask.apply(0b000), [0b000, 0b001, 0b100, 0b101]);
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    FloatMask(FloatMask),
    Memory(u64, u64),
}

#[aoc(day14, part1)]
fn solve_part1(input: &str) -> u64 {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let words = l.split(' ').collect::<Vec<_>>();
            if words[0] == "mask" {
                let mask_str = String::from(words[2]);
                let and_pattern = mask_str.replace('X', "1");
                let and_pattern = u64::from_str_radix(&and_pattern, 2).unwrap();
                let or_pattern = mask_str.replace('X', "0");
                let or_pattern = u64::from_str_radix(&or_pattern, 2).unwrap();
                let mask = Mask {
                    or_pattern,
                    and_pattern,
                };
                Instruction::Mask(mask)
            } else if words[0].starts_with("mem[") {
                let location = String::from(&words[0][4..(words[0].len() - 1)]);
                let location: u64 = location.parse().unwrap();
                let value: u64 = words[2].parse().unwrap();
                Instruction::Memory(location, value)
            } else {
                unreachable!()
            }
        })
        .collect();

    let mut mem = HashMap::new();
    let mut current_mask = Mask::default();
    for instr in instructions {
        match instr {
            Instruction::Mask(mask) => current_mask = mask,
            Instruction::Memory(location, value) => {
                mem.insert(location, current_mask.apply(value));
            }
            _ => unreachable!(),
        }
    }
    mem.values().sum()
}

#[aoc(day14, part2)]
fn solve_part2(input: &str) -> u64 {
    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let words = l.split(' ').collect::<Vec<_>>();
            if words[0] == "mask" {
                let mask_str = String::from(words[2]);
                let overwrite_pattern = mask_str.replace('X', "0");
                let overwrite_pattern = u64::from_str_radix(&overwrite_pattern, 2).unwrap();
                let float_pattern = mask_str.replace('1', "0").replace('X', "1");
                let float_pattern = u64::from_str_radix(&float_pattern, 2).unwrap();
                let mask = FloatMask {
                    overwrite_pattern,
                    float_pattern,
                };
                Instruction::FloatMask(mask)
            } else if words[0].starts_with("mem[") {
                let location = String::from(&words[0][4..(words[0].len() - 1)]);
                let location: u64 = location.parse().unwrap();
                let value: u64 = words[2].parse().unwrap();
                Instruction::Memory(location, value)
            } else {
                unreachable!()
            }
        })
        .collect();

    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = FloatMask::default();
    for instr in instructions {
        match instr {
            Instruction::FloatMask(mask) => {
                current_mask = mask;
            }
            Instruction::Memory(location, value) => {
                for loc in current_mask.apply(location) {
                    mem.insert(loc, value);
                }
            }
            _ => unreachable!(),
        }
    }
    mem.values().sum()
}
