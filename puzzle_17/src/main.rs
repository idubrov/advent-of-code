use std::collections::HashSet;

fn main() {
  // let (x1, x2) = (20i32, 30i32);
  // let (y1, y2) = (-10i32, -5i32);

  let (x1, x2) = (211i32, 232i32);
  let (y1, y2) = (-124i32, -69i32);

  let mut uniq = HashSet::new();
  let mut best = 0;
  for y in y1..=y2 {
    for t in 1..=(2 * y.abs()) {
      if (2 * y + t * (t - 1)) % (2 * t) == 0 {
        let vy = (2 * y + t * (t - 1)) / (2 * t);
        for x in x1..=x2 {
          if (2 * x + t * (t - 1)) % (2 * t) == 0 {
            let vx = (2 * x + t * (t - 1)) / (2 * t);
            if vx >= t {
              best = best.max(vy * (vy + 1) / 2);
              uniq.insert((vx, vy));
            }
          }
          let dd = 1 + 8 * x;
          let d = (dd as f64).sqrt() as i32;
          if d * d == dd && d % 2 == 1 {
            let vx = (d - 1) / 2;
            if vx < t {
              best = best.max(vy * (vy + 1) / 2);
              uniq.insert((vx, vy));
            }
          }
        }
      }
    }
  }
  eprintln!("{}", best);
  eprintln!("{}", uniq.len());
}
