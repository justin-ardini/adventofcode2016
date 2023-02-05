use adventofcode2016::util::read_lines;

fn main() {
  let path = "./inputs/day20.txt";
  let lines = read_lines(path);
  let nums = parse_lines(&lines);
  println!("-- Part 1 --");
  println!("{}", part1(&nums));
  println!("-- Part 2 --");
  println!("{}", part2(&nums));
}

fn part1(nums: &[(u32, u32)]) -> u32 {
  let mut i = 0;
  loop {
    match nums.iter().filter(|range| i >= range.0 && i <= range.1).map(|range| range.1).max() {
      None => {
        return i;
      }
      Some(v) => {
        i = v + 1;
      }
    }
  }
}

fn part2(nums: &[(u32, u32)]) -> usize {
  let mut count = 0;
  let mut i = 0;
  loop {
    match nums.iter().filter(|range| i >= range.0 && i <= range.1).map(|range| range.1).max() {
      None => {
        count += 1;
        i += 1;
      }
      Some(v) => {
        if v == 4294967295 {
          return count;
        }
        i = v + 1;
      }
    }
  }
}

fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Vec<(u32, u32)> {
  lines.iter().map(|line| {
    let nums = line.as_ref().split('-').map(|s| s.parse::<u32>().expect("NaN")).collect::<Vec<_>>();
    (nums[0], nums[1])
  }).collect::<Vec<_>>()
}
