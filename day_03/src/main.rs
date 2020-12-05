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

struct Vec2<T> {
    x: T,
    y: T,
}

fn count_trees(input: &String, slope: &Vec2<usize>) -> usize {
    input
        .lines()
        .step_by(slope.y)
        .enumerate()
        .filter(|(i, line)| line.chars().cycle().nth(slope.x * i).unwrap() == '#')
        .count()
}

fn part1(input: String) -> usize {
    let slope = Vec2 { x: 3, y: 1 };
    count_trees(&input, &slope)
}

fn part2(input: String) -> usize {
    let slopes = [
        Vec2 { x: 1, y: 1 },
        Vec2 { x: 3, y: 1 },
        Vec2 { x: 5, y: 1 },
        Vec2 { x: 7, y: 1 },
        Vec2 { x: 1, y: 2 },
    ];
    slopes
        .iter()
        .fold(1, |acc, slope| count_trees(&input, slope) * acc)
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
    fn tes2() {
        assert_eq!(part2(TEST_INPUT.to_string()), 336);
    }
}
