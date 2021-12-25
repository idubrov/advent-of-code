use std::io::BufRead;

fn move_creature(state: &[Vec<u8>], out: &mut Vec<Vec<u8>>, kind: u8) -> bool {
  let width = state[0].len();
  let height = state.len();
  for y in 0..height {
    for x in 0..width {
      out[y][x] = b'.';
    }
  }
  let mut moved = false;
  for y in 0..height {
    for x in 0..width {
      let (nx, ny) = if kind == b'>' { ((x + 1) % width, y) } else { (x, (y + 1) % height) };
      if state[y][x] == kind && state[ny][nx] == b'.' {
        out[ny][nx] = kind;
        moved = true;
      } else if state[y][x] != b'.' {
        out[y][x] = state[y][x];
      }
    }
  }
  moved
}

fn main() {
  let input = std::io::BufReader::new(std::fs::File::open("puzzle_25/src/input.txt").unwrap()).lines();
  let mut state = input
    .map(|line| line.unwrap().into_bytes())
    .collect::<Vec<_>>();
  let mut temp = state.clone();

  for idx in 0.. {
    let moved = move_creature(&state, &mut temp, b'>') | move_creature(&temp, &mut state, b'v');
    if !moved {
      eprintln!("{}", idx + 1);
      break;
    }
  }
}
