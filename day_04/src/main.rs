#![feature(str_split_once)]

use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Settings {
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short, long)]
    part: String,
}

#[derive(Debug)]
enum Units {
    In(usize),
    Cm(usize),
}

#[derive(Debug)]
struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: Units,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn valid(&self) -> bool {
        if self.byr < 1920 || self.byr > 2002 {
            return false;
        }
        if self.iyr < 2010 || self.iyr > 2020 {
            return false;
        }
        if self.eyr < 2020 || self.eyr > 2030 {
            return false;
        }
        match self.hgt {
            Units::Cm(height) => {
                if height < 150 || height > 192 {
                    return false;
                }
            }
            Units::In(height) => {
                if height < 59 || height > 76 {
                    return false;
                }
            }
        }
        if !self.hcl.starts_with('#') {
            return false;
        }
        if usize::from_str_radix(&self.hcl[1..], 16).is_err() {
            return false;
        }
        let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if !valid_eye_colors.iter().any(|&c| self.ecl.as_str() == c) {
            return false;
        }
        if self.pid.len() != 9 {
            return false;
        }
        if self.pid.parse::<usize>().is_err() {
            return false;
        }
        true
    }
}

impl TryFrom<PassportStrings> for Passport {
    type Error = &'static str;

    fn try_from(strings: PassportStrings) -> Result<Self, Self::Error> {
        if !strings.valid() {
            return Err("Missing values");
        }
        Ok(Self {
            byr: strings.0["byr"].parse().unwrap(),
            iyr: strings.0["iyr"].parse().unwrap(),
            eyr: strings.0["eyr"].parse().unwrap(),
            hgt: {
                let s = &strings.0["hgt"];
                let value = s[0..s.len() - 2].parse().unwrap();
                if s.ends_with("in") {
                    Units::In(value)
                } else if s.ends_with("cm") {
                    Units::Cm(value)
                } else {
                    return Err("Invalid height");
                }
            },
            hcl: strings.0["hcl"].clone(),
            ecl: strings.0["ecl"].clone(),
            pid: strings.0["pid"].clone(),
        })
    }
}

struct PassportStrings(HashMap<String, String>);

impl PassportStrings {
    fn valid(&self) -> bool {
        let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        required_keys.iter().all(|&key| self.0.contains_key(key))
    }
}

fn part1(input: String) -> usize {
    input_to_passport_strings(input.to_string())
        .iter()
        .filter(|passport| passport.valid())
        .count()
}

fn part2(input: String) -> usize {
    input_to_passport_strings(input.to_string())
        .drain(..)
        .filter_map(|passport_strings| Passport::try_from(passport_strings).ok())
        .filter(|passport| passport.valid())
        .count()
}

fn input_to_passport_strings(input: String) -> Vec<PassportStrings> {
    input
        .split("\n\n")
        .map(|passport| {
            PassportStrings(
                passport
                    .replace('\n', " ")
                    .split_ascii_whitespace()
                    .map(|pair| {
                        let bits = pair.split_once(':').unwrap();
                        (bits.0.to_string(), bits.1.to_string())
                    })
                    .collect(),
            )
        })
        .collect()
}

fn main() {
    let settings = Settings::from_args();
    // This is silly and I know it.  I am just playing around :)
    let input = {
        let mut input = String::new();
        std::fs::File::open(settings.input)
            .expect("Failed to open input file!")
            .read_to_string(&mut input)
            .expect("Failed to read all of the input file to a string!");
        input
    };

    let output = match settings.part.to_lowercase().as_str() {
        "1" | "one" => part1(input),
        "2" | "two" => part2(input),
        _ => panic!("i need a part NUMBER!!!!"),
    };
    println!("It's: {}", output);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const TEST_INPUT: &'static str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#; // TODO: split on empty lines, then build a dictionary split with ':'

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT.to_string()), 2);
    }
    #[test]
    fn tes2() {
        let valid = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        let invalid = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;
        assert_eq!(part2(valid.to_string()), 4);
        assert_eq!(part2(invalid.to_string()), 0);
    }
}
