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
    let slope_x = 3;
    let slope_y = 1;

    input
        .lines()
        .step_by(slope_y)
        .enumerate()
        .filter(|(i, line)| line.chars().cycle().nth(slope_x * i).unwrap() == '#')
        .count()
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

    const TEST_INPUT: &'static str = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT.to_string()), 7);
    }
    #[test]
    fn tes2() {}
}
