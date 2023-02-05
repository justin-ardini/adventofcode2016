use adventofcode2016::util::read_lines;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum InstructionType {
  Rect,
  RotateColumn,
  RotateRow,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Instruction {
  pub instruction_type: InstructionType,
  pub a: usize,
  pub b: usize,
}

fn main() {
  let path = "./inputs/day08.txt";
  let instructions = parse_lines(&read_lines(path));
  println!("-- Part 1 --");
  println!("{}", part1(&instructions));
  println!("-- Part 2 --");
  println!("{}", part2(&instructions));
}

fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Vec<Instruction> {
  lines.iter().map(|line| {
    if line.as_ref().starts_with("rect") {
      let parts = line.as_ref().split(' ').nth(1).unwrap().split('x').collect::<Vec<_>>();
      let a = parts[0].parse::<usize>().expect("not a number");
      let b = parts[1].parse::<usize>().expect("not a number");
      return Instruction{instruction_type: InstructionType::Rect, a: a, b: b};
    } else if line.as_ref().starts_with("rotate row") {
      let parts = line.as_ref().split(' ').collect::<Vec<_>>();
      let a = parts[2][2..].parse::<usize>().expect("not a number");
      let b = parts[4].parse::<usize>().expect("not a number");
      return Instruction{instruction_type: InstructionType::RotateRow, a: a, b: b};
    } else {
      let parts = line.as_ref().split(' ').collect::<Vec<_>>();
      let a = parts[2][2..].parse::<usize>().expect("not a number");
      let b = parts[4].parse::<usize>().expect("not a number");
      return Instruction{instruction_type: InstructionType::RotateColumn, a: a, b: b};
    }
  }).collect::<Vec<_>>()
}

fn part1(instructions: &[Instruction]) -> usize {
  let mut pixels = [[0; 50]; 6];
  for i in instructions {
    pixels = run_step(&pixels, &i);
  }
  pixels.iter().map(|r| r.iter().sum::<usize>()).sum()
}

fn run_step(curr: &[[usize; 50]; 6], i: &Instruction) -> [[usize; 50]; 6] {
  let mut next = *curr;
  match i.instruction_type {
    InstructionType::Rect => {
      for c in 0..i.a {
        for r in 0..i.b {
          next[r][c] = 1;
        }
      }
    }
    InstructionType::RotateRow => {
      for c in 0..curr[i.a].len() {
        next[i.a][c] = curr[i.a][(c + 50 - i.b) % 50];
      }
    }
    InstructionType::RotateColumn => {
      for r in 0..curr.len() {
        next[r][i.a] = curr[(r + 6 - i.b) % 6][i.a];
      }
    }
  }
  next
}

fn part2(instructions: &[Instruction]) -> usize {
  let mut pixels = [[0; 50]; 6];
  for i in instructions {
    pixels = run_step(&pixels, &i);
  }
  // Answer is printed to console, read it manually.
  for r in pixels.iter() {
    for c in r.iter() {
      print!("{}", if *c == 1 {"#"} else {"."});
    }
    println!();
  }
  2
}
