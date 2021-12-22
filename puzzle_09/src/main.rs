use std::io::BufRead;

struct Map {
  map: Vec<String>,
}

impl Map {
  fn at(&self, x: isize, y: isize) -> u8 {
    if x < 0 || y < 0 || (x as usize) >= self.map[0].len() || (y as usize) >= self.map.len() {
      return u8::MAX;
    }
    self.map[y as usize].as_bytes()[x as usize] - b'0'
  }

  fn lowest(&self, x: isize, y: isize) -> bool {
    let left = self.at(x - 1, y);
    let right = self.at(x + 1, y);
    let top = self.at(x, y - 1);
    let bottom = self.at(x, y + 1);
    let current = self.at(x, y);
    current < left && current < right && current < top && current < bottom
  }

  fn flows_to(&self, mut x: isize, mut y: isize) -> (isize, isize) {
    loop {
      let current = self.at(x, y);
      if self.at(x - 1, y) < current {
        x -= 1;
      } else if self.at(x + 1, y) < current {
        x += 1;
      } else if self.at(x, y - 1) < current {
        y -= 1;
      } else if self.at(x, y + 1) < current {
        y += 1;
      } else {
        return (x, y);
      }
    }
  }
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_09/src/input.txt").unwrap());
  let map = buf.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
  let map = Map { map };

  let mut total = 0;
  for y in 0..map.map.len() as isize {
    for x in 0..map.map[0].len() as isize {
      if map.lowest(x, y) {
        total += 1 + map.at(x, y) as usize;
      }
    }
  }
  eprintln!("{}", total);

  let mut counts = Vec::new();
  counts.resize(map.map.len() * map.map[0].len(), 0);

  let width = map.map[0].len() as isize;
  for y in 0..map.map.len() as isize {
    for x in 0..width as isize {
      if map.at(x, y) != 9 {
        let (to_x, to_y) = map.flows_to(x, y);
        counts[(to_y * width + to_x) as usize] += 1;
      }
    }
  }
  let mut basins = Vec::new();
  for y in 0..map.map.len() as isize {
    for x in 0..width as isize {
      if counts[(y * width + x) as usize] > 0 {
        basins.push(counts[(y * width + x) as usize]);
      }
    }
  }
  basins.sort();
  basins.reverse();
  eprintln!("{}", basins[0] * basins[1] * basins[2]);
}
