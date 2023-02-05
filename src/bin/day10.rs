use adventofcode2016::util::read_lines;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum InstructionType {
  Take,
  Give,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Instruction {
  pub instruction_type: InstructionType,
  pub bot: usize,
  pub low: usize,
  pub low_output: bool,
  pub high: usize,
  pub high_output: bool,
  pub value: usize,
}

fn main() {
  let path = "./inputs/day10.txt";
  let instructions = parse_lines(&read_lines(path));
  // Part 1 is printed as bot # as part of part 2.
  println!("{}", part2(&instructions));
}

fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Vec<Instruction> {
  lines.iter().map(|line| {
    if line.as_ref().starts_with("value") {
      let parts = line.as_ref().split(' ').collect::<Vec<_>>();
      let value = parts[1].parse::<usize>().expect("NaN");
      let bot = parts[5].parse::<usize>().expect("NaN");
      return Instruction{instruction_type: InstructionType::Take, bot: bot, value: value, low: 0, low_output: false, high: 0, high_output: false};
    } else {
      let parts = line.as_ref().split(' ').collect::<Vec<_>>();
      let bot = parts[1].parse::<usize>().expect("NaN");
      let low_output = parts[5].starts_with("output");
      let low = parts[6].parse::<usize>().expect("NaN");
      let high_output = parts[10].starts_with("output");
      let high = parts[11].parse::<usize>().expect("NaN");
      return Instruction{instruction_type: InstructionType::Give, bot: bot, value: 0, low: low, low_output: low_output, high: high, high_output: high_output};
    }
  }).collect::<Vec<_>>()
}

fn part2(instructions: &[Instruction]) -> usize {
  let mut bots = HashMap::new();
  let mut values = HashMap::new();
  let mut outputs = HashMap::new();
  for i in instructions {
    match i.instruction_type {
      InstructionType::Take => (),
      InstructionType::Give => {
        bots.insert(i.bot, (i.low, i.low_output, i.high, i.high_output));
      }
    }
  }
  for i in instructions {
    match i.instruction_type {
      InstructionType::Take => {
        take(i.value, i.bot, &mut values, &mut bots, &mut outputs);
      }
      InstructionType::Give => ()
    }
  }
  outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap()
}

fn take(value: usize, bot: usize, values: &mut HashMap<usize, usize>, bots: &mut HashMap<usize, (usize, bool, usize, bool)>, outputs: &mut HashMap<usize, usize>) {
  match values.remove(&bot) {
    Some(value2) => {
      let (low, low_output, high, high_output) = *bots.get(&bot).unwrap();
      let low_val = min(value, value2);
      let high_val = max(value, value2);
      if low_val == 17 && high_val == 61 {
        println!("Bot {}", bot);
      }
      if low_output {
        outputs.insert(low, low_val);
      } else {
        take(low_val, low, values, bots, outputs);
      }
      if high_output {
        outputs.insert(high, high_val);
      } else {
        take(high_val, high, values, bots, outputs);
      }
    }
    None => {
      values.insert(bot, value);
    }
  }
}
