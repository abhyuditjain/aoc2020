use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let (x, y) = two_sum(input, 2020).unwrap();
    x * y
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    for i in 0..(input.len() - 2) {
        if let Some((x, y)) = two_sum(&input[i + 1..], 2020 - input[i]) {
            return x * y * input[i];
        }
    }
    unreachable!()
}

fn two_sum(nums: &[u32], sum: u32) -> Option<(u32, u32)> {
    if nums.len() < 2 {
        return None;
    }
    let mut set = HashSet::new();
    for &x in nums.iter() {
        if set.contains(&(sum - x)) {
            return Some((sum - x, x));
        }
        set.insert(x);
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_sum_test() {
        // input slice length < 2
        assert!(two_sum(&[2], 1).is_none());

        // 2 sum exists
        assert!(two_sum(&[1, 2, 3, 4], 7).is_some());
        assert_eq!(two_sum(&[1, 2, 3, 4], 7), Some((3, 4)));

        // Non existent sum
        assert!(two_sum(&[1, 2, 3, 4], 8).is_none());
    }
}