use adventofcode2016::util::read_lines;
use intrusive_collections::intrusive_adapter;
use intrusive_collections::{LinkedList, LinkedListLink};
use std::collections::VecDeque;
use std::cell::Cell;

struct Item {
  link: LinkedListLink,
  value: Cell<i32>,
}

intrusive_adapter!(ItemAdapter = Box<Item>: Item { link: LinkedListLink });

fn main() {
  let path = "./inputs/day19.txt";
  let lines = read_lines(path);
  let num_elves = lines[0].parse::<usize>().expect("NaN");
  println!("-- Part 1 --");
  println!("{}", part1(num_elves));
  println!("-- Part 2 --");
  println!("{}", part2(num_elves));
}

fn part1(num_elves: usize) -> usize {
  let mut presents = vec![1; num_elves];
  let mut i = 0;
  let mut remaining = num_elves;
  loop {
    let curr = presents[i];
    if curr > 0 {
      let mut ni = i;
      let mut next = 0;
      while next == 0 {
        ni = (ni + 1) % num_elves;
        next = presents[ni];
      }
      presents[i] += presents[ni];
      presents[ni] = 0;
      remaining -= 1;
      if remaining == 1 {
        return i + 1;
      }
    }
    i = (i + 1) % num_elves;
  }
}

// Slow (>1 minute), but works.
fn part2(num_elves: usize) -> usize {
  let mut presents = VecDeque::with_capacity(num_elves);
  for i in 1..=num_elves {
    presents.push_back(i);
  }
  let mut i = 0;
  let mut remaining = num_elves;
  loop {
    let curr = presents[i];
    let ni = (i + remaining / 2) % remaining;
    presents.remove(ni);
    remaining -= 1;
    if remaining == 1 {
      return curr;
    }
    if ni < i {
      i = i % remaining;
    } else {
      i = (i + 1) % remaining;
    }
  }
}
