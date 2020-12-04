use std::str::FromStr;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|x| x.replace("\n", " "))
        .map(|x| parse_passport(x.as_str()))
        .collect()
}

fn parse_passport(line: &str) -> Passport {
    let mut passport: Passport = Default::default();
    line.split(' ').for_each(|part| {
        let pair: Vec<_> = part.splitn(2, ':').collect();
        let key = pair[0];
        let val = pair[1];
        match key {
            "byr" => passport.byr = Some(val.to_owned()),
            "iyr" => passport.iyr = Some(val.to_owned()),
            "eyr" => passport.eyr = Some(val.to_owned()),
            "hgt" => passport.hgt = Some(val.to_owned()),
            "hcl" => passport.hcl = Some(val.to_owned()),
            "ecl" => passport.ecl = Some(val.to_owned()),
            "pid" => passport.pid = Some(val.to_owned()),
            "cid" => passport.cid = Some(val.to_owned()),
            _ => {}
        }
    });
    passport
}

#[aoc(day4, part1)]
pub fn solve_part1(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|&p| p.are_required_fields_present())
        .count()
}

#[aoc(day4, part2)]
pub fn solve_part2(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|&p| p.are_required_fields_present() && p.are_required_fields_valid())
        .count()
}

#[derive(Default, Debug)]
pub struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn are_required_fields_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn are_required_fields_valid(&self) -> bool {
        self.is_byr_valid()
            && self.is_iyr_valid()
            && self.is_eyr_valid()
            && self.is_hgt_valid()
            && self.is_hcl_valid()
            && self.is_ecl_valid()
            && self.is_pid_valid()
            && self.is_cid_valid()
    }

    fn is_byr_valid(&self) -> bool {
        if let Some(year) = &self.byr {
            return if let Ok(year) = u32::from_str(year) {
                year >= 1920 && year <= 2002
            } else {
                false
            };
        }
        false
    }

    fn is_iyr_valid(&self) -> bool {
        if let Some(year) = &self.iyr {
            return if let Ok(year) = u32::from_str(year) {
                year >= 2010 && year <= 2020
            } else {
                false
            };
        }
        false
    }

    fn is_eyr_valid(&self) -> bool {
        if let Some(year) = &self.eyr {
            return if let Ok(year) = u32::from_str(year) {
                year >= 2020 && year <= 2030
            } else {
                false
            };
        }
        false
    }

    fn is_hgt_valid(&self) -> bool {
        if self.hgt.is_none() {
            return false;
        }
        let re = regex::Regex::new("^(?P<height>\\d+)(?P<unit>cm|in)$").unwrap();

        let captures = re.captures(&self.hgt.as_ref().unwrap());
        if captures.is_none() {
            return false;
        }
        let captures = captures.unwrap();
        if captures.len() < 2 {
            return false;
        }

        if let (Some(height), Some(unit)) = (captures.name("height"), captures.name("unit")) {
            let height = u32::from_str(height.as_str()).ok();
            if height.is_none() {
                return false;
            }
            let height = height.unwrap();
            return match unit.as_str() {
                "cm" => height >= 150 && height <= 193,
                "in" => height >= 59 && height <= 76,
                _ => panic!("unknown unit: {}", unit.as_str()),
            };
        }
        false
    }

    fn is_hcl_valid(&self) -> bool {
        if let Some(color) = &self.hcl {
            return regex::Regex::new("^#[0-9a-f]{6}$")
                .unwrap()
                .is_match(color.as_str());
        }
        false
    }

    fn is_ecl_valid(&self) -> bool {
        if let Some(color) = &self.ecl {
            return match color.as_str() {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false,
            };
        }
        false
    }

    fn is_pid_valid(&self) -> bool {
        if let Some(pid) = &self.pid {
            return regex::Regex::new("^[0-9]{9}$")
                .unwrap()
                .is_match(pid.as_str());
        }
        false
    }

    fn is_cid_valid(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_part1_test() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let passports = input_generator(input);
        assert_eq!(solve_part1(passports.as_slice()), 2);

        let input = "hcl:#866857 pid:983640144 hgt:61cm
ecl:hzl
byr:1991
iyr:1930 eyr:2024";
        let passports = input_generator(input);
        assert_eq!(solve_part1(passports.as_slice()), 1);
    }

    #[test]
    fn solve_part2_test() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let passports = input_generator(input);
        assert_eq!(solve_part2(passports.as_slice()), 4);

        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        let passports = input_generator(input);
        assert_eq!(solve_part2(passports.as_slice()), 0);
    }
}
