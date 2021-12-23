// 8, 9 | v0,1v | 10 | v2,3v | 11 | v4,5v | 12 | v6,7v | 13 14 |
// 0, 1 | r0 | 2 | r1 | 3 | r2 | 4 | r3 | 5 6
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Room {
  room: u8,
  depth: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Hall(u8);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Pos {
  R(Room),
  H(Hall),
}

impl Pos {
  fn as_room(&self) -> Option<Room> {
    match self {
      Pos::R(room) => Some(*room),
      _ => None,
    }
  }

  fn is_room(&self, room: u8) -> bool {
    match self {
      Pos::R(r) if r.room == room => true,
      _ => false,
    }
  }
}

fn room_hall(from: Pos, to: Pos) -> (Room, Hall) {
  match (from, to) {
    (Pos::R(r), Pos::H(h)) => (r, h),
    (Pos::H(h), Pos::R(r)) => (r, h),
    _ => panic!(),
  }
}

fn distance(from: Pos, to: Pos) -> u8 {
  let (from, to) = room_hall(from, to);
  let coords = [0, 1, 3, 5, 7, 9, 10];
  let first = from.room * 2 + 2;
  let second = coords[usize::from(to.0)];
  let horizontal = if first < second { second - first } else { first - second };
  horizontal + 1 + from.depth
}

const DEPTH: u8 = 2;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State([Pos; 4 * DEPTH as usize]);

const COST: [usize; 4] = [1, 10, 100, 1000];

impl State {
  fn is_completed(&self) -> bool {
    (0..4 * DEPTH).into_iter().all(|idx| (self.0[usize::from(idx)].is_room(idx / DEPTH)))
  }

  fn has_path(&self, from: Pos, to: Pos) -> bool {
    let (from, to) = room_hall(from, to);
    let path = match (from.room, to.0) {
      (0, 0) => [0, 1].as_slice(),
      (0, 1) => &[1],
      (0, 2) => &[2],
      (0, 3) => &[2, 3],
      (0, 4) => &[2, 3, 4],
      (0, 5) => &[2, 3, 4, 5],
      (0, 6) => &[2, 3, 4, 5, 6],

      (1, 0) => &[0, 1, 2],
      (1, 1) => &[1, 2],
      (1, 2) => &[2],
      (1, 3) => &[3],
      (1, 4) => &[3, 4],
      (1, 5) => &[3, 4, 5],
      (1, 6) => &[3, 4, 5, 6],

      (2, 0) => &[0, 1, 2, 3],
      (2, 1) => &[1, 2, 3],
      (2, 2) => &[2, 3],
      (2, 3) => &[3],
      (2, 4) => &[4],
      (2, 5) => &[4, 5],
      (2, 6) => &[4, 5, 6],

      (3, 0) => &[0, 1, 2, 3, 4],
      (3, 1) => &[1, 2, 3, 4],
      (3, 2) => &[2, 3, 4],
      (3, 3) => &[3, 4],
      (3, 4) => &[4],
      (3, 5) => &[5],
      (3, 6) => &[5, 6],
      (from, to) => panic!("{} {}", from, to),
    };
    path.iter().all(|p| !self.0.contains(&Pos::H(Hall(*p))))
  }

  fn solve(
    mut self,
    cost: usize,
    instr: &mut Vec<(Pos, Pos)>,
    visited: &mut HashMap<State, Option<usize>>,
  ) -> Option<usize> {
    if let Some(cost) = visited.get(&self) {
      return *cost;
    }
    if self.is_completed() {
      return Some(0);
    }
    let mut min_cost = None;
    for idx in 0..DEPTH * 4 {
      let my_room = idx / DEPTH;
      let src = self.0[idx as usize];
      if src.is_room(my_room) {
        //let d = src.as_room().unwrap().depth;
        //((d+1)..DEPTH).into_iter()
        if src.as_room().unwrap().depth == 0 {
          let neighboor = self.0[(idx ^ 1) as usize];
          if neighboor.is_room(my_room) {
            continue;
          }
        } else {
          continue;
        }
      }
      if let Pos::R(r) = src {
        // blocked from the top
        if r.depth == 1 && self.0.contains(&Pos::R(Room { room: r.room, depth: 0 })) {
          continue;
        }
      }

      if src.as_room().is_some() {
        for hall in 0..7 {
          self.try_walk(idx, Pos::H(Hall(hall)), cost, instr, visited, &mut min_cost);
        }
      } else {
        let bottom = Pos::R(Room {
          room: my_room,
          depth: 1,
        });
        if let Some(occ) = self.0.iter().position(|x| x == &bottom) {
          if ((occ as u8) / DEPTH) != my_room {
            continue;
          }
          if self.0.contains(&Pos::R(Room {
            room: my_room,
            depth: 0,
          })) {
            continue;
          }
          // check if taken by my kind, then take top spot
          let target = Pos::R(Room {
            room: my_room,
            depth: 0,
          });
          self.try_walk(idx, target, cost, instr, visited, &mut min_cost);
        } else {
          // take bottom spot
          let target = Pos::R(Room {
            room: my_room,
            depth: 1,
          });
          self.try_walk(idx, target, cost, instr, visited, &mut min_cost);
        }
      }
    }
    visited.insert(self, min_cost);
    min_cost
  }

  fn try_walk(
    &mut self,
    idx: u8,
    tgt: Pos,
    cost: usize,
    instr: &mut Vec<(Pos, Pos)>,
    visited: &mut HashMap<State, Option<usize>>,
    min_cost: &mut Option<usize>,
  ) {
    let src = self.0[idx as usize];
    instr.push((src, tgt));
    // temporary remove for has_path!
    self.0[idx as usize] = Pos::H(Hall(255));
    if self.has_path(src, tgt) {
      let delta_cost = usize::from(distance(src, tgt)) * COST[(idx / 2) as usize];
      self.0[idx as usize] = tgt;
      let alt_cost = self
        .solve(cost + delta_cost, instr, visited).map(|cost| cost + delta_cost);
      if min_cost.is_none() {
        *min_cost = alt_cost;
      } else if alt_cost.is_some() && alt_cost.unwrap() < min_cost.unwrap() {
        *min_cost = alt_cost;
      }
    }
    self.0[idx as usize] = src;
    instr.pop();
  }
}

fn main() {
  // #############
  // #...........#
  // ###B#C#B#D### 0  2  4  6
  //   #A#D#C#A#   1  3  5  7
  //   #########
  let state = State([
    Pos::R(Room { room: 0, depth: 1 }),
    Pos::R(Room { room: 3, depth: 1 }),
    Pos::R(Room { room: 0, depth: 0 }),
    Pos::R(Room { room: 2, depth: 0 }),
    Pos::R(Room { room: 1, depth: 0 }),
    Pos::R(Room { room: 2, depth: 1 }),
    Pos::R(Room { room: 1, depth: 1 }),
    Pos::R(Room { room: 3, depth: 0 }),
  ]);

  // #############
  // #...........#
  // ###B#B#C#D### 0  2  4  6
  //   #D#A#A#C#   1  3  5  7
  //   #########
  //let state = State([3, 5, 0, 2, 4, 7, 1, 6]);
  let cost = state.solve(0, &mut Vec::new(), &mut HashMap::new()).unwrap();
  println!("{}", cost);
}
