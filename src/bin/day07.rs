use adventofcode2016::util::read_lines;
use std::collections::HashSet;

fn main() {
  let path = "./inputs/day07.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines));
  println!("-- Part 2 --");
  println!("{}", part2(&lines));
}

fn part1<T: AsRef<str>>(lines: &[T]) -> usize {
  lines.iter().filter(|line| {
    let chars = line.as_ref().chars().collect::<Vec<_>>();
    let mut hypernet = false;
    let mut abba = false;
    for (i, c) in chars.iter().enumerate() {
      match c {
      '[' => {
        hypernet = true;
      }
      ']' => {
        hypernet = false;
      }
      _ => {
        if i > chars.len() - 4 {
          continue;
        }
        if *c == chars[i + 3] && chars[i + 1] == chars[i + 2] && chars[i + 1] != chars[i + 3] {
          if hypernet {
            return false;
          } else {
            abba = true;
          }
        }
      }
      }
    }
    abba
  }).count()
}

fn part2<T: AsRef<str>>(lines: &[T]) -> usize {
  lines.iter().filter(|line| {
    let chars = line.as_ref().chars().collect::<Vec<_>>();
    let mut sets = HashSet::new();
    let mut hypersets = HashSet::new();
    let mut hypernet = false;
    for (i, c) in chars.iter().enumerate() {
      match c {
      '[' => {
        hypernet = true;
      }
      ']' => {
        hypernet = false;
      }
      _ => {
        if i > chars.len() - 3 {
          continue;
        }
        if *c == chars[i + 2] && *c != chars[i + 1] {
          if hypernet {
            if sets.contains(&(chars[i + 1], *c)) {
              return true;
            }
            hypersets.insert((*c, chars[i + 1]));
          } else {
            if hypersets.contains(&(chars[i + 1], *c)) {
              return true;
            }
            sets.insert((*c, chars[i + 1]));
          }
        }
      }
      }
    }
    false
  }).count()
}
