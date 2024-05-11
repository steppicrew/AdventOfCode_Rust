use std::collections::HashMap;

use file;
use regex::Regex;

const REF: u8 = 0;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i128,
    y: i128,
}

fn parse_line(line: &str) -> HashMap<Point, usize> {
    let mut result: HashMap<Point, usize> = HashMap::new();

    let mut point: Point = Point { x: 0, y: 0 };
    let mut length = 0;
    for part in line.split(",") {
        let re = Regex::new(r"(\w)(\d+)").unwrap();
        let captures = re.captures(part).unwrap();
        let direction = captures.get(1).unwrap().as_str();
        let count = captures.get(2).unwrap().as_str().parse::<i128>().unwrap();
        let diff = match direction {
            "R" => Point { x: 1, y: 0 },
            "L" => Point { x: -1, y: 0 },
            "U" => Point { x: 0, y: 1 },
            "D" => Point { x: 0, y: -1 },
            _ => Point { x: 0, y: 0 },
        };
        for _ in 0..count {
            length += 1;
            point.x += diff.x;
            point.y += diff.y;
            let key = Point {
                x: point.x,
                y: point.y,
            };
            if !result.contains_key(&key) {
                result.insert(key, length);
            }
        }
    }

    result
}

fn solve1(lines: &Vec<String>) -> i128 {
    let wire1 = parse_line(&lines[0]);
    let wire2 = parse_line(&lines[1]);
    let mut result: Option<Point> = None;
    for point in wire1.keys() {
        if wire2.contains_key(point) {
            result = match result {
                None => Some(*point),
                Some(_point) if _point.x.abs() + _point.y.abs() > point.x.abs() + point.y.abs() => {
                    Some(*point)
                }
                _ => result,
            };
        }
    }
    match result {
        None => 0,
        Some(point) => point.x.abs() + point.y.abs(),
    }
}

fn solve2(lines: &Vec<String>) -> i128 {
    let wire1 = parse_line(&lines[0]);
    let wire2 = parse_line(&lines[1]);
    let mut result = wire1.capacity();
    for (point, length) in wire1 {
        if wire2.contains_key(&point) {
            result = result.min(length + wire2.get(&point).unwrap());
        }
    }
    result.try_into().unwrap()
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
