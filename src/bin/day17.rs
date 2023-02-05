use adventofcode2016::util::read_lines;
use std::collections::VecDeque;
use md5;

fn main() {
  let path = "./inputs/day17.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines[0]));
  println!("-- Part 2 --");
  println!("{}", part2(&lines[0]));
}

fn part1<T: AsRef<str>>(passcode: T) -> String {
  let mut visited = HashSet::new();
  let mut q = VecDeque::new();
  q.push_front(((0, 0), "".to_string()));
  while !q.is_empty() {
    let (pos, path) = q.pop_back().unwrap();
    if pos == (3, 3) {
      return path.to_string();
    }
    let doors = get_doors(passcode.as_ref(), &path);
    for (i, _d) in doors.iter().enumerate().filter(|(_i, d)| **d) {
      match take_door(pos, i) {
        Some((next, dir)) => {
          let mut next_path = path.to_string();
          next_path.push(dir);
          q.push_front((next, next_path));
        }
        None => (),
      }
    }
  }
  "".to_string()
}

fn part2<T: AsRef<str>>(passcode: T) -> usize {
  let mut q = VecDeque::new();
  q.push_back(((0, 0), "".to_string()));
  let mut max = 0;
  while !q.is_empty() {
    let (pos, path) = q.pop_back().unwrap();
    if pos == (3, 3) {
      if path.len() > max {
        max = path.len();
      }
      continue;
    }
    let doors = get_doors(passcode.as_ref(), &path);
    for (i, _d) in doors.iter().enumerate().filter(|(_i, d)| **d) {
      match take_door(pos, i) {
        Some((next, dir)) => {
          let mut next_path = path.to_string();
          next_path.push(dir);
          q.push_back((next, next_path));
        }
        None => (),
      }
    }
  }
  max
}

fn take_door(pos: (usize, usize), i: usize) -> Option<((usize, usize), char)> {
  match i {
    0 => if pos.0 > 0 {Some(((pos.0 - 1, pos.1), 'U'))} else {None},
    1 => if pos.0 < 3 {Some(((pos.0 + 1, pos.1), 'D'))} else {None},
    2 => if pos.1 > 0 {Some(((pos.0, pos.1 - 1), 'L'))} else {None},
    3 => if pos.1 < 3 {Some(((pos.0, pos.1 + 1), 'R'))} else {None},
    _ => panic!("Invalid door"),
  }
}

// true = open, false = closed
fn get_doors(passcode: &str, path: &str) -> Vec<bool> {
  let hash = format!("{:x}", md5::compute(passcode.to_string() + path));
  hash.chars().take(4).map(|c| {
    match c {
      'b'..='f' => true,
      _ => false,
    }
  }).collect::<Vec<_>>()
}
