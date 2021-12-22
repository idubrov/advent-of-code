use std::io::BufRead;

struct Map {
  map: Vec<Vec<u8>>,
}


impl Map {
  fn walk(&self, lowest: &mut Vec<usize>, visit: &mut Vec<(isize, isize)>) {
    while !visit.is_empty() {
      let (x, y) = visit.remove(0);
      let width = self.map[0].len() as isize;
      let height = self.map.len() as isize;
      let new_cost = lowest[(y * width + x) as usize] + usize::from(self.map[y as usize][x as usize]);
      if x > 0 && lowest[(y * width + x - 1) as usize] > new_cost {
        lowest[(y * width + x - 1) as usize] = new_cost;
        visit.push((x - 1, y));
      }
      if x < width - 1 && lowest[(y * width + x + 1) as usize] > new_cost {
        lowest[(y * width + x + 1) as usize] = new_cost;
        visit.push((x + 1, y));
      }
      if y > 0 && lowest[((y - 1) * width + x) as usize] > new_cost {
        lowest[((y - 1) * width + x) as usize] = new_cost;
        visit.push((x, y - 1));
      }
      if y < height - 1 && lowest[((y + 1) * width + x) as usize] > new_cost {
        lowest[((y + 1) * width + x) as usize] = new_cost;
        visit.push((x, y + 1));
      }
    }
  }

  fn shortest(&self) -> usize {
    let width = self.map[0].len();
    let height = self.map.len();
    let mut lowest = Vec::new();
    lowest.resize(width * height, usize::MAX);
    let mut visit = Vec::new();
    *lowest.last_mut().unwrap() = 0;
    visit.push(((width - 1) as isize, (height - 1) as isize));
    self.walk(&mut lowest, &mut visit);
    lowest[0]
  }
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_15/src/input.txt").unwrap());
  let map = buf
    .lines()
    .map(|line| {
      let mut line = line.unwrap().into_bytes();
      line.iter_mut().for_each(|elem| *elem -= b'0');
      line
    })
    .collect::<Vec<_>>();
  let map = Map {
    map,
  };
  eprintln!("{}", map.shortest());


  // generate larger map
  let width = map.map[0].len();
  let height = map.map.len();
  let mut mega = Vec::new();
  for yy in 0..5 {
    for y in 0..height {
      let mut line = Vec::with_capacity(width * 5);
      for xx in 0..5 {
        for x in 0..width {
          line.push(((map.map[y][x] - 1) + xx + yy) % 9 + 1);
        }
      }
      mega.push(line);
    }
  }
  let mega = Map {
   map: mega,
  };
  eprintln!("{}", mega.shortest());
}
