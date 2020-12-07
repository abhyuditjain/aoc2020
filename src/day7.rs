use std::collections::{HashMap, VecDeque};
use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;

const GOAL: &str = "shiny gold";

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(\w+ \w+) bags contain (.*)").unwrap();
    static ref ITEM_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut bags = HashMap::<String, Vec<(usize, String)>>::new();
    for line in input.lines() {
        if let Some((item, items)) = LINE_RE
            .captures(line.as_ref())
            .and_then(|captures| Some((captures.get(1)?.as_str(), captures.get(2)?.as_str())))
        {
            bags.insert(
                item.to_string(),
                ITEM_RE
                    .captures_iter(items)
                    .filter_map(|captures| {
                        Some((
                            captures.get(1)?.as_str().parse().ok()?,
                            captures.get(2)?.as_str().to_string(),
                        ))
                    }).collect(),
            );
        }
    }
    bags
}

#[aoc(day7, part1)]
pub fn solve_part1(bags: &HashMap<String, Vec<(usize, String)>>) -> usize {
    bags.keys().filter(|x| expand(&bags, x).any(|(_, item)| item == GOAL)).count()
}

#[aoc(day7, part2)]
pub fn solve_part2(bags: &HashMap<String, Vec<(usize, String)>>) -> usize {
    expand(bags, GOAL).map(|(count, _)| count).sum()
}

#[derive(Debug)]
struct Expand<'a> {
    bags: &'a HashMap<String, Vec<(usize, String)>>,
    queue: VecDeque<(usize, &'a str)>,
}

fn expand<'a>(bags: &'a HashMap<String, Vec<(usize, String)>>, bag: &str) -> Expand<'a> {
    Expand {
        bags,
        queue: bags.get(bag).map_or_else(VecDeque::new, |items| {
            items
                .iter()
                .map(|(count, item)| (*count, item.as_str()))
                .collect()
        }),
    }
}

impl<'a> Iterator for Expand<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|(count, item)| {
            if let Some(items) = self.bags.get(item) {
                for (subcount, subitem) in items {
                    self.queue.push_back((count * subcount, subitem));
                }
            }
            (count, item)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::join;

    static EXAMPLE_1: &[&str] = &[
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    static EXAMPLE_2: &[&str] = &[
        "shiny gold bags contain 2 dark red bags.",
        "dark red bags contain 2 dark orange bags.",
        "dark orange bags contain 2 dark yellow bags.",
        "dark yellow bags contain 2 dark green bags.",
        "dark green bags contain 2 dark blue bags.",
        "dark blue bags contain 2 dark violet bags.",
        "dark violet bags contain no other bags.",
    ];

    #[test]
    fn part1_examples() {
        assert_eq!(4, solve_part1(&input_generator(join(EXAMPLE_1, "\n").as_str())));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(32, solve_part2(&input_generator(join(EXAMPLE_1, "\n").as_str())));
        assert_eq!(126, solve_part2(&input_generator(join(EXAMPLE_2, "\n").as_str())));
    }
}