use std::collections::HashMap;

use file;

mod intcode;

const REF: u8 = 0;

fn solve1(lines: &Vec<String>) -> i128 {
    let mut intcode = intcode::Intcode::new(&lines[0], None);

    let mut grid: HashMap<(i128, i128), usize> = HashMap::new();

    let mut output = intcode.run().unwrap();
    while output.len() > 0 {
        let x = output.remove(0);
        let y = output.remove(0);
        let id = output.remove(0) as usize;
        grid.insert((x, y), id);
    }
    grid.values().filter(|v| (**v) == 2).count() as i128
}

fn print_board(grid: &HashMap<(i128, i128), usize>, score: usize, joystick: i128) {
    println!(
        "SCORE: {}, JOYSTICK: {}",
        score,
        match joystick {
            -1 => "⬅",
            0 => "●",
            1 => "➡",
            _ => panic!("Unknown joystick position {}", joystick),
        }
    );
    let min_x = grid.keys().map(|(x, _)| x).min().unwrap();
    let max_x = grid.keys().map(|(x, _)| x).max().unwrap();
    let min_y = grid.keys().map(|(_, y)| y).min().unwrap();
    let max_y = grid.keys().map(|(_, y)| y).max().unwrap();
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            print!(
                "{}",
                match grid.get(&(x, y)) {
                    Some(0) => " ",
                    Some(1) => "█",
                    Some(2) => "▒",
                    Some(3) => "⎼",
                    Some(4) => "●",
                    _ => ".",
                }
            );
        }
        println!("")
    }
}

fn solve2(lines: &Vec<String>) -> i128 {
    let mut line = String::from('2');
    line.push_str(&lines[0][1..]);

    let mut intcode = intcode::Intcode::new(&line, None);

    let mut ball_x = 0i128;
    let mut paddle_x = 0i128;
    let mut score = 0;

    let mut grid: HashMap<(i128, i128), usize> = HashMap::new();

    loop {
        match (intcode.run()) {
            Some(mut output) => {
                while output.len() > 0 {
                    let x = output.remove(0);
                    let y = output.remove(0);
                    let id = output.remove(0) as usize;
                    match (x, y) {
                        (-1, 0) => {
                            score = id;
                        }
                        (x, y) => {
                            grid.insert((x, y), id);
                            match id {
                                3 => paddle_x = x,
                                4 => ball_x = x,
                                _ => {}
                            }
                        }
                    };
                }
            }
            _ => {
                return score as i128;
            }
        }
        let joystick = match ball_x.cmp(&paddle_x) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Less => -1,
        };
        print_board(&grid, score, joystick);

        intcode.add_inputs(vec![joystick])
    }
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
