use adventofcode2016::util::read_lines;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum InstructionType {
  SwapPosition,
  SwapLetter,
  RotateLeft,
  RotateRight,
  RotatePosition,
  Reverse,
  Move,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Instruction {
  pub instruction_type: InstructionType,
  pub x: String,
  pub y: String,
}

fn main() {
  let path = "./inputs/day21.txt";
  let lines = read_lines(path);
  let instructions = parse_lines(&lines);
  println!("-- Part 1 --");
  println!("{}", part1(&instructions));
  println!("-- Part 2 --");
  println!("{}", part2(&instructions));
}

fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Vec<Instruction> {
  lines.iter().map(|line| {
    let l = line.as_ref();
    let parts = l.split(' ').collect::<Vec<_>>();
    if l.starts_with("swap position") {
      let x = parts[2].to_string();
      let y = parts[5].to_string();
      return Instruction{instruction_type: InstructionType::SwapPosition, x: x, y: y};
    } else if l.starts_with("swap letter") {
      let x = parts[2].to_string();
      let y = parts[5].to_string();
      return Instruction{instruction_type: InstructionType::SwapLetter, x: x, y: y};
    } else if l.starts_with("rotate left") {
      let x = parts[2].to_string();
      return Instruction{instruction_type: InstructionType::RotateLeft, x: x, y: "".to_string()};
    } else if l.starts_with("rotate right") {
      let x = parts[2].to_string();
      return Instruction{instruction_type: InstructionType::RotateRight, x: x, y: "".to_string()};
    } else if l.starts_with("rotate based") {
      let x = parts[6].to_string();
      return Instruction{instruction_type: InstructionType::RotatePosition, x: x, y: "".to_string()};
    } else if l.starts_with("reverse position") {
      let x = parts[2].to_string();
      let y = parts[4].to_string();
      return Instruction{instruction_type: InstructionType::Reverse, x: x, y: y};
    } else if l.starts_with("move position") {
      let x = parts[2].to_string();
      let y = parts[5].to_string();
      return Instruction{instruction_type: InstructionType::Move, x: x, y: y};
    } else {
      panic!("Invalid instruction");
    }
  }).collect::<Vec<_>>()
}

fn part1(instructions: &[Instruction]) -> String {
  let mut pw = "abcdefgh".chars().collect::<Vec<_>>();
  for i in instructions {
    match i.instruction_type {
      InstructionType::SwapPosition => {
        let x = i.x.parse::<usize>().expect("NaN");
        let y = i.y.parse::<usize>().expect("NaN");
        let tmp = pw[x];
        pw[x] = pw[y];
        pw[y] = tmp;
      }
      InstructionType::SwapLetter => {
        let a = i.x.chars().next().unwrap();
        let b = i.y.chars().next().unwrap();
        let x = pw.iter().position(|c| *c == a).unwrap();
        let y = pw.iter().position(|c| *c == b).unwrap();
        let tmp = pw[x];
        pw[x] = pw[y];
        pw[y] = tmp;
      }
      InstructionType::RotateLeft => {
        let x = i.x.parse::<usize>().expect("NaN");
        pw.rotate_left(x);
      }
      InstructionType::RotateRight => {
        let x = i.x.parse::<usize>().expect("NaN");
        pw.rotate_right(x);
      }
      InstructionType::RotatePosition => {
        let a = i.x.chars().next().unwrap();
        let x = pw.iter().position(|c| *c == a).unwrap();
        pw.rotate_right(1);
        let n = x + (if x >= 4 {1} else {0});
        pw.rotate_right(n);
      }
      InstructionType::Reverse => {
        let x = i.x.parse::<usize>().expect("NaN");
        let y = i.y.parse::<usize>().expect("NaN");
        for i in x..=y {
          let a = i;  // 2, 3, 4, 5
          let b = x + y - i;  // 5, 4, 3, 2
          if b < a {
            break;
          }
          let tmp = pw[a];
          pw[a] = pw[b];
          pw[b] = tmp;
        }
      }
      InstructionType::Move => {
        let x = i.x.parse::<usize>().expect("NaN");
        let y = i.y.parse::<usize>().expect("NaN");
        let a = pw.remove(x);
        pw.insert(y, a);
      }
    }
  }
  pw.iter().collect::<String>()
}

fn part2(instructions: &[Instruction]) -> String {
  let mut pw = "fbgdceah".chars().collect::<Vec<_>>();
  for i in instructions.iter().rev() {
    match i.instruction_type {
      InstructionType::SwapPosition => {
        let x = i.x.parse::<usize>().expect("NaN");
        let y = i.y.parse::<usize>().expect("NaN");
        let tmp = pw[x];
        pw[x] = pw[y];
        pw[y] = tmp;
      }
      InstructionType::SwapLetter => {
        let a = i.x.chars().next().unwrap();
        let b = i.y.chars().next().unwrap();
        let x = pw.iter().position(|c| *c == a).unwrap();
        let y = pw.iter().position(|c| *c == b).unwrap();
        let tmp = pw[x];
        pw[x] = pw[y];
        pw[y] = tmp;
      }
      InstructionType::RotateLeft => {
        let x = i.x.parse::<usize>().expect("NaN");
        pw.rotate_right(x);
      }
      InstructionType::RotateRight => {
        let x = i.x.parse::<usize>().expect("NaN");
        pw.rotate_left(x);
      }
      InstructionType::RotatePosition => {
        let a = i.x.chars().next().unwrap();
        let x = pw.iter().position(|c| *c == a).unwrap();
        match x {
          0 => {
            pw.rotate_left(1);
          }
          1 => {
            pw.rotate_left(1);
          }
          2 => {
            pw.rotate_left(6);
          }
          3 => {
            pw.rotate_left(2);
          }
          4 => {
            pw.rotate_left(7);
          }
          5 => {
            pw.rotate_left(3);
          }
          6 => (),
          7 => {
            pw.rotate_left(4);
          }
          _ => (),
        }
      }
      InstructionType::Reverse => {
        let x = i.x.parse::<usize>().expect("NaN");
        let y = i.y.parse::<usize>().expect("NaN");
        for i in x..=y {
          let a = i;
          let b = x + y - i;
          if b < a {
            break;
          }
          let tmp = pw[a];
          pw[a] = pw[b];
          pw[b] = tmp;
        }
      }
      InstructionType::Move => {
        let x = i.y.parse::<usize>().expect("NaN");
        let y = i.x.parse::<usize>().expect("NaN");
        let a = pw.remove(x);
        pw.insert(y, a);
      }
    }
  }
  pw.iter().collect::<String>()
}
