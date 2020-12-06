use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|group_answers| group_answers.lines().map(|str| str.to_string()).collect())
        .collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|group_answers| {
            group_answers
                .iter()
                .flat_map(|answer| answer.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|group_answers| {
            let answers = group_answers
                .iter()
                .map(|row| row.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();

            let mut in_all = answers[0].clone();
            for a in &answers[1..] {
                in_all.retain(|ch| a.contains(ch));
            }
            in_all.len()
        })
        .sum()
}
