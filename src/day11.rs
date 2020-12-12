use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Seat {
    Occupied,
    Floor,
    Empty,
}

const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
    (1, 0),
    (-1, 0),
];

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Seat::Floor => '.',
                Seat::Empty => 'L',
                Seat::Occupied => '#',
            }
        )
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Map(Vec<Vec<Seat>>);

impl fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for seat in row {
                write!(f, "{}", seat)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn get_seat(&self, i: isize, j: isize) -> Option<Seat> {
        if i < 0 || j < 0 || i >= self.0.len() as isize || j >= self.0[0].len() as isize {
            return None;
        }
        Some((self.0)[i as usize][j as usize].clone())
    }

    fn num_rows(&self) -> usize {
        self.0.len()
    }

    fn num_cols(&self) -> usize {
        if self.num_rows() == 0 {
            return 0;
        }
        self.0[0].len()
    }

    fn set_seat(&mut self, i: usize, j: usize, seat: Seat) {
        self.0[i][j] = seat
    }

    fn count_adjacent_seats(&self, i: usize, j: usize, seat_type: Seat) -> usize {
        DIRS.iter().fold(0, |sum, &(di, dj)| {
            match self.get_seat(i as isize + di, j as isize + dj) {
                Some(seat) if seat == seat_type => sum + 1,
                _ => sum,
            }
        })
    }

    fn count_adjacent_seats_visible(&self, i: usize, j: usize, seat_type: Seat) -> usize {
        let mut ct = 0;

        for &(di, dj) in DIRS.iter() {
            let (mut ni, mut nj) = (i as isize + di, j as isize + dj);
            while self.get_seat(ni, nj) == Some(Seat::Floor) {
                let (ni2, nj2) = (ni + di, nj + dj);
                ni = ni2;
                nj = nj2;
            }

            if let Some(seat) = self.get_seat(ni, nj) {
                if seat == seat_type {
                    ct += 1;
                }
            }
        }
        ct
    }

    fn count_seats(&self, seat_type: Seat) -> usize {
        self.0.iter().flatten().filter(|&x| x == &seat_type).count()
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Map {
    Map(input
        .lines()
        .map(|l| {
            l.bytes()
                .map(|b| match b {
                    b'.' => Seat::Floor,
                    b'L' => Seat::Empty,
                    b'#' => Seat::Occupied,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect())
}

#[aoc(day11, part1)]
pub fn solve_part1(map: &Map) -> usize {
    simulate(map, tick1)
}

#[aoc(day11, part2)]
pub fn solve_part2(map: &Map) -> usize {
    simulate(map, tick2)
}

fn simulate<F>(map: &Map, tick: F) -> usize
where
    F: Fn(&Map) -> Map,
{
    let mut last = map.clone();
    let mut curr = tick(map);
    while last != curr {
        last = curr.clone();
        curr = tick(&curr);
    }

    curr.count_seats(Seat::Occupied)
}

fn tick1(map: &Map) -> Map {
    let mut new_map = map.clone();

    for i in 0..map.num_rows() {
        for j in 0..map.num_cols() {
            if let Some(seat) = map.get_seat(i as isize, j as isize) {
                if seat == Seat::Floor {
                    continue;
                }
                let adjacent_count = map.count_adjacent_seats(i, j, Seat::Occupied);
                if seat == Seat::Occupied && adjacent_count >= 4 {
                    new_map.set_seat(i, j, Seat::Empty);
                } else if seat != Seat::Occupied && adjacent_count == 0 {
                    new_map.set_seat(i, j, Seat::Occupied);
                } else {
                    new_map.set_seat(i, j, map.get_seat(i as isize, j as isize).unwrap());
                }
            }
        }
    }
    new_map
}

fn tick2(map: &Map) -> Map {
    let mut new_map = map.clone();

    for i in 0..map.num_rows() {
        for j in 0..map.num_cols() {
            if let Some(seat) = map.get_seat(i as isize, j as isize) {
                if seat == Seat::Floor {
                    continue;
                }
                let ct = map.count_adjacent_seats_visible(i, j, Seat::Occupied);
                if seat == Seat::Occupied && ct >= 5 {
                    new_map.set_seat(i, j, Seat::Empty);
                } else if seat != Seat::Occupied && ct == 0 {
                    new_map.set_seat(i, j, Seat::Occupied);
                } else {
                    new_map.set_seat(i, j, map.get_seat(i as isize, j as isize).unwrap());
                }
            }
        }
    }

    new_map
}

#[cfg(test)]
mod test {
    use super::{Seat::*, *};

    use lazy_static::lazy_static;

    lazy_static! {
        static ref MAP: Map = Map(vec![
            vec![Floor, Occupied, Occupied, Floor, Occupied, Occupied, Floor],
            vec![Occupied, Floor, Occupied, Floor, Occupied, Floor, Occupied],
            vec![Occupied, Occupied, Floor, Floor, Floor, Occupied, Occupied],
            vec![Floor, Floor, Floor, Empty, Floor, Floor, Floor],
            vec![Occupied, Occupied, Floor, Floor, Floor, Occupied, Occupied],
            vec![Occupied, Floor, Occupied, Floor, Occupied, Floor, Occupied],
            vec![Floor, Occupied, Occupied, Floor, Occupied, Occupied, Floor],
        ]);
    }

    #[test]
    fn solve_part1_test() {
        assert_eq!(solve_part1(&MAP), 25);
    }

    #[test]
    fn solve_part2_test() {
        assert_eq!(solve_part2(&MAP), 9);
    }
}
