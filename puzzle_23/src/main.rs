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

  fn room(room: u8, depth: u8) -> Pos {
    Pos::R(Room { room, depth })
  }
}

fn distance(room: Room, hall: Hall) -> u8 {
  let coords = [0, 1, 3, 5, 7, 9, 10];
  let first = room.room * 2 + 2;
  let second = coords[usize::from(hall.0)];
  let horizontal = if first < second { second - first } else { first - second };
  horizontal + 1 + room.depth
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State<const RS: usize>([Pos; RS]);

const COST: [usize; 4] = [1, 10, 100, 1000];

impl<const RS: usize> State<RS> {
  const ROOMS: u8 = RS as u8;
  const DEPTH: u8 = Self::ROOMS / 4;

  fn room_for(idx: u8) -> u8 {
    4 * idx / Self::ROOMS
  }

  fn is_completed(&self) -> bool {
    (0..Self::ROOMS)
      .into_iter()
      .all(|idx| (self.0[usize::from(idx)].is_room(Self::room_for(idx as u8))))
  }

  fn path_clear(&self, room: Room, hall: Hall) -> bool {
    let path = match (room.room, hall.0) {
      (0, 0) => [1].as_slice(),
      (0, 1) => &[],
      (0, 2) => &[],
      (0, 3) => &[2],
      (0, 4) => &[2, 3],
      (0, 5) => &[2, 3, 4],
      (0, 6) => &[2, 3, 4, 5],

      (1, 0) => &[1, 2],
      (1, 1) => &[2],
      (1, 2) => &[],
      (1, 3) => &[],
      (1, 4) => &[3],
      (1, 5) => &[3, 4],
      (1, 6) => &[3, 4, 5],

      (2, 0) => &[1, 2, 3],
      (2, 1) => &[2, 3],
      (2, 2) => &[3],
      (2, 3) => &[],
      (2, 4) => &[],
      (2, 5) => &[4],
      (2, 6) => &[4, 5],

      (3, 0) => &[1, 2, 3, 4],
      (3, 1) => &[2, 3, 4],
      (3, 2) => &[3, 4],
      (3, 3) => &[4],
      (3, 4) => &[],
      (3, 5) => &[],
      (3, 6) => &[5],
      (from, to) => panic!("{} {}", from, to),
    };
    path.iter().all(|p| !self.0.contains(&Pos::H(Hall(*p))))
  }

  fn room_filled(&self, room: u8, from_depth: u8) -> bool {
    (from_depth + 1..Self::DEPTH)
      .all(|depth| Self::room_for(self.0.iter().position(|x| x == &Pos::room(room, depth)).unwrap() as u8) == room)
  }

  fn find_room(&self, room: u8) -> Option<Room> {
    for depth in (0..Self::DEPTH).rev() {
      let candidate = Room { room, depth };
      match self.0.iter().position(|x| x == &Pos::R(candidate)) {
        Some(occupant) => {
          if (Self::room_for(occupant as u8)) != room {
            return None;
          }
        }
        None => return Some(candidate),
      }
    }
    None
  }

  fn solve(mut self, visited: &mut HashMap<Self, Option<usize>>) -> Option<usize> {
    if let Some(cost) = visited.get(&self) {
      return *cost;
    }
    if self.is_completed() {
      return Some(0);
    }
    let mut min_cost = None;
    for idx in 0..Self::ROOMS {
      let my_room = Self::room_for(idx);
      let src = self.0[idx as usize];
      if src.is_room(my_room) && self.room_filled(my_room, src.as_room().unwrap().depth) {
        continue;
      }
      if let Pos::R(r) = src {
        if (0..r.depth).any(|d| self.0.contains(&Pos::room(r.room, d))) {
          continue;
        }
      }

      match src {
        Pos::R(room) => {
          for hall in 0..7 {
            if self.0.iter().any(|p| p == &Pos::H(Hall(hall))) {
              continue;
            }
            self.0[idx as usize] = Pos::H(Hall(hall));
            if self.path_clear(room, Hall(hall)) {
              let delta_cost = usize::from(distance(room, Hall(hall))) * COST[usize::from(Self::room_for(idx))];
              let alt_cost = self.solve(visited).map(|cost| cost + delta_cost);
              min_cost = min_cost.or(alt_cost).min(alt_cost.or(min_cost));
            }
          }
        }
        Pos::H(hall) => {
          if let Some(room) = self.find_room(my_room) {
            self.0[idx as usize] = Pos::R(room);
            if self.path_clear(room, hall) {
              let delta_cost = usize::from(distance(room, hall)) * COST[usize::from(Self::room_for(idx))];
              let alt_cost = self.solve(visited).map(|cost| cost + delta_cost);
              min_cost = min_cost.or(alt_cost).min(alt_cost.or(min_cost));
            }
          }
        }
      }
      self.0[idx as usize] = src;
    }
    visited.insert(self, min_cost);
    min_cost
  }
}

fn main() {
  // #############
  // #...........#
  // ###B#C#B#D###
  //   #A#D#C#A#
  //   #########
  let state = State([
    Pos::room(0, 1),
    Pos::room(3, 1),
    Pos::room(0, 0),
    Pos::room(2, 0),
    Pos::room(1, 0),
    Pos::room(2, 1),
    Pos::room(1, 1),
    Pos::room(3, 0),
  ]);
  println!("sample {}", state.solve(&mut HashMap::new()).unwrap());

  // #############
  // #...........#
  // ###B#C#B#D### 0  2  4  6
  //   #D#C#B#A#
  //   #D#B#A#C#
  //   #A#D#C#A#   1  3  5  7
  //   #########
  let state = State([
    Pos::room(0, 3),
    Pos::room(2, 2),
    Pos::room(3, 1),
    Pos::room(3, 3),
    Pos::room(0, 0),
    Pos::room(1, 2),
    Pos::room(2, 0),
    Pos::room(2, 1),
    Pos::room(1, 0),
    Pos::room(1, 1),
    Pos::room(2, 3),
    Pos::room(3, 2),
    Pos::room(0, 1),
    Pos::room(0, 2),
    Pos::room(1, 3),
    Pos::room(3, 0),
  ]);
  println!("sample real {}", state.solve(&mut HashMap::new()).unwrap());

  // #############
  // #...........#
  // ###B#B#C#D### 0  2  4  6
  //   #D#A#A#C#   1  3  5  7
  //   #########
  let state = State([
    Pos::room(1, 1),
    Pos::room(2, 1),
    Pos::room(0, 0),
    Pos::room(1, 0),
    Pos::room(2, 0),
    Pos::room(3, 1),
    Pos::room(0, 1),
    Pos::room(3, 0),
  ]);
  println!("input {}", state.solve(&mut HashMap::new()).unwrap());

  // #############
  // #...........#
  // ###B#B#C#D###
  //   #D#C#B#A#
  //   #D#B#A#C#
  //   #D#A#A#C#
  //   #########
  let state = State([
    Pos::room(1, 3),
    Pos::room(2, 2),
    Pos::room(2, 3),
    Pos::room(3, 1),
    Pos::room(0, 0),
    Pos::room(1, 0),
    Pos::room(1, 2),
    Pos::room(2, 1),
    Pos::room(1, 1),
    Pos::room(2, 0),
    Pos::room(3, 2),
    Pos::room(3, 3),
    Pos::room(0, 1),
    Pos::room(0, 2),
    Pos::room(0, 3),
    Pos::room(3, 0),
  ]);
  println!("input real {}", state.solve(&mut HashMap::new()).unwrap());
}
