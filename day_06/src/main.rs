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

fn part2(_input: String) -> usize {
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
        "2" | "two" => part2(input),
        _ => panic!("i need a part NUMBER!!!!"),
    };
    println!("It's: {}", output);
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn test1() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#
        .to_string();
        assert_eq!(part1(input), 11);
    }
    #[test]
    fn tes2() {}
}
