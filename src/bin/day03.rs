use adventofcode2016::util::read_lines;

fn main() {
  let path = "./inputs/day03.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines));
  println!("-- Part 2 --");
  println!("{}", part2(&lines));
}

fn part1<T: AsRef<str>>(lines: &[T]) -> usize {
  lines.iter().map(|line| {
    line.as_ref().split(" ")
      .map(|part| part.trim())
      .filter(|part| !part.is_empty())
      .map(|part| part.parse::<i32>().expect("not a number"))
      .collect()
  })
  .filter(|sides: &Vec<_>| is_triangle(sides[0], sides[1], sides[2]))
  .collect::<Vec<_>>().len()
}

fn part2<T: AsRef<str>>(lines: &[T]) -> i32 {
  let triples = lines.iter().map(|line| {
    line.as_ref().split(" ")
      .map(|part| part.trim())
      .filter(|part| !part.is_empty())
      .map(|part| part.parse::<i32>().expect("not a number"))
      .collect::<Vec<i32>>()
  }).collect::<Vec<_>>();
  let mut count = 0;
  for i in 0..(triples.len() / 3) {
    let s = i * 3;
    count += if is_triangle(triples[s][0], triples[s + 1][0], triples[s + 2][0]) {1} else {0};
    count += if is_triangle(triples[s][1], triples[s + 1][1], triples[s + 2][1]) {1} else {0};
    count += if is_triangle(triples[s][2], triples[s + 1][2], triples[s + 2][2]) {1} else {0};
  }
  count
}

fn is_triangle(a: i32, b: i32, c: i32) -> bool {
  a + b > c && a + c > b && b + c > a
}
