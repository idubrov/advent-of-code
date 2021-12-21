use std::io::BufRead;

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_03/src/input.txt").unwrap());
  let input = buf
    .lines()
    .map(|s| i32::from_str_radix(&s.unwrap(), 2).unwrap())
    .collect::<Vec<i32>>();
  let bits = 12;
  let mut mask = 1;
  let total = input.len();
  let mut gamma = 0;
  let mut epsilon = 0;
  for _ in 0..bits {
    let ones = input.iter().filter(|x| (**x & mask) != 0).count();
    if ones > total - ones {
      gamma += mask;
    } else {
      epsilon += mask;
    }
    mask *= 2;
  }
  eprintln!("{}", gamma * epsilon);

  let mut oxy_co2_masks = [0, 0];
  for shift in (1..(bits + 1)).rev() {
    let mask = 1 << (shift - 1);
    for oxy_co2 in 0..=1 {
      let total = input.iter().filter(|x| (**x >> shift) == (oxy_co2_masks[oxy_co2] >> shift)).count();
      let ones = input
        .iter()
        .filter(|x| (**x >> shift) == (oxy_co2_masks[oxy_co2] >> shift) && (**x & mask) != 0)
        .count();
      if total == 1 {
        if ones == 1 {
          oxy_co2_masks[oxy_co2] += mask;
        }
      } else if (ones >= total - ones) == (oxy_co2 == 0) {
        oxy_co2_masks[oxy_co2] += mask;
      }
    }
  }
  eprintln!("{}", oxy_co2_masks[0] * oxy_co2_masks[1]);
}
