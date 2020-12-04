use re_parse::ReParse;
use regex::Regex;
use serde_derive::Deserialize;
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

#[derive(Debug, Deserialize, ReParse)]
#[re_parse(regex = r"(?P<min>\d+)-(?P<max>\d+) (?P<c>\w): (?P<passwd>\w+)")]
struct PasswordEntry {
    min: usize,
    max: usize,
    c: char,
    passwd: String,
}

impl PasswordEntry {
    fn valid_sled(&self) -> bool {
        let count = self.passwd.chars().filter(|&c| c == self.c).count();
        (count >= self.min) && (count <= self.max)
    }
    fn valid_toboggan(&self) -> bool {
        let a = self.passwd.chars().nth(self.min - 1).unwrap();
        let b = self.passwd.chars().nth(self.max - 1).unwrap();
        (a == self.c) ^ (b == self.c)
    }
}

fn parse_input(input: String) -> Vec<PasswordEntry> {
    input
        .lines()
        .map(|line| line.parse().expect("Failed to parse an input line!"))
        .collect()
}

fn part1(input: String) -> usize {
    let input = parse_input(input);
    input.iter().filter(|e| e.valid_sled()).count()
}

fn part2(input: String) -> usize {
    let input = parse_input(input);
    input.iter().filter(|e| e.valid_toboggan()).count()
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
    use crate::PasswordEntry;

    #[test]
    fn test_sled() {
        let a: PasswordEntry = "1-3 a: abcde".parse().unwrap();
        let b: PasswordEntry = "1-3 b: cdefg".parse().unwrap();
        let c: PasswordEntry = "2-9 c: ccccccccc".parse().unwrap();
        assert!(a.valid_sled());
        assert!(!b.valid_sled());
        assert!(c.valid_sled());
    }
    #[test]
    fn test_toboggan() {
        let a: PasswordEntry = "1-3 a: abcde".parse().unwrap();
        let b: PasswordEntry = "1-3 b: cdefg".parse().unwrap();
        let c: PasswordEntry = "2-9 c: ccccccccc".parse().unwrap();
        assert!(a.valid_toboggan());
        assert!(!b.valid_toboggan());
        assert!(!c.valid_toboggan());
    }
}
