use std::collections::HashSet;
use std::io::BufRead;

pub enum Transform {
  X_Y_Z,
  X_NZ_Y,
  X_NY_NZ,
  X_Z_NY,

  NX_NZ_NY,
  NX_Y_NZ,
  NX_Z_Y,
  NX_NY_Z,

  Y_Z_X,
  Y_NX_Z,
  Y_NZ_NX,
  Y_X_NZ,

  NY_NX_NZ,
  NY_Z_NX,
  NY_X_Z,
  NY_NZ_X,

  Z_X_Y,
  Z_NY_X,
  Z_NX_NY,
  Z_Y_NX,

  NZ_NY_NX,
  NZ_X_NY,
  NZ_Y_X,
  NZ_NX_Y,
}

const ALL_TRANSFORMS: [Transform; 24] = [
  Transform::X_Y_Z,
  Transform::X_NZ_Y,
  Transform::X_NY_NZ,
  Transform::X_Z_NY,
  Transform::NX_NZ_NY,
  Transform::NX_Y_NZ,
  Transform::NX_Z_Y,
  Transform::NX_NY_Z,
  Transform::Y_Z_X,
  Transform::Y_NX_Z,
  Transform::Y_NZ_NX,
  Transform::Y_X_NZ,
  Transform::NY_NX_NZ,
  Transform::NY_Z_NX,
  Transform::NY_X_Z,
  Transform::NY_NZ_X,
  Transform::Z_X_Y,
  Transform::Z_NY_X,
  Transform::Z_NX_NY,
  Transform::Z_Y_NX,
  Transform::NZ_NY_NX,
  Transform::NZ_X_NY,
  Transform::NZ_Y_X,
  Transform::NZ_NX_Y,
];

impl Transform {
  fn transform(&self, pos: &Pos) -> Pos {
    let Pos { x, y, z } = *pos;
    use Transform::*;
    let (i, j, k) = match self {
      X_Y_Z => (x, y, z),
      X_NZ_Y => (x, -z, y),
      X_NY_NZ => (x, -y, -z),
      X_Z_NY => (x, z, -y),

      NX_NZ_NY => (-x, -z, -y),
      NX_Y_NZ => (-x, y, -z),
      NX_Z_Y => (-x, z, y),
      NX_NY_Z => (-x, -y, z),

      Y_Z_X => (y, z, x),
      Y_NX_Z => (y, -x, z),
      Y_NZ_NX => (y, -z, -x),
      Y_X_NZ => (y, x, -z),

      NY_NX_NZ => (-y, -x, -z),
      NY_Z_NX => (-y, z, -x),
      NY_X_Z => (-y, x, z),
      NY_NZ_X => (-y, -z, x),

      Z_X_Y => (z, x, y),
      Z_NY_X => (z, -y, x),
      Z_NX_NY => (z, -x, -y),
      Z_Y_NX => (z, y, -x),

      NZ_NY_NX => (-z, -y, -x),
      NZ_X_NY => (-z, x, -y),
      NZ_Y_X => (-z, y, x),
      NZ_NX_Y => (-z, -x, y),
    };
    Pos { x: i, y: j, z: k }
  }
}

impl std::ops::Add for Pos {
  type Output = Pos;

  fn add(self, rhs: Self) -> Self::Output {
    Pos {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    }
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
  x: i32,
  y: i32,
  z: i32,
}

#[derive(Debug)]
struct Scanner {
  beacons: Vec<Pos>,
}

struct World {
  scanner: Vec<PlacedScanner>,
}

struct PlacedScanner {
  offset: Pos,
  beacons: HashSet<Pos>,
}

fn main() {
  let input = std::io::BufReader::new(std::fs::File::open("puzzle_19/src/input.txt").unwrap()).lines();
  let mut scanners = Vec::new();
  let mut beacons = Vec::new();
  for line in input {
    let line = line.unwrap();
    if line.trim().is_empty() {
    } else if line.starts_with("--- scanner") {
      if !beacons.is_empty() {
        scanners.push(Scanner { beacons });
      }
      beacons = Vec::new();
    } else {
      let mut it = line.split(",");
      let (x, y, z) = (
        it.next().unwrap().parse().unwrap(),
        it.next().unwrap().parse().unwrap(),
        it.next().unwrap().parse().unwrap(),
      );
      beacons.push(Pos { x, y, z });
    }
  }
  if !beacons.is_empty() {
    scanners.push(Scanner { beacons });
  }

  let first = scanners.remove(0);
  let mut world = World {
    scanner: vec![PlacedScanner {
      offset: Pos { x: 0, y: 0, z: 0 },
      beacons: first.beacons.into_iter().collect(),
    }],
  };

  while !scanners.is_empty() {
    let mut found = None;
    'iter: for scanner in 0..scanners.len() {
      for tx in ALL_TRANSFORMS {
        for tgt_scanner in &world.scanner {
          for tgt_beacon in &tgt_scanner.beacons {
            for src_beacon in &scanners[scanner].beacons {
              let temp = tx.transform(src_beacon);
              let offset = Pos {
                x: tgt_beacon.x - temp.x,
                y: tgt_beacon.y - temp.y,
                z: tgt_beacon.z - temp.z,
              };

              assert_eq!(tx.transform(src_beacon) + offset, *tgt_beacon);
              let count = scanners[scanner]
                .beacons
                .iter()
                .filter(|b| tgt_scanner.beacons.contains(&(tx.transform(b) + offset)))
                .count();
              if count >= 12 {
                found = Some(scanner);
                world.scanner.push(PlacedScanner {
                  offset,
                  beacons: scanners[scanner]
                    .beacons
                    .iter()
                    .map(|b| tx.transform(b) + offset)
                    .collect(),
                });
                break 'iter;
              }
            }
          }
        }
      }
    }
    scanners.remove(found.unwrap());
  }

  let mut max_dist = 0;
  for first in &world.scanner {
    for second in &world.scanner {
      let dist = (first.offset.x - second.offset.x).abs()
        + (first.offset.y - second.offset.y).abs()
        + (first.offset.z - second.offset.z).abs();
      max_dist = max_dist.max(dist);
    }
  }

  let mut result = HashSet::new();
  for chunk in world.scanner {
    result.extend(chunk.beacons);
  }
  eprintln!("{:?}", result.len());

  eprintln!("{}", max_dist);
}
