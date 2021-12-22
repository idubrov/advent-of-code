use std::io::BufRead;

#[derive(Debug)]
struct Instruction {
  on: bool,
  cube: Cube,
}

impl std::str::FromStr for Instruction {
  type Err = ();

  fn from_str(mut s: &str) -> Result<Self, Self::Err> {
    let on = if s.starts_with("on ") {
      s = &s["on ".len()..];
      true
    } else {
      s = &s["off ".len()..];
      false
    };
    let mut it = s.split(",");
    let mut coords: [(i32, i32); 3] = [(0, 0); 3];
    for i in 0..3 {
      let part = it.next().unwrap();
      let mut split = part[2..].split("..");
      coords[i] = (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
      );
    }
    Ok(Instruction {
      on,
      cube: Cube { coords },
    })
  }
}

struct Minicube {
  data: Vec<bool>,
}

impl Minicube {
  fn apply(&mut self, inst: &Instruction) {
    for x in inst.cube.coords[0].0.max(-50)..=inst.cube.coords[0].1.min(50) {
      for y in inst.cube.coords[1].0.max(-50)..=inst.cube.coords[1].1.min(50) {
        for z in inst.cube.coords[2].0.max(-50)..=inst.cube.coords[2].1.min(50) {
          let offset = (z + 50) * 101 * 101 + (y + 50) * 101 + (x + 50);
          self.data[offset as usize] = inst.on;
        }
      }
    }
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Cube {
  coords: [(i32, i32); 3],
}

impl Cube {
  fn volume(&self) -> usize {
    self.coords.iter().map(|c| (c.1 + 1 - c.0) as usize).product()
  }
}

struct World {
  cubes: Vec<Cube>,
}

impl World {
  fn apply(&mut self, instruction: &Instruction) {
    for coord in 0..3 {
      self.split(coord, instruction.cube.coords[coord].0 - 1);
      self.split(coord, instruction.cube.coords[coord].1)
    }
    self.remove_contained(&instruction.cube);
    if instruction.on {
      self.cubes.push(instruction.cube.clone());
    }
  }

  fn split(&mut self, coord: usize, split: i32) {
    let mut extra = Vec::new();
    for cube in &mut self.cubes {
      if cube.coords[coord].0 <= split && split < cube.coords[coord].1 {
        let mut other = cube.coords;
        other[coord].0 = split + 1;
        extra.push(Cube { coords: other });
        cube.coords[coord].1 = split;
      }
    }
    self.cubes.extend(extra.into_iter());
  }

  fn remove_contained(&mut self, cube: &Cube) {
    let mut i = 0;
    while i < self.cubes.len() {
      if (0..3).into_iter().all(|coord| {
        self.cubes[i].coords[coord].0 >= cube.coords[coord].0 && self.cubes[i].coords[coord].1 <= cube.coords[coord].1
      }) {
        self.cubes.swap_remove(i);
      } else {
        i += 1;
      }
    }
  }

  fn volume(&self) -> usize {
    self.cubes.iter().map(|c| c.volume()).sum()
  }
}

fn main() {
  let input = std::io::BufReader::new(std::fs::File::open("puzzle_22/src/input.txt").unwrap()).lines();
  let instructions = input
    .map(|line| line.unwrap().parse::<Instruction>().unwrap())
    .collect::<Vec<_>>();

  let mut data = Vec::new();
  data.resize(101 * 101 * 101, false);
  let mut minicube = Minicube { data };
  for inst in &instructions {
    minicube.apply(inst);
  }
  let count = minicube.data.iter().filter(|v| **v).count();
  eprintln!("{}", count);

  let mut world = World { cubes: Vec::new() };
  for inst in &instructions {
    world.apply(&inst);
  }
  eprintln!("{:?}", world.volume());
}
