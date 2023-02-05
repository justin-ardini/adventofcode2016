use adventofcode2016::util::read_lines;
use multiset::HashMultiSet;

struct Room {
  pub letters: HashMultiSet<char>,
  pub name: Vec<String>,
  pub id: u32,
}

fn main() {
  let path = "./inputs/day04.txt";
  let lines = read_lines(path);
  println!("-- Part 1 --");
  println!("{}", part1(&lines));
  println!("-- Part 2 --");
  println!("{}", part2(&lines));
}

fn part1<T: AsRef<str>>(lines: &[T]) -> usize {
  let rooms = lines.iter().filter_map(|line| build_room(line.as_ref())).collect::<Vec<Room>>();
  rooms.iter().fold(0, |b, r| b + r.id as usize)
}

fn build_room(line: &str) -> Option<Room> {
  let mut letters: HashMultiSet<char> = HashMultiSet::new();
  let mut id = 0;
  let mut name = Vec::new();
  for part in line.split('-') {
    match part.chars().nth(0).unwrap().to_digit(10) {
      Some(_) => {
        let subparts: Vec<&str> = part.split('[').collect();
        id = subparts[0].parse::<u32>().expect("not a number");
        let mut c_prev = 'a';
        let mut count_prev = 0;
        for c in subparts[1][0..subparts[1].len() - 1].chars() {
          let mut count_max = 0;
          for e in letters.distinct_elements() {
            let count_e = letters.count_of(&e);
            if count_e > count_max {
              count_max = count_e;
            }
          }
          let count = letters.count_of(&c);
          letters.remove_all(&c);
          if count < count_max {
            return None;
          } else if count == count_prev && c < c_prev {
            return None
          }
          c_prev = c;
          count_prev = count;
        }
      }
      None => {
        name.push(String::from(part));
        for c in part.chars() {
          letters.insert(c);
        }
      }
    }
  }
  Some(Room{letters: letters, name: name, id: id})
}

fn part2<T: AsRef<str>>(lines: &[T]) -> i32 {
  // Prints all names with IDs, manually look for the answer.
  let rooms = lines.iter().filter_map(|line| build_room(line.as_ref())).collect::<Vec<Room>>();
  rooms.iter().for_each(|room| {
    room.name.iter().for_each(|part| {
      print!("{} ", part.chars()
          .map(|c| {
            let mut n = (c as u8) + (room.id % 26) as u8;
            if n >= 123 {
              n -= 26;
            }
            n as char
          }).collect::<String>());
    });
    print!("{}", room.id);
    println!();
  });
  -1
}
