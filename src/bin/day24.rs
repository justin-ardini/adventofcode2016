use adventofcode2016::util::read_lines;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const WALL: isize = -2;
const EMPTY: isize = -1;

fn main() {
  let path = "./inputs/day24.txt";
  let lines = read_lines(path);
  let grid = parse_lines(&lines);
  let mut adj = HashMap::new();
  for i in 0..=7 {
    adj.insert(i, find_distances(&grid, i));
  }
  println!("{:?}", adj);
  println!("-- Part 1 --");
  println!("{}", part1(&adj));
  println!("-- Part 2 --");
  println!("{}", part2(&adj));
}

fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Vec<Vec<isize>> {
  lines.iter().map(|line| {
    line.as_ref().chars().map(|c| {
      match c {
        '#' => WALL,
        '.' => EMPTY,
        _ => c.to_digit(10).expect("NaN") as isize,
      }
    }).collect::<Vec<isize>>()
  }).collect::<Vec<_>>()
}

fn find_distances(grid: &[Vec<isize>], start: isize) -> Vec<(isize, isize)> {
  // Get start pos
  let mut start_pos = (0, 0);
  for (r, row) in grid.iter().enumerate() {
    for (c, tile) in row.iter().enumerate() {
      if *tile == start {
        start_pos = (r, c);
        break;
      }
    }
  }
  // bfs for all distances
  let mut visited = HashSet::new();
  visited.insert(start_pos);
  let mut q = VecDeque::new();
  q.push_front((start_pos, 0));
  let mut distances = vec![];
  while !q.is_empty() {
    let (pos, distance) = q.pop_back().unwrap();
    let tile = grid[pos.0][pos.1];
    if tile >= 0 && tile != start {
      distances.push((tile, distance));
    }
    for r in pos.0-1..=pos.0+1 {
      let c = pos.1;
      if r < grid.len() && grid[r][c] != WALL && !visited.contains(&(r, c)) {
        visited.insert((r, c));
        q.push_front(((r, c), distance + 1));
      }
    }
    for c in pos.1-1..=pos.1+1 {
      let r = pos.0;
      if c < grid[r].len() && grid[r][c] != WALL && !visited.contains(&(r, c)) {
        visited.insert((r, c));
        q.push_front(((r, c), distance + 1));
      }
    }
  }
  distances
}

fn part1(adj: &HashMap<isize, Vec<(isize, isize)>>) -> isize {
  let mut reached = HashSet::new();
  reached.insert(0);
  shortest_path(adj, 0, 0, &mut reached)
}

fn shortest_path(adj: &HashMap<isize, Vec<(isize, isize)>>, start: isize, distance: isize, reached: &mut HashSet<isize>) -> isize {
  if reached.len() == adj.len() {
    return distance;
  }
  let mut min_distance = 100000;
  for next in adj.get(&start).unwrap().iter() {
    let n = next.0;
    let ndistance = next.1;
    if reached.contains(&n) {
      continue;
    }
    let mut nreached = reached.clone();
    nreached.insert(n);
    min_distance = min(min_distance, shortest_path(adj, n, distance + ndistance, &mut nreached));
  }
  min_distance
}

fn part2(adj: &HashMap<isize, Vec<(isize, isize)>>) -> isize {
  let mut reached = HashSet::new();
  shortest_path2(adj, 0, 0, &mut reached)
}

// Shortest path except it only counts if the last number is 0.
fn shortest_path2(adj: &HashMap<isize, Vec<(isize, isize)>>, start: isize, distance: isize, reached: &mut HashSet<isize>) -> isize {
  if reached.len() == adj.len() {
    return if start == 0 {distance} else {100000};
  }
  let mut min_distance = 100000;
  for next in adj.get(&start).unwrap().iter() {
    let n = next.0;
    let ndistance = next.1;
    if reached.contains(&n) {
      continue;
    }
    let mut nreached = reached.clone();
    nreached.insert(n);
    min_distance = min(min_distance, shortest_path2(adj, n, distance + ndistance, &mut nreached));
  }
  min_distance
}

fn print_grid(grid: &[Vec<isize>]) {
  let p = grid.iter().map(|line| {
    line.iter().map(|i| {
      match i {
        -2 => "#".to_string(),
        -1 => ".".to_string(),
        _ => i.to_string(),
      }
    }).collect::<String>()
  }).collect::<Vec<_>>();
  println!("{:?}", p);
}
