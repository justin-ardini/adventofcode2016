use adventofcode2016::util::read_lines;

fn main() {
  let path = "./inputs/day09.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines[0]));
  println!("-- Part 2 --");
  println!("{}", part2(&lines[0]));
}

fn part1<T: AsRef<str>>(seq: T) -> usize {
  let chars = seq.as_ref().chars().collect::<Vec<_>>();
  let mut i = 0;
  let mut len = 0;
  while i < chars.len() {
    let marker = chars[i + 1..].splitn(2, |c| *c == ')').nth(0).unwrap().iter().collect::<String>();
    let ab = marker.split('x').map(|s| s.parse::<usize>().expect("NaN")).collect::<Vec<_>>();
    len += ab[0] * ab[1];
    i += marker.len() + 2 + ab[0];
  }
  len
}

fn part2<T: AsRef<str>>(seq: T) -> usize {
  let chars = seq.as_ref().chars().collect::<Vec<_>>();
  return len_rec(&chars, 1, 0, chars.len());
}

fn len_rec(chars: &Vec<char>, mult: usize, s: usize, e: usize) -> usize {
  let mut len = 0;
  let mut i = s;
  while i < e {
    let c = chars[i];
    if c == '(' {
      let marker = chars[i + 1..].splitn(2, |c| *c == ')').nth(0).unwrap().iter().collect::<String>();
      let ab = marker.split('x').map(|s| s.parse::<usize>().expect("NaN")).collect::<Vec<_>>();
      len += len_rec(chars, mult * ab[1], i + marker.len() + 2, i + marker.len() + 2 + ab[0]);
      i += marker.len() + 2 + ab[0];
    } else {
      len += mult;
      i += 1
    }
  }
  len
}
