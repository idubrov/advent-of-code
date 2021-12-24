use std::io::BufRead;

enum Instruction {}

impl std::str::FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    todo!()
  }
}


// inp a - Read an input value and write it to variable a.
// add a b - Add the value of a to the value of b, then store the result in variable a.
// mul a b - Multiply the value of a by the value of b, then store the result in variable a.
// div a b - Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
// mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
// eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.

fn main() {
  // let input = std::io::BufReader::new(std::fs::File::open("puzzle_24/src/sample.txt").unwrap()).lines();
  // let instructions = input
  //   .map(|line| line.unwrap().parse::<Instruction>().unwrap())
  //   .collect::<Vec<_>>();
  eprintln!("{}", -5/2);
}


fn compute(mut x: i32, mut y: i32, mut z: i32, mut w: i32) {
  inp w
  x *= 0;
  add x z
  mod x 26
  div z 1 PARAM
  add x 14 PARAM
  eql x w
  eql x 0
  mul y 0
  add y 25
  mul y x
  add y 1
  mul z y
  mul y 0
  add y w
  add y 8 PARAM
  mul y x
  add z y

}


/*
(x, y, z, w) = compute(1, 14, 8)
(x, y, z, w) = compute(1, 15, 11)
(x, y, z, w) = compute(1, 13, 2)
(x, y, z, w) = compute(26, -10, 11)
(x, y, z, w) = compute(1, 14, 1)
(x, y, z, w) = compute(26, -3, 5)
(x, y, z, w) = compute(26, -14, 10)
(x, y, z, w) = compute(1, 12, 6)
(x, y, z, w) = compute(1, 14, 1)
(x, y, z, w) = compute(1, 12, 11)
(x, y, z, w) = compute(26, -6, 9)
(x, y, z, w) = compute(26, -6, 14)
(x, y, z, w) = compute(26, -2, 11)
(x, y, z, w) = compute(26, -9, 2)
 */