use adventofcode2016::util::read_lines;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum InstructionType {
  Cpy,
  Inc,
  Dec,
  Jnz,
  Tgl,
  Out,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Instruction {
  pub instruction_type: InstructionType,
  pub x: String,
  pub y: String,
}


fn main() {
  let path = "./inputs/day25.txt";
  let lines = read_lines(path);
  let instructions = parse_lines(&lines);
  println!("-- Part 1 --");
  println!("{}", part1(&mut instructions.clone()));
  println!("-- Part 2 --");
  println!("{}", part2(&mut instructions.clone()));
}

fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Vec<Instruction> {
  lines.iter().map(|line| {
    let parts = line.as_ref().split(' ').collect::<Vec<_>>();
    match parts[0] {
      "inc" => {
        let x = parts[1];
        return Instruction{instruction_type: InstructionType::Inc, x: x.to_string(), y: "".to_string()};
      }
      "dec" => {
        let x = parts[1];
        return Instruction{instruction_type: InstructionType::Dec, x: x.to_string(), y: "".to_string()};
      }
      "cpy" => {
        let x = parts[1];
        let y = parts[2];
        return Instruction{instruction_type: InstructionType::Cpy, x: x.to_string(), y: y.to_string()};
      }
      "jnz" => {
        let x = parts[1];
        let y = parts[2];
        return Instruction{instruction_type: InstructionType::Jnz, x: x.to_string(), y: y.to_string()};
      }
      "tgl" => {
        let x = parts[1];
        return Instruction{instruction_type: InstructionType::Tgl, x: x.to_string(), y: "".to_string()};
      }
      "out" => {
        let x = parts[1];
        return Instruction{instruction_type: InstructionType::Out, x: x.to_string(), y: "".to_string()};
      }
      _ => panic!("Invalid instruction")
    }
  }).collect::<Vec<_>>()
}

fn part1(instructions: &[Instruction]) -> i32 {
  let mut i = 0;
  loop {
    let mut next = instructions.to_vec();
    if is_clock_signal(&mut next, i, 10) {
      return i;
    }
    i += 1;
  }
}

fn is_clock_signal(instructions: &mut Vec<Instruction>, init: i32, limit: usize) -> bool {
  let mut regs = [0; 4];
  regs[0] = init;
  let mut p: i32 = 0;
  let mut next = 0;
  let mut count = 0;
  loop {
    if p >= instructions.len() as i32 {
      break;
    }
    let i = &instructions[p as usize];
    match i.instruction_type {
      InstructionType::Inc => {
        let x = to_index(&i.x);
        regs[x] += 1;
      }
      InstructionType::Dec => {
        let x = to_index(&i.x);
        regs[x] -= 1;
      }
      InstructionType::Cpy => {
        let x = to_val(&i.x, &regs);
        let y = to_index(&i.y);
        regs[y] = x;
      }
      InstructionType::Jnz => {
        let x = to_val(&i.x, &regs);
        if x != 0 {
          p += to_val(&i.y, &regs);
          continue;
        }
      }
      InstructionType::Tgl => {
        let x = to_val(&i.x, &regs);
        let n = (p + x) as usize;
        if n >= 0 && n < instructions.len() {
          let mut instr = &mut instructions[n];
          match instr.instruction_type {
            InstructionType::Inc => {
              instr.instruction_type = InstructionType::Dec;
            }
            InstructionType::Dec => {
              instr.instruction_type = InstructionType::Inc;
            }
            InstructionType::Tgl => {
              instr.instruction_type = InstructionType::Inc;
            }
            InstructionType::Cpy => {
              instr.instruction_type = InstructionType::Jnz;
            }
            InstructionType::Jnz => {
              instr.instruction_type = InstructionType::Cpy;
            }
            InstructionType::Out => {
              instr.instruction_type = InstructionType::Inc;
            }
          }
        }
      }
      InstructionType::Out => {
        let x = to_val(&i.x, &regs);
        if x != next {
          return false;
        }
        count += 1;
        if count == limit {
          return true;
        }
        if next == 0 {
          next = 1;
        } else {
          next = 0;
        }
      }
    }
    p += 1;
  }
  false
}

fn to_index(s: &str) -> usize {
  match s {
    "a" => 0,
    "b" => 1,
    "c" => 2,
    "d" => 3,
    _ => panic!("Invalid register")
  }
}

fn to_val(s: &str, regs: &[i32; 4]) -> i32 {
  match s {
    "a" => regs[0],
    "b" => regs[1],
    "c" => regs[2],
    "d" => regs[3],
    _ => s.parse::<i32>().expect("NaN"),
  }
}

fn part2(_instructions: &mut Vec<Instruction>) -> i32 {
  1
}
