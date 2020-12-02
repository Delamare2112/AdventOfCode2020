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

fn parse_input(input: String) -> Vec<PasswordEntry> {
    input
        .lines()
        .map(|line| line.parse().expect("Failed to parse an input line!"))
        .collect()
}

fn part1(input: String) -> usize {
    let input = parse_input(input);
    unimplemented!()
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
        "2" | "two" => unimplemented!(),
        _ => panic!("i need a part NUMBER!!!!"),
    };
    println!("It's: {}", output);
}
