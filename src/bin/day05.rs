use adventofcode2016::util::read_lines;
use md5;

fn main() {
  let path = "./inputs/day05.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines[0]));
  println!("-- Part 2 --");
  println!("{}", part2(&lines[0]));
}

fn part1<T: AsRef<str>>(id: T) -> String {
  let mut i = 0;
  let mut result = String::with_capacity(8);
  loop {
    let next = id.as_ref().to_owned();
    let hash = format!("{:x}", md5::compute(next + &i.to_string()));
    if hash.starts_with("00000") {
      result.push(hash.chars().nth(5).unwrap());
      if result.len() == 8 {
        return result;
      }
    }
    i += 1;
  }
}

fn part2<T: AsRef<str>>(id: T) -> String {
  let mut i = 0;
  let mut result = [' '; 8];
  let mut filled = 0;
  loop {
    let next = id.as_ref().to_owned();
    let hash = format!("{:x}", md5::compute(next + &i.to_string()));
    if hash.starts_with("00000") {
      let pos = hash.chars().nth(5).unwrap().to_digit(16).unwrap() as usize;
      if pos < 8 && result[pos] == ' ' {
        result[pos] = hash.chars().nth(6).unwrap();
        filled += 1;
        if filled == 8 {
          return result.iter().collect::<String>();
        }
      }
    }
    i += 1;
  }
}
