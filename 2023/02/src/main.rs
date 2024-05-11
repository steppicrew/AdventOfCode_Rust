use regex::Regex;

use file;

const REF: u8 = 0;

type Int = i128;

fn parse_line(line: &String) -> (Int, Int, Int, Int) {
    let re_game = Regex::new(r"^Game (\d+):\s*(.+)").unwrap();
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();
    let cap = re_game.captures(&line).unwrap();
    let game_number: Int = cap.get(1).unwrap().as_str().parse::<Int>().unwrap();
    let mut max_red: Int = 0;
    let mut max_green: Int = 0;
    let mut max_blue: Int = 0;

    for game in cap.get(2).unwrap().as_str().split(";") {
        match re_red.captures(game) {
            Some(cap) => {
                let red = cap.get(1).unwrap().as_str().parse::<Int>().unwrap();
                if red > max_red {
                    max_red = red
                }
            }
            None => {}
        }
        match re_green.captures(game) {
            Some(cap) => {
                let green = cap.get(1).unwrap().as_str().parse::<Int>().unwrap();
                if green > max_green {
                    max_green = green
                }
            }
            None => {}
        }
        match re_blue.captures(game) {
            Some(cap) => {
                let blue = cap.get(1).unwrap().as_str().parse::<Int>().unwrap();
                if blue > max_blue {
                    max_blue = blue
                }
            }
            None => {}
        }
    }
    (game_number, max_red, max_green, max_blue)
}

fn solve1(lines: &Vec<String>) -> i128 {
    let mut sum: Int = 0;
    for line in lines {
        let p = parse_line(line);

        if p.1 <= 12 && p.2 <= 13 && p.3 <= 14 {
            sum += p.0
        }
    }
    sum
}

fn solve2(lines: &Vec<String>) -> i128 {
    let mut sum: Int = 0;
    for line in lines {
        let p = parse_line(line);

        sum += p.1 * p.2 * p.3
    }
    sum
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
