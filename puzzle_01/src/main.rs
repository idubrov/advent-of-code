use std::io::BufRead;

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_01/src/input.txt").unwrap());
  let input = buf.lines().map(|s| s.unwrap().parse().unwrap()).collect::<Vec<i32>>();

  let first = input.windows(2).map(|w| w[0] < w[1]).filter(|x| *x).count();
  println!("{}", first);

  let input2 = input.windows(3).map(|w| w[0] + w[1] + w[2]).collect::<Vec<_>>();
  let second = input2.windows(2).map(|w| w[0] < w[1]).filter(|x| *x).count();
  println!("{}", second);
}
