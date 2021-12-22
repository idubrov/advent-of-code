use std::collections::HashMap;

struct Dice(usize);

impl Dice {
  fn next(&mut self) -> usize {
    let val = self.0 + 1;
    self.0 += 1;
    self.0 %= 100;
    val
  }
}

const OUTCOMES: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

#[derive(PartialEq, Eq, Hash, Debug)]
struct State {
  p1: usize,
  p2: usize,
  s1: usize,
  s2: usize,
}

fn advance(states: HashMap<State, usize>, w1: &mut usize, w2: &mut usize) -> HashMap<State, usize> {
  let mut result = HashMap::new();
  for (state, count) in states {
    for p1_outcome in 3..=9 {
      let p1 = ((state.p1 - 1 + p1_outcome) % 10) + 1;
      let s1 = state.s1 + p1;
      let p1_count = OUTCOMES[p1_outcome] * count;
      if s1 >= 21 {
        *w1 += p1_count;
        continue;
      }

      for p2_outcome in 3..=9 {
        let p2 = ((state.p2 - 1 + p2_outcome) % 10) + 1;
        let s2 = state.s2 + p2;
        let p2_count = OUTCOMES[p2_outcome] * p1_count;
        if s2 >= 21 {
          *w2 += p2_count;
          continue;
        }

        *result.entry(State {
          p1,
          p2,
          s1,
          s2,
        }).or_default() += p2_count;
      }
    }
  }
  result
}

fn main() {
  // let start_p1 = 4;
  // let start_p2 = 8;
  let start_p1 = 8;
  let start_p2 = 10;

  // sample
  let mut p1 = start_p1;
  let mut p2 = start_p2;

  let mut d100 = Dice(0);
  let mut s1 = 0;
  let mut s2 = 0;
  let mut rolls = 0;
  loop {
    let r1 = d100.next() + d100.next() + d100.next();
    p1 = (((p1 - 1) + r1) % 10) + 1;
    s1 += p1;
    rolls += 3;
    if s1 >= 1000 {
      eprintln!("{}", s2 * rolls);
      break;
    }
    let r2 = d100.next() + d100.next() + d100.next();
    rolls += 3;
    p2 = (((p2 - 1) + r2) % 10) + 1;
    s2 += p2;
    if s2 >= 1000 {
      eprintln!("{}", s1 * rolls);
      break;
    }
  }

  let mut state = HashMap::new();
  let mut w1 = 0;
  let mut w2 = 0;
  state.insert(State {
    p1: start_p1,
    p2: start_p2,
    s1: 0,
    s2: 0,
  }, 1);
  while !state.is_empty() {
    state = advance(state, &mut w1, &mut w2);
  }

  eprintln!("{}", w1.max(w2));
}
