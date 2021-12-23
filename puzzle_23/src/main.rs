use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Room {
  room: usize,
  depth: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Hall(usize);

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Pos {
  Room(Room),
  Hall(Hall),
}

impl Pos {
  fn room(room: usize, depth: usize) -> Pos {
    Pos::Room(Room { room, depth })
  }
}

fn distance(room: Room, hall: Hall) -> usize {
  let coords = [0, 1, 3, 5, 7, 9, 10];
  let first = room.room * 2 + 2;
  let second = coords[hall.0];
  let horizontal = if first < second { second - first } else { first - second };
  horizontal + 1 + room.depth
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct State<const ROOMS: usize>([Pos; ROOMS]);

const COST: [usize; 4] = [1, 10, 100, 1000];

impl<const ROOMS: usize> State<ROOMS> {
  const DEPTH: usize = ROOMS / 4;

  fn room_for(idx: usize) -> usize {
    4 * idx / ROOMS
  }

  fn is_completed(&self) -> bool {
    (0..4).all(|room| self.room_filled(room, 0))
  }

  fn path_clear(&self, room: Room, hall: Hall) -> bool {
    const PATHS: [&[usize]; 28] = [
      // room 0
      &[1],
      &[],
      &[],
      &[2],
      &[2, 3],
      &[2, 3, 4],
      &[2, 3, 4, 5],
      // room 1
      &[1, 2],
      &[2],
      &[],
      &[],
      &[3],
      &[3, 4],
      &[3, 4, 5],
      // room 2
      &[1, 2, 3],
      &[2, 3],
      &[3],
      &[],
      &[],
      &[4],
      &[4, 5],
      // room 3
      &[1, 2, 3, 4],
      &[2, 3, 4],
      &[3, 4],
      &[4],
      &[],
      &[],
      &[5],
    ];
    PATHS[room.room * 7 + hall.0]
      .iter()
      .map(|p| Pos::Hall(Hall(*p)))
      .all(|p| self.occupant(p).is_none())
  }

  fn occupant(&self, pos: Pos) -> Option<usize> {
    self.0.iter().position(|x| *x == pos)
  }

  fn room_filled(&self, room: usize, from_depth: usize) -> bool {
    (from_depth..Self::DEPTH).all(|depth| {
      self
        .occupant(Pos::room(room, depth))
        .map_or(false, |occ| Self::room_for(occ) == room)
    })
  }

  fn find_room_spot(&self, room: usize) -> Option<Room> {
    for depth in (0..Self::DEPTH).rev() {
      let candidate = Room { room, depth };
      match self.occupant(Pos::Room(candidate)) {
        Some(occupant) if (Self::room_for(occupant)) != room => return None,
        None => return Some(candidate),
        _ => {}
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
    for idx in 0..ROOMS {
      let my_room = Self::room_for(idx);
      let src = self.0[idx as usize];
      match self.0[idx as usize] {
        // blocked
        Pos::Room(room) if (0..room.depth).any(|d| self.0.contains(&Pos::room(room.room, d))) => {}
        // packed already
        Pos::Room(room) if room.room == my_room && self.room_filled(my_room, room.depth) => {}
        // in room, want hall
        Pos::Room(room) => {
          for hall in 0..7 {
            let target =  Pos::Hall(Hall(hall));
            if self.occupant(target).is_some() {
              continue;
            }
            self.0[idx as usize] = target;
            if self.path_clear(room, Hall(hall)) {
              let delta_cost = distance(room, Hall(hall)) * COST[Self::room_for(idx)];
              let alt_cost = self.solve(visited).map(|cost| cost + delta_cost);
              min_cost = min_cost.or(alt_cost).min(alt_cost.or(min_cost));
            }
          }
        }
        // in hall, want room
        Pos::Hall(hall) => {
          if let Some(room) = self.find_room_spot(my_room) {
            self.0[idx as usize] = Pos::Room(room);
            if self.path_clear(room, hall) {
              let delta_cost = distance(room, hall) * COST[Self::room_for(idx)];
              let alt_cost = self.solve(visited).map(|cost| cost + delta_cost);
              min_cost = min_cost.or(alt_cost).min(alt_cost.or(min_cost));
            }
          }
        }
      }
      // revert!
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
