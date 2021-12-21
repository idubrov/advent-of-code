use std::io::BufRead;

struct Map {
  map: Vec<i32>,
  width: usize,
  height: usize,
  flashes: usize,
}

impl Map {
  fn tick(&mut self) {
    self.map.iter_mut().for_each(|x| *x += 1);
    for y in 0..self.height {
      for x in 0..self.width {
        self.check_flash(x as isize, y as isize);
      }
    }
  }

  fn at(&mut self, x: isize, y: isize) -> &mut i32 {
    &mut self.map[(y as usize) * self.width + x as usize]
  }

  fn bump(&mut self, x: isize, y: isize) {
    if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
      return;
    }
    if *self.at(x, y) == 0 {
      return;
    }
    *self.at(x, y) += 1;
    self.check_flash(x, y);
  }

  fn check_flash(&mut self, x: isize, y: isize) {
    if *self.at(x, y) > 9 {
      self.flashes += 1;
      *self.at(x, y) = 0;
      self.bump(x - 1, y);
      self.bump(x + 1, y);
      self.bump(x, y - 1);
      self.bump(x, y + 1);
      self.bump(x - 1, y - 1);
      self.bump(x - 1, y + 1);
      self.bump(x + 1, y - 1);
      self.bump(x + 1, y + 1);
    }
  }
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_11/src/input.txt").unwrap());
  let mut map = Vec::new();
  let mut width = 0;
  for line in buf.lines() {
    width = line.as_ref().unwrap().trim().len();
    map.extend(line.unwrap().as_bytes().iter().map(|ch| i32::from(ch - b'0')));
  }
  let mut map = Map {
    width,
    height: map.len() / width,
    map,
    flashes: 0,
  };
  let mut total = 0;
  let mut all_flash = 0;
  for step in 1.. {
    map.tick();
    total += map.flashes;
    if all_flash == 0 && map.flashes == map.width * map.height {
      all_flash = step;
    }
    map.flashes = 0;
    if step == 100 {
      println!("{}", total);
    }

    if step >= 100 && all_flash != 0 {
      break;
    }
  }
  eprintln!("{}", all_flash);
}
