use adventofcode2016::util::read_lines;
use multiset::HashMultiSet;

fn main() {
  let path = "./inputs/day06.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines));
  println!("-- Part 2 --");
  println!("{}", part2(&lines));
}

fn part1<T: AsRef<str>>(lines: &[T]) -> String {
  let mut sets = vec![HashMultiSet::new(); 8];
  for line in lines {
    line.as_ref().chars().enumerate().for_each(|(i, c)| {
      sets[i].insert(c);
    });
  }
  sets.iter().map(|letters| max_letter(letters)).collect::<String>()
}

fn part2<T: AsRef<str>>(lines: &[T]) -> String {
  let mut sets = vec![HashMultiSet::new(); 8];
  for line in lines {
    line.as_ref().chars().enumerate().for_each(|(i, c)| {
      sets[i].insert(c);
    });
  }
  sets.iter().map(|letters| min_letter(letters)).collect::<String>()
}

fn max_letter(letters: &HashMultiSet<char>) -> char {
  let mut count_max = 0;
  let mut c_max = ' ';
  for c in letters.distinct_elements() {
    let count_c = letters.count_of(&c);
    if count_c > count_max {
      count_max = count_c;
      c_max = *c;
    }
  }
  c_max
}

fn min_letter(letters: &HashMultiSet<char>) -> char {
  let mut count_min = 100;
  let mut c_min = ' ';
  for c in letters.distinct_elements() {
    let count_c = letters.count_of(&c);
    if count_c < count_min {
      count_min = count_c;
      c_min = *c;
    }
  }
  c_min
}
