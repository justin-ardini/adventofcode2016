use adventofcode2016::util::read_lines;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq,Eq, Hash)]
struct Item {
  pub id: u32,
  pub is_chip: bool,
}


fn main() {
  let path = "./inputs/day11.txt";
  let lines = read_lines(path);
  let floors = parse_lines(&lines);
  println!("-- Part 1 --");
  println!("{}", part1(&floors, 10));
  println!("-- Part 2 --");
  println!("{}", part2(&floors));
}

fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Vec<BTreeSet<Item>> {
  let mut floors = vec![BTreeSet::new(); 4];
  lines.iter().enumerate().for_each(|(i, line)| {
    let parts = line.as_ref().split("contains").collect::<Vec<_>>();
    if parts[1].contains("nothing relevant") {
      return;
    }
    parts[1].split(", ").for_each(|name| {
      let is_chip = name.contains("chip");
      let id = if name.contains("polonium") {
        0
      } else if name.contains("thulium") {
        1
      } else if name.contains("promethium") {
        2
      } else if name.contains("ruthenium") {
        3
      } else if name.contains("cobalt") {
        4
      } else if name.contains("hydrogen") {
        5
      } else if name.contains("lithium") {
        6
      } else {
        panic!("Invalid name");
      };
      floors[i].insert(Item{id: id, is_chip: is_chip});
    });
  });
  floors
}

fn part1(start: &Vec<BTreeSet<Item>>, n: usize) -> i32 {
  let mut q = VecDeque::new();
  let mut seen = HashSet::new();
  q.push_front((start.clone(), 0, 0));
  while !q.is_empty() {
    let (floors, i, moves) = q.pop_back().unwrap();
    if moves > 100 {
      return -2;
    }
    if !is_valid(&floors) {
      continue;
    }
    if is_win(&floors, n) {
      return moves;
    }
    let floor = floors.get(i).unwrap();
    if i < floors.len() - 1 {
      // 1 item moves
      for (j, item) in floor.iter().enumerate() {
        let mut next = floors.clone();
        let next_floor = next.get_mut(i).unwrap();
        next_floor.remove(item);
        next.get_mut(i + 1).unwrap().insert(*item);
        if seen.insert((next.clone(), i + 1)) {
          q.push_front((next, i + 1, moves + 1));
        }
        // 2 items move
        for (k, item2) in floor.iter().enumerate() {
          if k >= j {
            break;
          }
          if item.is_chip != item2.is_chip && item.id != item2.id {
            continue;
          }
          let mut next = floors.clone();
          let next_floor = next.get_mut(i).unwrap();
          next_floor.remove(item);
          next_floor.remove(item2);
          let to_add = next.get_mut(i + 1).unwrap();
          to_add.insert(*item);
          to_add.insert(*item2);
          if seen.insert((next.clone(), i + 1)) {
            q.push_front((next, i + 1, moves + 1));
          }
        }
      }
    }
    if i > 0 {
      // 1 item moves
      for item in floor.iter() {
        let mut next = floors.clone();
        let next_floor = next.get_mut(i).unwrap();
        next_floor.remove(item);
        next.get_mut(i - 1).unwrap().insert(*item);
        if seen.insert((next.clone(), i - 1)) {
          q.push_front((next, i - 1, moves + 1));
        }
      }
    }
  }
  -1
}

fn part2(floors: &Vec<BTreeSet<Item>>) -> i32 {
  let mut start = floors.clone();
  start[0].insert(Item{id: 10, is_chip: false});
  start[0].insert(Item{id: 10, is_chip: true});
  start[0].insert(Item{id: 11, is_chip: false});
  start[0].insert(Item{id: 11, is_chip: true});
  part1(&start, 14)
}

fn is_valid(floors: &Vec<BTreeSet<Item>>) -> bool {
  floors.iter().all(|floor| is_valid_floor(floor))
}

fn is_valid_floor(floor: &BTreeSet<Item>) -> bool {
  for item in floor.iter() {
    if item.is_chip {
      if !is_safe(floor, item) {
        return false;
      }
    }
  }
  true
}

fn is_safe(floor: &BTreeSet<Item>, chip: &Item) -> bool {
  let mut safe = true;
  for item in floor.iter() {
    if chip == item {
      continue
    }
    if chip.id == item.id {
      return true;
    } else if !item.is_chip {
      safe = false;
    }
  }
  safe
}

fn is_win(floors: &Vec<BTreeSet<Item>>, n: usize) -> bool {
  floors[3].len() >= n
}
