use adventofcode2016::util::read_lines;

fn main() {
  let path = "./inputs/day16.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines[0], 272));
  println!("-- Part 2 --");
  println!("{}", part1(&lines[0], 35651584));
}

fn part1<T: AsRef<str>>(init: T, target: usize) -> String {
  let mut a = String::from(init.as_ref());
  while a.len() < target {
    let b = a.chars().rev().map(|c| if c == '1' {'0'} else {'1'}).collect::<String>();
    a = a + "0" + &b;
  }
  let mut sum = checksum(&a[..target]);
  while sum.len() % 2 == 0 {
    sum = checksum(&sum);
  }
  sum
}

fn checksum(s: &str) -> String {
  let chars = s.chars().collect::<Vec<_>>();
  s.chars().enumerate().filter_map(|(i, c)| {
    if i % 2 == 1 {
      return None;
    } else if c == chars[i + 1] {
      return Some('1');
    } else {
      return Some('0');
    }
  }).collect()
}

fn part2<T: AsRef<str>>(_input: T) -> i32 {
  2
}
