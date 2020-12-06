use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let limits = parts.next().unwrap();
            let mut limits = limits.split('-').map(|s| s.parse().unwrap());
            Password {
                limits: (limits.next().unwrap(), limits.next().unwrap()),
                letter: String::from(parts.next().unwrap()).chars().next().unwrap(),
                password: String::from(parts.next().unwrap()),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Password]) -> usize {
    input.iter().filter(|&p| is_password_valid_part1(p)).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Password]) -> usize {
    input.iter().filter(|&p| is_password_valid_part2(p)).count()
}

pub struct Password {
    limits: (usize, usize),
    letter: char,
    password: String,
}

fn is_password_valid_part1(pass: &Password) -> bool {
    let count = pass.password.matches(pass.letter).count();
    count >= pass.limits.0 && count <= pass.limits.1
}

fn is_password_valid_part2(pass: &Password) -> bool {
    let (letter1, letter2) = (
        pass.password.chars().nth(pass.limits.0 - 1).unwrap(),
        pass.password.chars().nth(pass.limits.1 - 1).unwrap(),
    );
    (letter1 == pass.letter) ^ (letter2 == pass.letter)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_password_valid_part1_test() {
        let p = Password {
            letter: 'p',
            limits: (1, 2),
            password: String::from("validpassword"),
        };
        assert!(is_password_valid_part1(&p));

        // false when doesn't exist
        let p = Password {
            letter: 'p',
            limits: (1, 2),
            password: String::from("invalid"),
        };
        assert!(!is_password_valid_part1(&p));

        // false when too many occurences are there
        let p = Password {
            letter: 'p',
            limits: (1, 2),
            password: String::from("pppppppp"),
        };
        assert!(!is_password_valid_part1(&p));
    }

    #[test]
    fn is_password_valid_part2_test() {
        // true, occurs at 1st pos
        let p = Password {
            letter: 'p',
            limits: (1, 2),
            password: String::from("pas"),
        };
        assert!(is_password_valid_part2(&p));

        // true, occurs at 2nd pos
        let p = Password {
            letter: 'p',
            limits: (1, 2),
            password: String::from("aps"),
        };
        assert!(is_password_valid_part2(&p));

        // false, doesn't exist
        let p = Password {
            letter: 'p',
            limits: (1, 2),
            password: String::from("invalid"),
        };
        assert!(!is_password_valid_part2(&p));

        // false, when occurs at both pos
        let p = Password {
            letter: 'p',
            limits: (1, 2),
            password: String::from("pppppppp"),
        };
        assert!(!is_password_valid_part2(&p));
    }
}
