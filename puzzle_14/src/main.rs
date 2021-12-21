use std::collections::HashMap;
use std::io::BufRead;


fn next_pairs(input: &HashMap<(char, char), usize>, dict: &HashMap<(char, char), char>) -> HashMap<(char, char), usize> {
  let mut out = HashMap::new();
  for (key, count) in input {
    if let Some(ch) = dict.get(key) {
      *out.entry((key.0, *ch)).or_default() += *count;
      *out.entry((*ch, key.1)).or_default() += *count;
    } else {
      *out.entry(*key).or_default() += *count;
    }
  }
  out
}

fn score(input: &HashMap<(char, char), usize>) -> usize {
  let mut counts: HashMap<char, usize> = HashMap::new();
  for ((first, second), count) in input {
    *counts.entry(*first).or_default() += count;
    *counts.entry(*second).or_default() += count;
  }
  counts.remove(&'_');
  let mut counts = counts.iter().collect::<Vec<_>>();
  counts.sort_by_key(|(_, other)| *other);
  (counts.last().unwrap().1 - counts.first().unwrap().1) / 2
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_14/src/input.txt").unwrap());
  let mut lines = buf.lines();
  let line = lines.next().unwrap().unwrap();
  let mut input = HashMap::new();
  for idx in 0..line.len() - 1 {
    let mut it = line[idx..].chars();
    *input.entry((it.next().unwrap(), it.next().unwrap())).or_default() += 1;
  }
  *input.entry(('_', line.chars().next().unwrap())).or_default() += 1;
  *input.entry((line[line.len() - 1..].chars().next().unwrap(), '_')).or_default() += 1;

  assert!(lines.next().unwrap().unwrap().is_empty());
  let mut dict = HashMap::new();
  for line in lines {
    let mut it = line.as_ref().unwrap().split(" -> ");
    let (pat, ins) = (it.next().unwrap().to_owned(), it.next().unwrap().chars().next().unwrap());
    let mut it = pat.chars();
    dict.insert((it.next().unwrap(), it.next().unwrap()), ins);
  }

  for _ in 0..10 {
    input = next_pairs(&input, &dict);
  }
  eprintln!("{}", score(&input));

  for _ in 0..30 {
    input = next_pairs(&input, &dict);
  }
  eprintln!("{}", score(&input));
}
