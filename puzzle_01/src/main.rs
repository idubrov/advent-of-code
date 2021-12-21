use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let b = std::io::BufReader::new(std::fs::File::open("puzzle_01/src/input.txt").unwrap());
    println!(
        "{}",
        b.lines()
            .collect::<Result<Vec<String>, _>>()
            .unwrap()
            .windows(2)
            .map(|w| i32::from_str(&w[0]).unwrap() < i32::from_str(&w[1]).unwrap())
            .filter(|x| *x)
            .count()
    );
}
