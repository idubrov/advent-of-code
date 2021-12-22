use std::collections::{HashMap, HashSet};
use std::io::BufRead;

struct Map {
  map: HashMap<String, Vec<String>>,
}

impl Map {
  fn insert(&mut self, from: &str, to: &str) {
    self.map.entry(from.to_owned()).or_default().push(to.to_owned());
    self.map.entry(to.to_owned()).or_default().push(from.to_owned());
  }

  fn wade<'a>(&'a self, current: &'a str, visited: &mut HashSet<&'a str>, total: &mut usize, can_visit_2nd: &mut bool) {
    if current == "end" {
      *total += 1;
    } else if !visited.contains(current) {
      if current.chars().next().unwrap().is_lowercase() {
        visited.insert(current);
      }
      for out in &self.map[current] {
        self.wade(out, visited, total, can_visit_2nd);
      }
      visited.remove(current);
    } else if current != "start" && *can_visit_2nd {
      *can_visit_2nd = false;
      for out in &self.map[current] {
        self.wade(out, visited, total, can_visit_2nd);
      }
      *can_visit_2nd = true;
    }
  }
}

fn main() {
  let buf = std::io::BufReader::new(std::fs::File::open("puzzle_12/src/input.txt").unwrap());

  let mut map = Map { map: HashMap::new() };
  for line in buf.lines() {
    let line = line.unwrap();
    let mut it = line.split("-");
    let (from, to) = (it.next().unwrap(), it.next().unwrap());
    map.insert(from, to);
  }
  let mut visited = HashSet::new();
  let mut total = 0;
  let mut can_visit_2nd = false;
  map.wade("start", &mut visited, &mut total, &mut can_visit_2nd);
  eprintln!("{:?}", total);

  let mut visited = HashSet::new();
  let mut total = 0;
  let mut can_visit_2nd = true;
  visited.clear();
  map.wade("start", &mut visited, &mut total, &mut can_visit_2nd);
  eprintln!("{:?}", total);
}
