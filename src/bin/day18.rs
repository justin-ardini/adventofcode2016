use adventofcode2016::util::read_lines;

fn main() {
  let path = "./inputs/day18.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines[0], 40));
  println!("-- Part 2 --");
  println!("{}", part1(&lines[0], 400000));
}

fn part1<T: AsRef<str>>(start: T, num_rows: usize) -> i32 {
  let mut rows = vec![];
  rows.push(start.as_ref().chars().map(|c| if c == '.' {1} else {0}).collect::<Vec<_>>());
  for r in 1..num_rows {
    let prev = rows[r - 1].clone();
    let mut next = vec![];
    for c in 0..prev.len() {
      let left = if c == 0 {1} else {prev[c - 1]};
      let mid = prev[c];
      let right = if c == rows[r - 1].len() - 1 {1} else {prev[c + 1]};
      if left == 0 && mid == 0 && right == 1 {
        next.push(0);
      } else if left == 1 && mid == 0 && right == 0 {
        next.push(0);
      } else if left == 0 && mid == 1 && right == 1 {
        next.push(0);
      } else if left == 1 && mid == 1 && right == 0 {
        next.push(0);
      } else {
        next.push(1);
      }
    }
    rows.push(next);
  }
  rows.iter().map(|r| r.iter().sum::<i32>()).sum()
}
