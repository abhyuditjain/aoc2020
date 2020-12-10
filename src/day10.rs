use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<usize> {
    let mut input: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    input.push(0);
    input.push(3 + input.iter().max().copied().unwrap());
    input.sort_unstable();
    input
}

#[aoc(day10, part1)]
pub fn solve_part1(adapters: &[usize]) -> usize {
    let mut previous_adapter = 0;
    let mut diff1 = 0;
    let mut diff3 = 0;
    for &adapter in adapters {
        if adapter - previous_adapter == 1 {
            diff1 += 1;
        }
        if adapter - previous_adapter == 3 {
            diff3 += 1;
        }
        previous_adapter = adapter
    }
    diff1 * diff3
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    rec(input, 0, &mut HashMap::new())
}

fn rec(ads: &[usize], i: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if cache.contains_key(&i) {
        return *cache.get(&i).unwrap();
    }
    if i == ads.len() - 1 {
        return 1;
    }

    let mut res = 0;
    for j in (i + 1)..=(i + 3) {
        if j < ads.len() && ads[j] - ads[i] <= 3 {
            res += rec(&ads, j, cache);
        }
    }
    cache.insert(i, res);

    res
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_1: &[usize] = &[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    #[test]
    fn solve_part1_test() {
        assert_eq!(solve_part1(EXAMPLE_1), 35);
    }
}
