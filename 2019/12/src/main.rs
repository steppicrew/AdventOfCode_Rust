use regex::Regex;

use file;

const REF: u8 = 0;

struct Moon {
    position: [isize; 3],
    velocity: [isize; 3],
}

impl Moon {
    fn energy(&self) -> usize {
        let pot = self.position[0].abs() + self.position[1].abs() + self.position[2].abs();
        let kin = self.velocity[0].abs() + self.velocity[1].abs() + self.velocity[2].abs();
        (kin * pot) as usize
    }
}

fn parse_line(line: &String) -> Moon {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    match re.captures(line.as_str()) {
        Some(captures) => Moon {
            position: [
                captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
            ],
            velocity: [0, 0, 0],
        },
        _ => panic!("Could not parse line {}", line),
    }
}

fn parse_lines(lines: &Vec<String>) -> Vec<Moon> {
    vec![
        parse_line(&lines[0]),
        parse_line(&lines[1]),
        parse_line(&lines[2]),
        parse_line(&lines[3]),
    ]
}

fn apply_gravity(mut moons: Vec<Moon>) -> Vec<Moon> {
    for i in 0..(moons.len() - 1) {
        for j in i..moons.len() {
            for index in 0..3 {
                match moons[i].position[index].cmp(&moons[j].position[index]) {
                    std::cmp::Ordering::Less => {
                        moons[i].velocity[index] += 1;
                        moons[j].velocity[index] -= 1;
                    }
                    std::cmp::Ordering::Greater => {
                        moons[i].velocity[index] -= 1;
                        moons[j].velocity[index] += 1;
                    }
                    std::cmp::Ordering::Equal => {}
                }
            }
        }
    }
    moons
}

fn solve1(lines: &Vec<String>) -> i128 {
    let mut moons = parse_lines(lines);

    let rounds = match REF {
        1 => 10,
        2 => 100,
        0 => 1000,
        _ => 0,
    };

    for round in 0..rounds {
        moons = apply_gravity(moons);
        for i in 0..moons.len() {
            for index in 0..3 {
                moons[i].position[index] += moons[i].velocity[index]
            }
        }
    }

    let mut sum = 0usize;
    for moon in moons {
        sum += moon.energy();
    }

    sum as i128
}

fn solve2(lines: &Vec<String>) -> i128 {
    let mut moons = parse_lines(lines);

    let mut count = 0;
    loop {
        count += 1;
        moons = apply_gravity(moons);
        for i in 0..moons.len() {
            for index in 0..3 {
                moons[i].position[index] += moons[i].velocity[index]
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
