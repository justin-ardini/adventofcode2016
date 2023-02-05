use adventofcode2016::util::read_lines;
use md5;

fn main() {
  let path = "./inputs/day14.txt";
  let lines = read_lines(path);
  let salt = &lines[0];
  println!("-- Part 1 --");
  println!("{}", part1(salt));
  println!("-- Part 2 --");
  println!("{}", part2(salt));
}

fn part1<T: AsRef<str>>(salt: T) -> usize {
  let mut hashes = vec![];
  for i in 0..100000 {
    hashes.push(format!("{:x}", md5::compute(salt.as_ref().to_owned() + &i.to_string())));
  }
  let mut num_keys = 0;
  for i in 0..99000 {
    match get_triplet(&hashes[i]) {
      Some(c) => {
        for j in i + 1..=i + 1000 {
          if has_five(&hashes[j], c) {
            num_keys += 1;
            if num_keys == 64 {
              return i;
            }
            break;
          }
        }
      }
      None => ()
    }
  }
  0
}

fn part2<T: AsRef<str>>(salt: T) -> usize {
  let mut hashes = vec![];
  for i in 0..30000 {
    let mut hash = salt.as_ref().to_owned() + &i.to_string();
    for _ in 0..2017 {
      hash = format!("{:x}", md5::compute(hash));
    }
    hashes.push(hash);
  }
  let mut num_keys = 0;
  for i in 0..29000 {
    match get_triplet(&hashes[i]) {
      Some(c) => {
        for j in i + 1..=i + 1000 {
          if has_five(&hashes[j], c) {
            num_keys += 1;
            if num_keys == 64 {
              return i;
            }
            break;
          }
        }
      }
      None => ()
    }
  }
  0
}

fn get_triplet(hash: &str) -> Option<char> {
  let mut count = 0;
  let mut curr = ' ';
  for c in hash.chars() {
    if c == curr {
      count += 1;
      if count == 3 {
        return Some(curr);
      }
    } else {
      curr = c;
      count = 1;
    }
  }
  return None;
}

fn has_five(hash: &str, c: char) -> bool {
  hash.contains(&c.to_string().repeat(5))
}
