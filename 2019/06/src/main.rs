use std::collections::HashMap;

use file;

const REF: u8 = 0;

fn solve1(lines: &Vec<String>) -> i128 {
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(")").collect();
        let center = parts[0];
        let orbitee = parts[1];
        orbits.insert(orbitee, center);
    }
    let mut count = 0;
    for (_, mut center) in &orbits {
        count += 1;
        while center.cmp(&"COM").is_ne() {
            count += 1;
            center = orbits.get(center).unwrap();
        }
    }
    count
}

fn get_path<'a>(orbits: &'a HashMap<&str, &'a str>, name: &str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let mut center = orbits.get(&name).unwrap();
    while center.cmp(&"COM").is_ne() {
        result.push(&center);
        center = orbits.get(center).unwrap();
    }
    result.reverse();
    result
}

fn solve2(lines: &Vec<String>) -> i128 {
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(")").collect();
        let center = parts[0];
        let orbitee = parts[1];
        orbits.insert(orbitee, center);
    }
    let you: Vec<&str> = get_path(&orbits, "YOU");
    let san: Vec<&str> = get_path(&orbits, "SAN");
    for (i, center) in you.iter().enumerate() {
        if center.cmp(san.get(i).unwrap()).is_ne() {
            return (you.len() + san.len() - 2 * i) as i128;
        }
    }
    0
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
