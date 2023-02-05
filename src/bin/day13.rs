use adventofcode2016::util::read_lines;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
  let path = "./inputs/day13.txt";
  let n = read_lines(path)[0].parse::<u32>().expect("NaN");
  println!("-- Part 1 --");
  println!("{}", part1(n));
  println!("-- Part 2 --");
  println!("{}", part2(n));
}

fn part1(n: u32) -> u32 {
  let mut tiles = HashMap::new();
  let mut q = VecDeque::new();
  let mut seen = HashSet::new();
  q.push_front((0, 0, 0));
  while !q.is_empty() {
    let (x, y, moves) = q.pop_back().unwrap();
    if !seen.insert((x, y)) {
      continue;
    }
    if x == 31 && y == 39 {
      return moves;
    }
    let mut next = vec![
      (x + 1, y, get_tile(&mut tiles, &(x + 1, y), n)),
      (x, y + 1, get_tile(&mut tiles, &(x, y + 1), n)),
    ];
    if x > 0 {
      next.push((x - 1, y, get_tile(&mut tiles, &(x - 1, y), n)));
    }
    if y > 0 {
      next.push((x, y - 1, get_tile(&mut tiles, &(x, y - 1), n)));
    }
    for (nx, ny, t) in next.into_iter() {
      if t == 0 {
        q.push_front((nx, ny, moves + 1));
      }
    }
  }
  999
}

fn part2(n: u32) -> usize {
  let mut tiles = HashMap::new();
  let mut q = VecDeque::new();
  let mut seen = HashSet::new();
  q.push_front((0, 0, 0));
  while !q.is_empty() {
    let (x, y, moves) = q.pop_back().unwrap();
    if moves >= 50 {
      continue;
    }
    if !seen.insert((x, y)) {
      continue;
    }
    if x == 31 && y == 39 {
      return moves;
    }
    let mut next = vec![
      (x + 1, y, get_tile(&mut tiles, &(x + 1, y), n)),
      (x, y + 1, get_tile(&mut tiles, &(x, y + 1), n)),
    ];
    if x > 0 {
      next.push((x - 1, y, get_tile(&mut tiles, &(x - 1, y), n)));
    }
    if y > 0 {
      next.push((x, y - 1, get_tile(&mut tiles, &(x, y - 1), n)));
    }
    for (nx, ny, t) in next.into_iter() {
      if t == 0 {
        q.push_front((nx, ny, moves + 1));
      }
    }
  }
  tiles.values().filter(|t| **t == 0).count()
}

fn get_tile(tiles: &mut HashMap<(u32, u32), u32>, xy: &(u32, u32), n: u32) -> u32 {
  match tiles.get(xy) {
    Some(t) => *t,
    None => {
      let t = compute_tile(xy.0, xy.1, n);
      tiles.insert(*xy, t);
      t
    }
  }
}

fn compute_tile(x: u32, y: u32, n: u32) -> u32 {
  let t = x*x + 3*x + 2*x*y + y + y*y + n;
  if format!("{:b}", t).chars().filter(|c| *c == '1').count() % 2 == 0 {0} else {1}
}
