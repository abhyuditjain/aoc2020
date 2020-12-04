#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut map = Vec::new();
    input.lines().for_each(|l| {
        l.chars().for_each(|ch| map.push(ch == '#'));
    });
    Map { width, height, map }
}

#[aoc(day3, part1)]
pub fn solve_part1(map: &Map) -> usize {
    tree_count(map, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(map: &Map) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().map(|&(x, y)| tree_count(map, x, y)).product()
}

fn tree_count(map: &Map, slope_x: usize, slope_y: usize) -> usize {
    (0..)
        .step_by(slope_x)
        .zip((0..map.height).step_by(slope_y))
        .map(|(x, y)| map.is_tree(x, y))
        .filter(|&tree| tree)
        .count()
}

pub struct Map {
    width: usize,
    height: usize,
    map: Vec<bool>,
}

impl Map {
    fn is_tree(&self, x: usize, y: usize) -> bool {
        self.map[y * self.width + (x % self.width)]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree_count_test() {
        let map = Map {
            width: 2,
            height: 2,
            map: vec![true, true, false, true],
        };

        assert_eq!(tree_count(&map, 1, 1), 2);
        assert_eq!(tree_count(&map, 2, 1), 1);
    }
}
