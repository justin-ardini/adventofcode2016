use adventofcode2016::util::read_lines;

// (curr, len)
type Disc = (usize, usize);

fn main() {
  let path = "./inputs/day15.txt";
  let lines = read_lines(path);
  let mut discs = parse_lines(&lines);
  println!("-- Part 1 --");
  println!("{}", part1(&discs));
  println!("-- Part 2 --");
  discs.push((discs.len() + 1, 11));
  println!("{}", part1(&discs));
}

fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Vec<Disc> {
  lines.iter().enumerate().map(|(i, line)| {
    let parts = line.as_ref().split(' ').collect::<Vec<_>>();
    let len = parts[3].parse::<usize>().expect("NaN");
    let curr = parts[11].parse::<usize>().expect("NaN");
    ((curr + i + 1) % len, len)
  }).collect::<Vec<_>>()
}

fn part1(start: &[Disc]) -> i32 {
  let mut discs = start.to_vec();
  for t in 0..10000000 {
    if discs.iter().all(|(curr, _)| *curr == 0) {
      return t;
    }
    discs = discs.iter().map(|(curr, len)| ((*curr + 1) % *len, *len)).collect::<Vec<_>>();
  }
  -1
}
