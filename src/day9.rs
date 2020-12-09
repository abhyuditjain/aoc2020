use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::convert::identity;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &[isize]) -> isize {
    get_first_invalid_number(input, 25)
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &[isize]) -> isize {
    get_encryption_weakness(input, 25)
}

fn get_first_invalid_number(nums: &[isize], preamble_size: usize) -> isize {
    nums.windows(preamble_size + 1)
        .filter_map(
            |x| match has_two_sum(&x[..preamble_size], x[preamble_size]) {
                false => Some(x[preamble_size]),
                _ => None,
            },
        )
        .collect::<Vec<isize>>()
        .first()
        .cloned()
        .unwrap()
}

fn get_encryption_weakness(nums: &[isize], preamble_size: usize) -> isize {
    let first_invalid_number = get_first_invalid_number(nums, preamble_size);

    let contiguous_set: &[isize] = (2..nums.len())
        .map(|window_size| {
            nums.windows(window_size)
                .find(|window| window.iter().sum::<isize>() == first_invalid_number)
        })
        .find_map(identity)
        .unwrap();

    let min = contiguous_set.iter().min().unwrap();
    let max = contiguous_set.iter().max().unwrap();
    min + max
}

fn has_two_sum(arr: &[isize], sum: isize) -> bool {
    let mut set = HashSet::new();
    for x in arr {
        if set.contains(&(sum - *x)) {
            return true;
        }
        set.insert(x);
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_first_invalid_number_test() {
        let input: Vec<isize> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(get_first_invalid_number(input.as_slice(), 5), 127);
    }

    #[test]
    fn get_encryption_weakness_test() {
        let input: Vec<isize> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(get_encryption_weakness(input.as_slice(), 5), 62);
    }
}
