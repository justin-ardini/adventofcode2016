use adventofcode2016::util::read_lines;
use adventofcode2016::vec2d::Vec2d;
use std::collections::HashSet;

fn main() {
  let path = "./inputs/day01.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines[0]));
  println!("-- Part 2 --");
  println!("{}", part2(&lines[0]));
}

fn part1<T: AsRef<str>>(instrs: T) -> i64 {
  let mut dir = Vec2d {x: 0, y: 1};
  let mut pos = Vec2d {x: 0, y: 0};
  instrs.as_ref().split(", ").for_each(|i| {
    match i.chars().nth(0).unwrap() {
      'L' => {
        dir = dir.rotate(-90)
      }
      'R' => {
        dir = dir.rotate(90)
      }
      _ => ()
    }
    let d = i[1..].parse::<i64>().expect("not a number");
    pos = pos.add(&dir.mult(d));
  });
  pos.manhattan_distance(&Vec2d{x: 0, y: 0})
}

fn part2<T: AsRef<str>>(instrs: T) -> i64 {
  let mut dir = Vec2d {x: 0, y: 1};
  let mut pos = Vec2d {x: 0, y: 0};
  let mut visited = HashSet::new();
  visited.insert(pos);
  for instr in instrs.as_ref().split(", ") {
    match instr.chars().nth(0).unwrap() {
      'L' => {
        dir = dir.rotate(-90)
      }
      'R' => {
        dir = dir.rotate(90)
      }
      _ => ()
    }
    let d = instr[1..].parse::<i64>().expect("not a number");
    for _ in 1..=d {
      pos = pos.add(&dir);
      if visited.contains(&pos) {
        return pos.manhattan_distance(&Vec2d{x: 0, y: 0});
      }
      visited.insert(pos);
    }
  }
  -1
}
