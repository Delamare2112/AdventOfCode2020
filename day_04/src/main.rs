#![feature(str_split_once)]

use std::collections::HashMap;
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

struct Passport(HashMap<String, String>);

impl Passport {
    fn valid(&self) -> bool {
        let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        required_keys.iter().all(|&key| self.0.contains_key(key))
    }
}

fn part1(input: String) -> usize {
    parse(input.to_string())
        .iter()
        .filter(|passport| passport.valid())
        .count()
}

fn part2(_input: String) -> usize {
    unimplemented!()
}

fn parse(input: String) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|passport| {
            Passport(
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
    use crate::part1;

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
    fn tes2() {}
}
