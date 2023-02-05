use adventofcode2016::util::read_lines;
use adventofcode2016::vec2d::Vec2d;
use std::cmp::max;
use std::cmp::min;

fn main() {
  let path = "./inputs/day02.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines));
  println!("-- Part 2 --");
  println!("{}", part2(&lines));
}

fn part1<T: AsRef<str>>(lines: &[T]) -> String {
  let mut pos = Vec2d{x: 1, y: 1};
  lines.iter().map(|line| {
    line.as_ref().chars().for_each(|c| {
    match c {
      'U' => {
        pos.y = max(pos.y - 1, 0);
      }
      'D' => {
        pos.y = min(pos.y + 1, 2);
      }
      'L' => {
        pos.x = max(pos.x - 1, 0);
      }
      'R' => {
        pos.x = min(pos.x + 1, 2);
      }
      _ => ()
    }
    });
    pos.y * 3 + pos.x + 1
  }).map(|i| i.to_string())
  .collect::<String>()
}

fn part2<T: AsRef<str>>(lines: &[T]) -> String {
  let keypad = [
      ['0', '0', '1', '0', '0'],
      ['0', '2', '3', '4', '0'],
      ['5', '6', '7', '8', '9'],
      ['0', 'A', 'B', 'C', '0'],
      ['0', '0', 'D', '0', '0'],
  ];
  let mut pos = Vec2d{x: 0, y: 2};
  lines.iter().map(|line| {
    line.as_ref().chars().for_each(|c| {
    match c {
      'U' => {
        pos.y = max(pos.y - 1, (pos.x - 2).abs());
      }
      'D' => {
        pos.y = min(pos.y + 1, 4 - (pos.x - 2).abs());
      }
      'L' => {
        pos.x = max(pos.x - 1, (pos.y - 2).abs());
      }
      'R' => {
        pos.x = min(pos.x + 1, 4 - (pos.y - 2).abs());
      }
      _ => ()
    }
    });
    keypad[pos.y as usize][pos.x as usize]
  })
  .collect::<String>()
}
