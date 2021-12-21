use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
struct Board {
  board: Vec<Vec<(i32, bool)>>,
}

impl Board {
  fn is_win(&self) -> bool {
    for row in 0..5 {
      if self.board[row].iter().all(|x| x.1) {
        return true;
      }
    }
    for col in 0..5 {
      if self.board.iter().all(|x| x[col].1) {
        return true;
      }
    }
    false
  }

  fn update(&mut self, value: i32) {
    for row in 0..5 {
      for col in 0..5 {
        if self.board[row][col].0 == value {
          self.board[row][col].1 = true;
        }
      }
    }
  }

  fn score(&self) -> i32 {
    let mut score = 0;
    for row in &self.board {
      for (val, flag) in row {
        if !flag {
          score += val;
        }
      }
    }
    score
  }
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_04/src/input.txt").unwrap());
  let mut input = buf.lines();
  let numbers = input
    .next()
    .unwrap()
    .unwrap()
    .split(",")
    .map(|x| i32::from_str(x).unwrap())
    .collect::<Vec<_>>();
  eprintln!("{:?}", numbers);
  let mut boards = Vec::new();
  loop {
    let line = input.next();
    if line.is_none() {
      break;
    }
    assert!(line.unwrap().unwrap().trim().is_empty());
    let mut rows = Vec::new();
    for _ in 0..5 {
      rows.push(
        input
          .next()
          .unwrap()
          .unwrap()
          .split_whitespace()
          .map(|x| (i32::from_str(x).unwrap(), false))
          .collect::<Vec<_>>(),
      );
    }
    boards.push(Board { board: rows })
  }

  let mut found_first = false;
  for value in numbers {
    let last = boards.iter().enumerate().find(|(_, b)| !b.is_win()).map(|x| x.0);
    boards.iter_mut().for_each(|board| board.update(value));
    if let Some(won) = boards.iter().filter(|b| b.is_win()).next() {
      if !found_first {
        found_first = true;
        eprintln!("{}", value * won.score());
      }
    }

    if boards.iter().filter(|b| b.is_win()).count() == boards.len() {
      eprintln!("{}", value * boards[last.unwrap()].score());
      break;
    }
  }
}
