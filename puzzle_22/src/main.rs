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
  fn overlaps(&self, other: &Cube) -> bool {
    (0..3)
      .into_iter()
      .all(|coord| self.coords[coord].0 <= other.coords[coord].1 && self.coords[coord].1 >= other.coords[coord].0)
  }
}

struct World {
  cubes: Vec<Cube>,
}

fn split(cubes: &mut Vec<Cube>, coord: usize, split: i32) {
  for idx in 0..cubes.len() {
    if cubes[idx].coords[coord].0 <= split && split < cubes[idx].coords[coord].1 {
      let mut other = cubes[idx];
      other.coords[coord].0 = split + 1;
      cubes.push(other);
      cubes[idx].coords[coord].1 = split;
    }
  }
}

impl World {
  fn apply(&mut self, instruction: &Instruction) {
    self.remove_intersection(&instruction.cube);
    if instruction.on {
      self.cubes.push(instruction.cube.clone());
    }
  }

  fn remove_intersection(&mut self, cube: &Cube) {
    let mut extra = Vec::new();
    let mut i = 0;
    while i < self.cubes.len() {
      let current = self.cubes[i];
      if current.overlaps(cube) {
        let mut cubes = vec![self.cubes.swap_remove(i)];
        for coord in 0..3 {
          split(&mut cubes, coord, cube.coords[coord].0 - 1);
          split(&mut cubes, coord, cube.coords[coord].1);
        }
        extra.extend(cubes.into_iter().filter(|c| !c.overlaps(cube)));
      } else {
        i += 1;
      }
    }
    self.cubes.extend(extra);
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
