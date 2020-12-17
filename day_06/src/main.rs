use std::collections::HashSet;
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

fn part1(input: String) -> usize {
    input.split("\n\n").fold(0, |a, group| {
        a + group
            .chars()
            .filter(|&c| c != '\n')
            .collect::<HashSet<char>>()
            .len()
    })
}

fn part2(input: String) -> usize {
    input.split("\n\n").fold(0, |a, group| {
        let mut first = group.lines().next().unwrap().chars().collect::<Vec<_>>();
        for line in group.lines().skip(1) {
            let mut to_remove = Vec::new();
            for (i, &a) in first.iter().enumerate() {
                if !line.contains(a) {
                    to_remove.push(i);
                }
            }
            to_remove.sort();
            for (j, k) in to_remove.iter().enumerate() {
                first.remove(k - j);
            }
        }
        a + first.len()
    })
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
        "2" | "two" => part2(input), // 970 is too low!
        _ => panic!("i need a part NUMBER!!!!"),
    };
    println!("It's: {}", output);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const INPUT: &'static str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT.to_string()), 11);
    }
    #[test]
    fn tes2() {
        assert_eq!(part2(INPUT.to_string()), 6);
    }
}
