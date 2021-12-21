use std::io::BufRead;
use std::str::FromStr;

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_07/src/input.txt").unwrap());
  let positions = buf.lines()
    .next()
    .unwrap()
    .unwrap()
    .split(",")
    .map(|x| i32::from_str(x).unwrap())
    .collect::<Vec<_>>();

  let max = positions.iter().copied().max().unwrap();
  let mut min = i32::MAX;
  for pos in 0..=max {
    let fuel = positions.iter().map(|x| (*x - pos).abs()).sum();
    min = min.min(fuel);
  }
  eprintln!("{}", min);

  let max = positions.iter().copied().max().unwrap();
  let mut min = i32::MAX;
  for pos in 0..=max {
    let fuel = positions.iter().map(|x| {
      let delta = (*x - pos).abs();
      delta * (delta + 1) / 2
    }).sum();
    min = min.min(fuel);
  }

  eprintln!("{}", min);
}
