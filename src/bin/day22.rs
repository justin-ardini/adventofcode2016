use adventofcode2016::util::read_lines;

const NORMAL: usize = 0;
const FULL: usize = 1;
const EMPTY: usize = 2;
const GOAL: usize = 3;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Cell {
  pub x: usize,
  pub y: usize,
  pub used: usize,
  pub size: usize,
}

fn main() {
  let path = "./inputs/day22.txt";
  let lines = read_lines(path);
  let cells = parse_lines(&lines);
  // Note: Grid is hardcoded for 34x31 input size.
  let grid = build_grid(&cells);
  print_grid(&grid);
  println!("-- Part 1 --");
  println!("{}", part1(&cells));
  println!("-- Part 2 --");
  println!("{}", part2(&grid));
}

// Note: Extra info was manually stripped from input.
fn parse_lines<T: AsRef<str>>(lines: &[T]) -> Vec<Cell> {
  lines.iter().map(|line| {
    let parts = line.as_ref().split_whitespace().collect::<Vec<_>>();
    let xy = parts[0].split('-').collect::<Vec<_>>();
    let x = xy[0][1..].parse::<usize>().expect("NaN");
    let y = xy[1][1..].parse::<usize>().expect("NaN");
    let size = parts[1][0..parts[1].len()-1].parse::<usize>().expect("NaN");
    let used = parts[2][0..parts[2].len()-1].parse::<usize>().expect("NaN");
    Cell{x: x, y: y, used: used, size: size}
  }).collect::<Vec<_>>()
}

fn build_grid(cells: &[Cell]) -> Vec<Vec<usize>> {
  let mut grid = vec![vec![0; 31]; 34];
  let goal = cells.iter().find(|cell| cell.x == 0 && cell.y == 30).unwrap();
  let max_space = cells.iter().map(|cell| cell.size - cell.used).max().unwrap();
  let goal_space = goal.size - goal.used;
  for cell in cells.iter() {
    let space = cell.size - cell.used;
    if cell.x == 33 && cell.y == 0 {
      grid[cell.x][cell.y] = GOAL;
    } else if space >= max_space {
      grid[cell.x][cell.y] = EMPTY;
    } else if space >= goal_space && cell.used <= 85 {
      grid[cell.x][cell.y] = NORMAL;
    } else {
      grid[cell.x][cell.y] = FULL;
    }
  }
  grid
}

fn part1(cells: &[Cell]) -> i32 {
  let mut n = 0;
  for a in cells {
    for b in cells {
      if a != b && a.used > 0 && a.used <= (b.size - b.used) {
        n += 1;
      }
    }
  }
  n
}

fn part2(grid: &[Vec<usize>]) -> usize {
  let mut goal = grid.iter().enumerate().find_map(|(r, row)| {
    match row.iter().enumerate().find_map(|(c, v)| if *v == GOAL {Some(c)} else {None}) {
      Some(c) => Some((r, c)),
      None => None,
    }
  }).unwrap();
  let mut empty = grid.iter().enumerate().find_map(|(r, row)| {
    match row.iter().enumerate().find_map(|(c, v)| if *v == EMPTY {Some(c)} else {None}) {
      Some(c) => Some((r, c)),
      None => None,
    }
  }).unwrap();
  // First spot above wall is at x=4. Go here then go next to the goal.
  // Manually view the printed grid to see why this works.
  let mut distance = ((empty.0 - 4) + (goal.0 - 4 - 1)) + empty.1;
  empty = (goal.0 - 1, goal.1);
  loop {
    // Swap goal & empty.
    goal = empty;
    // empty = (goal.0 + 1, goal.1);
    distance += 1;
    if goal.0 == 0 && goal.1 == 0 {
      return distance;
    }
    // Then move empty (down, left, left, up).
    empty = (goal.0 - 1, goal.1);
    distance += 4;
  }
}

fn print_grid(grid: &[Vec<usize>]) {
  let p = grid.iter().map(|line| {
    line.iter().map(|i| {
      match i {
        &NORMAL => ".".to_string(),
        &FULL => "#".to_string(),
        &EMPTY => "_".to_string(),
        &GOAL => "G".to_string(),
        _ => i.to_string(),
      }
    }).collect::<String>()
  }).collect::<Vec<_>>();
  println!("{:#?}", p);
}
