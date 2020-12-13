use aoc_runner_derive::{aoc, aoc_generator};
use num::Integer;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (u64, Vec<(u64, u64)>) {
    let mut lines = input.lines();
    let arrival = lines.next().unwrap().parse().unwrap();
    let times: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, x)| match x {
            "x" => None,
            _ => Some((i as u64, x.parse().unwrap())),
        })
        .collect();
    (arrival, times)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(u64, Vec<(u64, u64)>)) -> u64 {
    let (wait, n) = input
        .1
        .iter()
        .map(|(_, x)| (x - (input.0 % x), x))
        .min()
        .unwrap();

    wait * n
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &(u64, Vec<(u64, u64)>)) -> u64 {
    let mut delta = 1;
    let mut t = 0;

    for (offset, dt) in &input.1 {
        loop {
            if (t + *offset) % dt == 0 {
                break;
            }
            t += delta;
        }
        delta = delta.lcm(dt);
    }
    t
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::join;

    const INPUT: &[&str] = &["939", "7,13,x,x,59,x,31,19"];

    #[test]
    fn solve_part1_test() {
        assert_eq!(
            solve_part1(&input_generator(join(INPUT, "\n").as_str())),
            295
        );
    }

    #[test]
    fn solve_part2_test() {
        assert_eq!(
            solve_part2(&input_generator(join(INPUT, "\n").as_str())),
            1068781
        );
    }
}
