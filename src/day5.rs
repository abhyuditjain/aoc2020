use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'F' => Direction::FRONT,
                    'B' => Direction::BACK,
                    'L' => Direction::LEFT,
                    'R' => Direction::RIGHT,
                    _ => panic!("Invalid character: {}", c),
                })
                .collect()
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Vec<Direction>]) -> u16 {
    input.iter().map(|x| get_seat_id(x)).max().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Vec<Direction>]) -> u16 {
    let mut seat_ids: Vec<u16> = input.iter().map(|x| get_seat_id(x)).collect();
    seat_ids.sort_unstable();

    let mut result = None;
    let mut previous = seat_ids[0];
    let mut it = seat_ids.iter().skip(1).peekable();
    while let Some(sid) = it.next() {
        if let Some(&&next_sid) = it.peek() {
            if !(previous == sid - 1 && next_sid == sid + 1) {
                result = Some(sid + 1);
                break;
            }
            previous = *sid;
        }
    }
    result.unwrap()
}

fn get_seat_id(steps: &[Direction]) -> u16 {
    (get_row(steps) * 8) + get_seat(steps)
}

fn get_row(remaining: &[Direction]) -> u16 {
    binary_partition(0, 127, &remaining[0..7])
}

fn get_seat(remaining: &[Direction]) -> u16 {
    binary_partition(0, 7, &remaining[7..])
}

fn binary_partition(low: u16, high: u16, steps: &[Direction]) -> u16 {
    if steps.is_empty() {
        return low;
    }
    match steps.first().unwrap() {
        Direction::FRONT | Direction::LEFT => binary_partition(low, (low + high) / 2, &steps[1..]),
        Direction::BACK | Direction::RIGHT => {
            binary_partition(((low + high) / 2) + 1, high, &steps[1..])
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    FRONT,
    BACK,
    LEFT,
    RIGHT,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_row_test() {
        use Direction::{BACK, FRONT, LEFT, RIGHT};
        let steps = vec![
            FRONT, BACK, FRONT, BACK, BACK, FRONT, FRONT, RIGHT, LEFT, RIGHT,
        ];
        assert_eq!(get_row(&steps), 44);
    }

    #[test]
    fn get_seat_test() {
        use Direction::{BACK, FRONT, LEFT, RIGHT};
        let steps = vec![
            FRONT, BACK, FRONT, BACK, BACK, FRONT, FRONT, RIGHT, LEFT, RIGHT,
        ];
        assert_eq!(get_seat(&steps), 5);
    }
}
