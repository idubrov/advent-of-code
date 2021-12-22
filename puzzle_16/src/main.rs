use std::io::BufRead;

struct Cursor<'a> {
  data: &'a [u8],
  bit: usize,
}

impl<'a> Cursor<'a> {
  fn new(data: &'a [u8]) -> Self {
    Cursor { data, bit: 0 }
  }
  fn read_u8(&mut self, bits: usize) -> u8 {
    let from = self.bit / 8;
    let to = (self.bit + bits - 1) / 8;
    let shift = self.bit % 8;
    let high = (self.data[from] << shift) >> (8 - bits);
    self.bit += bits;
    if from == to {
      high
    } else {
      let low = self.data[to] >> (16 - bits - shift);
      assert_eq!(high | low, high + low);
      high | low
    }
  }
  fn read_u16(&mut self, bits: usize) -> u16 {
    if bits < 8 {
      u16::from(self.read_u8(bits))
    } else {
      u16::from(self.read_u8(bits - 8)) << 8 | u16::from(self.read_u8(8))
    }
  }
}

struct ParseInfo {
  version_sum: usize,
  value: usize,
}


fn parse(cur: &mut Cursor) -> ParseInfo {
  let version = cur.read_u8(3);
  let tid = cur.read_u8(3);
  if tid == 4 {
    let mut lit = 0usize;
    while {
      let flag = cur.read_u8(1);
      lit <<= 4;
      lit |= usize::from(cur.read_u8(4));
      flag != 0
    } {}
    return ParseInfo {
      version_sum: usize::from(version),
      value: lit,
    };
  }

  let mut children = Vec::new();
  if cur.read_u8(1) == 0 {
    let len = cur.read_u16(15);
    let end = cur.bit + usize::from(len);
    while cur.bit < end {
      children.push(parse(cur));
    }
  } else {
    let count = cur.read_u16(11);
    for _ in 0..count {
      children.push(parse(cur))
    }
  }
  let value = match tid {
    0 => children.iter().map(|x| x.value).sum::<usize>(),
    1 => children.iter().map(|x| x.value).product::<usize>(),
    2 => children.iter().map(|x| x.value).min().unwrap(),
    3 => children.iter().map(|x| x.value).max().unwrap(),
    5 if children[0].value > children[1].value => 1,
    5 => 0,
    6 if children[0].value < children[1].value => 1,
    6 => 0,
    7 if children[0].value == children[1].value => 1,
    7 => 0,
    _ => panic!(),
  };
  ParseInfo {
    version_sum: usize::from(version) + children.iter().map(|x| x.version_sum).sum::<usize>(),
    value,
  }
}

fn main() {
  let line = std::io::BufReader::new(std::fs::File::open("puzzle_16/src/input.txt").unwrap())
    .lines()
    .next()
    .unwrap()
    .unwrap();
  let data = (0..line.len())
    .step_by(2)
    .map(|idx| u8::from_str_radix(&line[idx..idx + 2], 16).unwrap())
    .collect::<Vec<u8>>();

  let mut cursor = Cursor::new(&data);
  let result = parse(&mut cursor);
  eprintln!("{}", result.version_sum);
  eprintln!("{}", result.value);
}
