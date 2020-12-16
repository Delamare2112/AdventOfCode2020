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

fn s2b(input: &str, set: char) -> usize {
    let mut ret = 0;
    for (i, v) in input.chars().rev().enumerate() {
        if v == set {
            ret = ret | (1 << i);
        }
    }
    ret
}

fn get_row(input: &str) -> usize {
    s2b(&input.replace('L', "").replace('R', ""), 'B')
}

fn get_column(input: &str) -> usize {
    s2b(&input.replace('F', "").replace('B', ""), 'R')
}

fn get_seat_id(input: &str) -> usize {
    let row = get_row(input);
    let column = get_column(input);
    (row * 8) + column
}

fn part1(input: String) -> usize {
    input
        .lines()
        .map(|line| get_seat_id(line))
        .max()
        .expect("Failed to get a max seat ID!")
}

fn part2(input: String) -> usize {
    let mut sorted = input
        .lines()
        .map(|line| get_seat_id(line))
        .collect::<Vec<usize>>();
    sorted.sort();
    let (_, b) = sorted
        .iter()
        .skip(1)
        .zip(sorted.iter())
        .find(|(&a, &b)| a - b != 1)
        .expect("Failed to find part 2!");
    b + 1
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
    use crate::{get_column, get_row, get_seat_id, s2b};

    #[test]
    fn s2bt() {
        let input = "FBFBBFFRLR";
        assert_eq!(s2b(&input[0..7], 'B'), 44);
        assert_eq!(s2b(&input[7..], 'R'), 5);
        assert_eq!(get_row(input), 44);
        assert_eq!(get_column(input), 5);
    }
    #[test]
    fn test1() {
        assert_eq!(get_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(get_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(get_seat_id("BBFFBBFRLL"), 820);
    }
    #[test]
    fn tes2() {}
}
