use std::fmt::Display;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
struct Line {
  x1: i32,
  y1: i32,
  x2: i32,
  y2: i32,
}

impl FromStr for Line {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut it = s.split(" -> ");
    let (left, right) = (it.next().unwrap(), it.next().unwrap());
    let mut left = left.trim().split(",");
    let mut right = right.trim().split(",");
    let (x1, y1) = (left.next().unwrap().parse().unwrap(), left.next().unwrap().parse().unwrap());
    let (x2, y2) = (right.next().unwrap().parse().unwrap(), right.next().unwrap().parse().unwrap());
    Ok(Line {
      x1, y1, x2, y2
    })
  }
}

struct Board {
  width: i32,
  height: i32,
  board: Vec<i32>,
}

impl Board {
  fn new(width: i32, height: i32) -> Board {
    let mut board = Vec::new();
    board.resize((width * height) as usize, 0);
    Board {
      width,
      height,
      board,
    }
  }

  fn draw_straight(&mut self, line: &Line) {
    if line.x1 == line.x2 {
      let from = line.y1.min(line.y2);
      let to = line.y1.max(line.y2);
      for y in from..=to {
        self.board[(y * self.width + line.x1) as usize] += 1;
      }
    } else if line.y1 == line.y2 {
      let from = line.x1.min(line.x2);
      let to = line.x1.max(line.x2);
      for x in from..=to {
        self.board[(line.y1 * self.width + x) as usize] += 1;
      }
    }
  }

  fn draw_diagonal(&mut self, line: &Line) {
    let from_y = line.y1.min(line.y2);
    let to_y = line.y1.max(line.y2);
    let from_x = line.x1.min(line.x2);
    let to_x = line.x1.max(line.x2);
    if to_y - from_y == to_x - from_x {
      for delta in 0..=(to_x - from_x) {
        let flag = (line.x2 > line.x1) == (line.y2 > line.y1);
        let y = if flag { from_y + delta } else { to_y - delta };
        self.board[(y * self.width + from_x + delta) as usize] += 1;
      }
    }
  }



  fn overlaps(&self) -> usize {
    self.board.iter().filter(|x| **x > 1).count()
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        write!(f, "{}", self.board[(y * self.width + x) as usize])?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}


fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_05/src/input.txt").unwrap());
  let input = buf.lines().map(|line| line.unwrap().parse().unwrap()).collect::<Vec<Line>>();
  let width = input.iter().map(|line| line.x1).max().max(input.iter().map(|line| line.x2).max()).unwrap() + 1;
  let height = input.iter().map(|line| line.y1).max().max(input.iter().map(|line| line.y2).max()).unwrap() + 1;
  let mut board = Board::new(width, height);
  for line in &input {
    board.draw_straight(line);
  }
  eprintln!("{}", board.overlaps());

  for line in &input {
    board.draw_diagonal(line);
  }
  eprintln!("{}", board.overlaps());
}
