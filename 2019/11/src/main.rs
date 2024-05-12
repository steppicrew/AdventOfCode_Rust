use std::collections::HashMap;

use file;

mod intcode;

const REF: u8 = 0;

fn get_current_color(panel: &HashMap<(isize, isize), bool>, pos: (isize, isize)) -> i128 {
    match panel.get(&pos) {
        Some(true) => 1,
        Some(false) | None => 0,
    }
}
fn solve1(lines: &Vec<String>) -> i128 {
    let mut direction: (isize, isize) = (0, -1);
    let mut panel: HashMap<(isize, isize), bool> = HashMap::new();

    let mut pos: (isize, isize) = (0, 0);

    let mut intcode = intcode::Intcode::new(&lines[0], None);

    loop {
        intcode.add_inputs(vec![get_current_color(&panel, pos)]);
        match intcode.run() {
            Some(output) => {
                match output[0] {
                    0 => panel.insert(pos, false),
                    1 => panel.insert(pos, true),
                    o => panic!("Unknown color output {}", o),
                };
                match output[1] {
                    0 => direction = (direction.1, -direction.0),
                    1 => direction = (-direction.1, direction.0),
                    o => panic!("Unknown color output {}", o),
                }
                pos = (pos.0 + direction.0, pos.1 + direction.1);
            }
            None => break,
        }
    }

    panel.len() as i128
}

fn solve2(lines: &Vec<String>) -> i128 {
    let mut direction: (isize, isize) = (0, -1);
    let mut panel: HashMap<(isize, isize), bool> = HashMap::new();

    let mut pos: (isize, isize) = (0, 0);
    panel.insert(pos, true);

    let mut intcode = intcode::Intcode::new(&lines[0], None);

    loop {
        intcode.add_inputs(vec![get_current_color(&panel, pos)]);
        match intcode.run() {
            Some(output) => {
                match output[0] {
                    0 => panel.insert(pos, false),
                    1 => panel.insert(pos, true),
                    o => panic!("Unknown color output {}", o),
                };
                match output[1] {
                    0 => direction = (direction.1, -direction.0),
                    1 => direction = (-direction.1, direction.0),
                    o => panic!("Unknown color output {}", o),
                }
                pos = (pos.0 + direction.0, pos.1 + direction.1);
            }
            None => break,
        }
    }

    let mut min_x: isize = 0;
    let mut max_x: isize = 0;
    let mut min_y: isize = 0;
    let mut max_y: isize = 0;
    for p in panel.keys() {
        min_x = min_x.min(p.0);
        max_x = max_x.max(p.0);
        min_y = min_y.min(p.1);
        max_y = max_y.max(p.1);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!(
                "{}",
                match get_current_color(&panel, (x, y)) {
                    1 => "█",
                    0 => " ",
                    c => panic!("Unknown color {}", c),
                }
            )
        }
        println!("")
    }
    0
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
