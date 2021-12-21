use std::io::BufRead;

fn is_opening(ch: u8) -> bool {
  ch == b'(' || ch == b'[' || ch == b'{' || ch == b'<'
}

fn score_of(ch: u8) -> i32 {
  match ch {
    b')' => 3,
    b']' => 57,
    b'}' => 1197,
    b'>' => 25137,
    _ => panic!(),
  }
}

fn complete_score_of(ch: u8) -> u128 {
  match ch {
    b'(' => 1,
    b'[' => 2,
    b'{' => 3,
    b'<' => 4,
    _ => panic!(),
  }
}

fn is_match(left: u8, right: u8) -> bool {
  match (left, right) {
    (b'(', b')') => true,
    (b'[', b']') => true,
    (b'{', b'}') => true,
    (b'<', b'>') => true,
    _ => false,
  }
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_10/src/input.txt").unwrap());
  let program = buf
    .lines()
    .collect::<Result<Vec<String>, _>>().unwrap();

  let mut score = 0;
  let mut complete_scores = Vec::new();
  for line in &program {
    let mut current = line.as_bytes().to_owned();
    let mut index = 0;
    while index < current.len() {
      let c = current[index];
      if is_opening(c) {
        index += 1;
      } else {
        if index == 0 || !is_match(current[index - 1], c) {
          score += score_of(c);
          break;
        } else {
          index -= 1;
          current.remove(index);
          current.remove(index);
        }
      }
    }

    if index == current.len() {
      complete_scores.push(current.iter().rev().fold(0u128, |acc, x| acc * 5 + complete_score_of(*x)));
    }
  }
  eprintln!("{}", score);
  complete_scores.sort();
  eprintln!("{}", complete_scores[complete_scores.len() / 2]);
}
