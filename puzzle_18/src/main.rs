use std::io::BufRead;
use std::ops::Add;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Token {
  LBRACKET,
  RBRACKET,
  NUMBER(u32),
}

impl Token {
  fn as_num(&self) -> u32 {
    match self {
      Token::NUMBER(n) => *n,
      _ => unreachable!(),
    }
  }
}

#[derive(Clone)]
struct Line(Vec<Token>);

impl Line {
  fn magnitude(&self) -> u32 {
    let mut pos = 0;
    self.magnitude_impl(&mut pos)
  }
  fn magnitude_impl(&self, pos: &mut usize) -> u32 {
    match self.0[*pos] {
      Token::NUMBER(num) => {
        *pos += 1;
        num
      }
      Token::LBRACKET => {
        *pos += 1;
        let left = self.magnitude_impl(pos);
        let right = self.magnitude_impl(pos);
        *pos += 1;
        3 * left + 2 * right
      }
      _ => panic!(),
    }
  }
}

impl Add for Line {
  type Output = Line;

  fn add(self, rhs: Self) -> Self::Output {
    let mut result = Vec::new();
    result.push(Token::LBRACKET);
    result.extend(self.0.into_iter());
    result.extend(rhs.0.into_iter());
    result.push(Token::RBRACKET);
    reduce(&mut result);
    Line(result)
  }
}

fn parse(s: &str) -> Line {
  let mut result = Vec::new();
  for ch in s.chars() {
    match ch {
      '0'..='9' => result.push(Token::NUMBER(u32::from((ch as u8) - b'0'))),
      '[' => result.push(Token::LBRACKET),
      ']' => result.push(Token::RBRACKET),
      _ => {}
    }
  }
  Line(result)
}

fn reduce(num: &mut Vec<Token>) {
  'outer: loop {
    let mut pos = 0;
    let mut nesting = 0;
    while pos < num.len() {
      if num[pos] == Token::LBRACKET {
        nesting += 1;
        if nesting == 5 {
          let left = num[pos + 1].as_num();
          let right = num[pos + 2].as_num();
          for lpos in (0..pos).rev() {
            if let Token::NUMBER(ref mut l) = &mut num[lpos] {
              *l += left;
              break;
            }
          }
          for rpos in pos + 3..num.len() {
            if let Token::NUMBER(ref mut r) = &mut num[rpos] {
              *r += right;
              break;
            }
          }
          num.remove(pos);
          num.remove(pos);
          num.remove(pos);
          num[pos] = Token::NUMBER(0);
          continue 'outer;
        }
      } else if num[pos] == Token::RBRACKET {
        nesting -= 1;
      }
      pos += 1;
    }

    let mut pos = 0;
    while pos < num.len() {
      match num[pos] {
        Token::NUMBER(val) if val >= 10 => {
          let left = num[pos].as_num() / 2;
          let right = val - left;
          num[pos] = Token::LBRACKET;
          num.insert(pos + 1, Token::NUMBER(left));
          num.insert(pos + 2, Token::NUMBER(right));
          num.insert(pos + 3, Token::RBRACKET);
          continue 'outer;
        }
        _ => {}
      }
      pos += 1;
    }
    break;
  }
}

fn main() {
  let numbers = std::io::BufReader::new(std::fs::File::open("puzzle_18/src/input.txt").unwrap())
    .lines()
    .map(|line| parse(line.as_ref().unwrap()))
    .collect::<Vec<_>>();

  let result = numbers
    .clone()
    .into_iter()
    .reduce(|first, second| first + second)
    .unwrap();

  let mut max = 0;
  for i in 0..numbers.len() {
    for j in 0..numbers.len() {
      if i != j {
        let sum = numbers[i].clone() + numbers[j].clone();
        max = max.max(sum.magnitude());
      }
    }
  }
  eprintln!("{}", result.magnitude());
  eprintln!("{}", max);
}
