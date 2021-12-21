use std::io::BufRead;

struct Book {
  width: usize,
  stride: usize,
  height: usize,
  map: Vec<bool>,
}

impl Book {
  fn fold_y(&mut self, y: usize) {
    for x in 0..self.width {
      for delta in 0..y {
        self.map[(y - 1 - delta) * self.stride + x] |= self.map.get((y + 1 + delta) * self.stride + x).copied().unwrap_or(false);
      }
    }
    self.height = y;
  }

  fn fold_x(&mut self, x: usize) {
    for y in 0..self.height {
      for delta in 0..x {
        self.map[y * self.stride + (x - 1 - delta)] |= self.map.get(y * self.stride + (x + 1 + delta)).copied().unwrap_or(false);
      }
    }
    self.width = x;
  }

  fn count(&self) -> usize {
    let mut count = 0;
    for y in 0..self.height {
      for x in 0..self.width {
        if self.map[y * self.stride + x] {
          count += 1;
        }
      }
    }
    count
  }
}

impl std::fmt::Display for Book {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for y in 0..self.height {
      for x in 0..self.width {
        if self.map[y * self.stride + x] {
          write!(f, "*")?;
        } else {
          write!(f, "_")?;
        }
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_13/src/input.txt").unwrap());
  let mut lines = buf.lines();
  let mut positions: Vec<(usize, usize)> = Vec::new();
  for line in &mut lines {
    let line = line.unwrap();
    if line.is_empty() {
      break;
    }
    let mut it = line.split(",");
    positions.push((it.next().unwrap().parse().unwrap(), it.next().unwrap().parse().unwrap()));
  }
  let width = positions.iter().map(|(x, _)| x).max().unwrap() + 1;
  let height = positions.iter().map(|(_, y)| y).max().unwrap() + 1;
  let mut map = Vec::new();
  map.resize(width * height, false);
  let mut map = Book {
    width,
    stride: width,
    height,
    map,
  };
  for (x, y) in &positions {
    map.map[y * map.stride + x] = true;
  }

  let mut first = true;
  for line in lines {
    let line = line.unwrap();
    if line.starts_with("fold along y=") {
      map.fold_y(line["fold along y=".len()..].parse().unwrap());
    } else {
      map.fold_x(line["fold along x=".len()..].parse().unwrap());
    }
    if first {
      eprintln!("{}", map.count());
      first = false;
    }
  }

  // PERCGJPB
  eprintln!("{}", map);
}
