use std::collections::HashSet;

use file;
mod intcode;

type ResultType = usize;

const REF: u8 = 0;

type Position = (i16, i16);

fn invert_direction(direction: u8) -> u8 {
    match direction {
        0 => 1,
        1 => 0,
        2 => 3,
        3 => 2,
        d => panic!("Unknown direction {}", d),
    }
}
fn step(position: Position, direction: u8) -> Position {
    match direction {
        0 => (position.0, position.1 - 1),
        1 => (position.0, position.1 + 1),
        2 => (position.0 - 1, position.1),
        3 => (position.0 + 1, position.1),
        d => panic!("Unknown direction {}", d),
    }
}

fn print_maze(seen: &HashSet<Position>, wall: &HashSet<Position>, position: &Position) {
    let min_x = seen.iter().map(|p| p.0).min().unwrap().min(0);
    let max_x = seen.iter().map(|p| p.0).max().unwrap().max(0);
    let min_y = seen.iter().map(|p| p.1).min().unwrap().min(0);
    let max_y = seen.iter().map(|p| p.1).max().unwrap().max(0);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if (x, y) == *position {
                print!("D");
            } else if (0, 0) == (x, y) {
                print!("#");
            } else if wall.contains(&(x, y)) {
                print!("█");
            } else if seen.contains(&(x, y)) {
                print!("·");
            } else {
                print!("?");
            }
        }
        if y == 0 {
            print!(" <-");
        }
        println!("");
    }
    println!("---------------------------------------------");
}

fn print_oxygen(seen: &HashSet<Position>, oxygen: &HashSet<Position>, start_position: &Position) {
    let min_x = seen
        .iter()
        .map(|p| p.0)
        .min()
        .unwrap()
        .min(oxygen.iter().map(|p| p.0).min().unwrap());
    let max_x = seen
        .iter()
        .map(|p| p.0)
        .max()
        .unwrap()
        .max(oxygen.iter().map(|p| p.0).max().unwrap());
    let min_y = seen
        .iter()
        .map(|p| p.1)
        .min()
        .unwrap()
        .min(oxygen.iter().map(|p| p.1).min().unwrap());
    let max_y = seen
        .iter()
        .map(|p| p.1)
        .max()
        .unwrap()
        .max(oxygen.iter().map(|p| p.1).max().unwrap());
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if (0, 0) == (x, y) {
                print!("#");
            } else if (x, y) == *start_position {
                print!("O");
            } else if seen.contains(&(x, y)) {
                print!("·");
            } else if oxygen.contains(&(x, y)) {
                print!("O");
            } else {
                print!(" ");
            }
        }
        if y == 0 {
            print!(" <-");
        }
        println!("");
    }
    println!("---------------------------------------------");
}

fn solve1(lines: &Vec<String>) -> ResultType {
    let mut intcode = intcode::Intcode::new(&lines[0], None);

    let mut position: Position = (0, 0);
    let mut stack: Vec<u8> = Vec::new();
    let mut seen: HashSet<Position> = HashSet::new();
    let mut wall: HashSet<Position> = HashSet::new();

    loop {
        for direction in 0..=4 {
            if direction == 4 {
                match stack.pop() {
                    Some(origin) => {
                        let new_direction = invert_direction(origin);
                        intcode.add_inputs(vec![new_direction as i128 + 1]);
                        match intcode.run() {
                            Some(r) => match r[0] {
                                1 => position = step(position, new_direction),
                                r => panic!("Unexpected result on return: {}", r),
                            },
                            _ => panic!("Program stopped unexpectedly"),
                        }
                    }
                    None => panic!("Empty stack"),
                }
                break;
            }
            let next_position = step(position, direction);
            if seen.contains(&next_position) {
                continue;
            }
            intcode.add_inputs(vec![direction as i128 + 1]);
            match intcode.run() {
                Some(result) => {
                    let result = result[0];
                    seen.insert(next_position);
                    match result {
                        0 => {
                            wall.insert(next_position);
                            print_maze(&seen, &wall, &position);
                        }
                        1 => {
                            position = next_position;
                            stack.push(direction);
                            print_maze(&seen, &wall, &position);
                            break;
                        }
                        2 => {
                            position = next_position;
                            stack.push(direction);
                            print_maze(&seen, &wall, &position);
                            return stack.len();
                        }
                        r => {
                            panic!("Invalid result code {}", r);
                        }
                    }
                }
                None => panic!("Program stopped"),
            }
        }
    }
}

fn fill_oxygen(
    start_position: Position,
    seen: HashSet<Position>,
    wall: HashSet<Position>,
) -> ResultType {
    let mut oxygen: HashSet<Position> = HashSet::new();
    oxygen.insert(start_position);
    let mut all_oxygen: HashSet<Position> = HashSet::new();
    all_oxygen.insert(start_position);
    let mut count = 0;

    let mut seen = seen;

    for p in wall {
        seen.remove(&p);
    }

    while seen.len() > 0 {
        print_oxygen(&seen, &all_oxygen, &start_position);
        count += 1;
        let mut new_oxygen: HashSet<Position> = HashSet::new();
        for p in oxygen {
            for p in [
                (p.0 - 1, p.1),
                (p.0 + 1, p.1),
                (p.0, p.1 - 1),
                (p.0, p.1 + 1),
            ] {
                if seen.remove(&p) {
                    new_oxygen.insert(p);
                    all_oxygen.insert(p);
                }
            }
        }
        oxygen = new_oxygen
    }

    count
}

fn solve2(lines: &Vec<String>) -> ResultType {
    let mut intcode = intcode::Intcode::new(&lines[0], None);

    let mut position: Position = (0, 0);
    let mut start_position: Position = (0, 0);
    let mut stack: Vec<u8> = Vec::new();
    let mut seen: HashSet<Position> = HashSet::new();
    let mut wall: HashSet<Position> = HashSet::new();

    loop {
        for direction in 0..=4 {
            if direction == 4 {
                match stack.pop() {
                    Some(origin) => {
                        let new_direction = invert_direction(origin);
                        intcode.add_inputs(vec![new_direction as i128 + 1]);
                        match intcode.run() {
                            Some(r) => match r[0] {
                                1 => position = step(position, new_direction),
                                r => panic!("Unexpected result on return: {}", r),
                            },
                            _ => panic!("Program stopped unexpectedly"),
                        }
                    }
                    None => {
                        return fill_oxygen(start_position, seen, wall);
                    }
                }
                break;
            }
            let next_position = step(position, direction);
            if seen.contains(&next_position) {
                continue;
            }
            intcode.add_inputs(vec![direction as i128 + 1]);
            match intcode.run() {
                Some(result) => {
                    let result = result[0];
                    seen.insert(next_position);
                    match result {
                        0 => {
                            wall.insert(next_position);
                        }
                        1 => {
                            position = next_position;
                            stack.push(direction);
                            break;
                        }
                        2 => {
                            position = next_position;
                            stack.push(direction);
                            start_position = position;
                            break;
                        }
                        r => {
                            panic!("Invalid result code {}", r);
                        }
                    }
                }
                None => panic!("Program stopped"),
            }
        }
    }
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
