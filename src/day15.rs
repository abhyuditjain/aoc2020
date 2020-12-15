use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, VecDeque};

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<u32> {
    input.split(',').map(|w| w.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
fn solve_part1(input: &[u32]) -> u32 {
    run(&input, 2020)
}

#[aoc(day15, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    run(&input, 30000000)
}

fn run(input: &[u32], turn_count: u32) -> u32 {
    let mut spoken: HashMap<u32, VecDeque<u32>> = HashMap::new();
    let mut last_spoken = 0;

    for (turn, n) in input.iter().cloned().enumerate() {
        speak_number(&mut spoken, &mut last_spoken, turn as u32 + 1, n);
    }

    for turn in input.len() as u32 + 1..=turn_count {
        if let Some(v) = spoken.get_mut(&last_spoken) {
            if v.len() == 1 {
                speak_number(&mut spoken, &mut last_spoken, turn, 0);
            } else {
                let before_last = v.pop_front().unwrap();
                let diff = v[v.len() - 1] - before_last;
                speak_number(&mut spoken, &mut last_spoken, turn, diff);
            }
        } else {
            unreachable!();
        }
    }

    last_spoken
}

#[inline(always)]
fn speak_number(
    spoken: &mut HashMap<u32, VecDeque<u32>>,
    last_spoken: &mut u32,
    turn: u32,
    n: u32,
) {
    spoken
        .entry(n)
        .or_insert_with(VecDeque::new)
        .push_back(turn);
    *last_spoken = n;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test() {
        let input: &[u32] = &[0, 3, 6];
        assert_eq!(run(input, 4), 0);
        assert_eq!(run(input, 2020), 436);
    }
}
