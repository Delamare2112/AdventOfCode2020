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

fn input_to_ilines(input: String) -> Vec<isize> {
    input
        .lines()
        .map(|line| line.parse().expect("WHAT IS thiS LiNE?!?!!?"))
        .collect()
}

fn part1(input: String) -> isize {
    let input = input_to_ilines(input);
    for (i, a) in input.iter().enumerate() {
        if let Some(b) = input.iter().skip(i + 1).find(|b| *a + **b == 2020) {
            return a * b;
        }
    }
    panic!("No answer found!")
}

fn part2(input: String) -> isize {
    let input = input_to_ilines(input);
    for (ai, a) in input.iter().enumerate() {
        for (bi, b) in input
            .iter()
            .enumerate()
            .skip(ai as usize)
            .filter(|(_, b)| (*a + *b).abs() < 2020)
        {
            if let Some(c) = input.iter().skip(bi).find(|c| *a + *b + **c == 2020) {
                return a * b * c;
            }
        }
    }
    panic!("No answer found!")
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
