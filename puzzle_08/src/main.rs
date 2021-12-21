use std::io::BufRead;


#[derive(Debug)]
struct Input {
  pattern: Vec<String>,
  digits: Vec<String>,
}

const DIGITS: [i32; 10] = [
  0b1110111,
  0b100100,
  0b1011101,
  0b1101101,
  0b101110,
  0b1101011,
  0b1111011,
  0b100101,
  0b1111111,
  0b1101111,
];

fn digit_of(input: &str, permut: &[i32; 7]) -> Option<usize> {
  let mut digit = 0;
  for ch in input.chars() {
    digit |= 1 << permut[(ch as u8 - b'a') as usize];
  }
  DIGITS.iter().position(|d| *d == digit)
}

impl std::str::FromStr for Input {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut it = s.split("|");
    let (left, right) = (it.next().unwrap().trim(), it.next().unwrap().trim());
    let pattern = left.split_whitespace().map(|x| x.trim().to_owned()).collect::<Vec<String>>();
    let digits = right.split_whitespace().map(|x| x.trim().to_owned()).collect::<Vec<String>>();
    Ok(Input {
      pattern, digits,
    })
  }
}

type Permutation = [i32; 7];
fn generate(k: usize, current: &mut Permutation, permuts: &mut Vec<Permutation>) {
  if k == 1 {
    permuts.push(*current);
    return;
  }
  generate(k - 1, current, permuts);
  for i in 0..k - 1 {
    if k % 2 == 0 {
      current.swap(i, k - 1);
    } else {
      current.swap(0, k - 1);
    }
    generate(k - 1, current, permuts);
  }
}

fn main() {

  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_08/src/input.txt").unwrap());
  let inputs = buf.lines().map(|line| line.unwrap().parse().unwrap()).collect::<Vec<Input>>();
  let mut total = 0;
  for input in &inputs {
    for dig in &input.digits {
      if dig.len() == 2 || dig.len() == 4 || dig.len() == 3 || dig.len() == 7 {
        total += 1;
      }
    }
  }
  eprintln!("{}", total);

  let mut total = 0;
  let mut swaps = [0, 1, 2, 3, 4, 5, 6];
  let mut permuts = Vec::new();
  generate(swaps.len(), &mut swaps, &mut permuts);
  for input in &inputs {
    let permut = permuts
      .iter()
      .find(|permut| input.pattern.iter().all(|p| digit_of(p, permut).is_some()))
      .unwrap();

    let mut value = 0;
    for d in &input.digits {
      let digit = digit_of(d, permut).unwrap();
      value *= 10;
      value += digit;
    }
    total += value;
  }
  eprintln!("{}", total);
}
