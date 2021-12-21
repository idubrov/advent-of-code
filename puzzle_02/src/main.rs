use std::io::BufRead;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Command {
  Forward(i32),
  Down(i32),
  Up(i32),
}

impl FromStr for Command {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(if s.starts_with("forward ") {
      Command::Forward(s["forward ".len()..].parse().unwrap())
    } else if s.starts_with("down ") {
      Command::Down(s["down ".len()..].parse().unwrap())
    } else if s.starts_with("up ") {
      Command::Up(s["up ".len()..].parse().unwrap())
    } else {
      panic!()
    })
  }
}

#[derive(Clone, Copy, Debug)]
struct Pos {
  horizontal: i32,
  depth_or_aim: i32,
  depth: i32,
}

impl std::ops::AddAssign<Command> for Pos {
  fn add_assign(&mut self, rhs: Command) {
    match rhs {
      Command::Forward(x) => {
        self.horizontal += x;
        self.depth += self.depth_or_aim * x;
      }
      Command::Up(x) => self.depth_or_aim -= x,
      Command::Down(x) => self.depth_or_aim += x,
    }
  }
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_02/src/input.txt").unwrap());
  let input = buf
    .lines()
    .map(|s| s.unwrap().parse().unwrap())
    .collect::<Vec<Command>>();
  let mut pos = Pos {
    horizontal: 0,
    depth_or_aim: 0,
    depth: 0,
  };

  for cmd in input {
    pos += cmd;
  }
  eprintln!("{}", pos.horizontal * pos.depth_or_aim);
  eprintln!("{}", pos.horizontal * pos.depth);
}
