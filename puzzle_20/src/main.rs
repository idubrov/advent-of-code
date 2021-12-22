use std::collections::HashSet;
use std::io::BufRead;

struct Image {
  x: (i32, i32),
  y: (i32, i32),
  bits: HashSet<(i32, i32)>,
  rest: char,
}

impl Image {
  fn bit(&self, x: i32, y: i32) -> usize {
    if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
      if self.rest == '#' {
        1
      } else {
        0
      }
    } else {
      if self.bits.contains(&(x, y)) {
        1
      } else {
        0
      }
    }
  }
}

fn enhance(image: &Image, tx: &str) -> Image {
  let x0 = image.bits.iter().map(|(x, _)| x).min().unwrap() - 1;
  let x1 = image.bits.iter().map(|(x, _)| x).max().unwrap() + 1;
  let y0 = image.bits.iter().map(|(_, y)| y).min().unwrap() - 1;
  let y1 = image.bits.iter().map(|(_, y)| y).max().unwrap() + 1;
  let mut result = HashSet::new();
  for y in y0..=y1 {
    for x in x0..=x1 {
      let index = (image.bit(x - 1, y - 1) << 8)
        + (image.bit(x, y - 1) << 7)
        + (image.bit(x + 1, y - 1) << 6)
        + (image.bit(x - 1, y) << 5)
        + (image.bit(x, y) << 4)
        + (image.bit(x + 1, y) << 3)
        + (image.bit(x - 1, y + 1) << 2)
        + (image.bit(x, y + 1) << 1)
        + (image.bit(x + 1, y + 1) << 0);

      if tx.as_bytes()[index] == b'#' {
        result.insert((x, y));
      }
    }
  }

  Image {
    x: (x0, x1),
    y: (y0, y1),
    bits: result,
    rest: if image.rest == '#' {
      tx.as_bytes()[511] as char
    } else {
      tx.as_bytes()[0] as char
    },
  }
}

fn main() {
  let mut input = std::io::BufReader::new(std::fs::File::open("puzzle_20/src/input.txt").unwrap()).lines();
  let tx = input.next().unwrap().unwrap();
  assert_eq!(512, tx.len());
  assert!(input.next().unwrap().unwrap().is_empty());
  let mut bits = HashSet::new();
  let mut x = 0;
  let mut y = 0;
  while let Some(Ok(line)) = input.next() {
    x = 0;
    for ch in line.chars() {
      if ch == '#' {
        bits.insert((x, y));
      }
      x += 1;
    }
    y += 1;
  }
  let mut image = Image {
    x: (0, x),
    y: (0, y),
    bits,
    rest: '.',
  };

  for _ in 0..2 {
    image = enhance(&image, &tx);
  }
  eprintln!("{}", image.bits.len());

  for _ in 0..48 {
    image = enhance(&image, &tx);
  }
  eprintln!("{}", image.bits.len());
}
