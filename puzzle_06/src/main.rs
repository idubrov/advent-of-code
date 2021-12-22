use std::io::BufRead;
use std::str::FromStr;

struct State {
  fish: [i128; 9],
}

impl State {
  fn new(fishes: &[i32]) -> State {
    let mut state = State { fish: [0; 9] };
    for fish in fishes {
      state.fish[*fish as usize] += 1;
    }
    state
  }
  fn tick(&mut self) {
    let mut add = self.fish[0];
    for i in 0..8 {
      self.fish[i] = self.fish[i + 1];
    }
    self.fish[6] += add;
    self.fish[8] = add;
  }
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_06/src/input.txt").unwrap());
  let fish = buf
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .split(",")
    .map(|x| i32::from_str(x).unwrap())
    .collect::<Vec<_>>();

  let mut fish: State = State::new(&fish);
  for _ in 0..80 {
    fish.tick();
  }
  println!("{}", fish.fish.iter().copied().sum::<i128>());

  for _ in 0..(256 - 80) {
    fish.tick();
  }
  println!("{}", fish.fish.iter().copied().sum::<i128>());
}
