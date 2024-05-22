use std::collections::HashMap;

use file;
use itertools::Itertools;
use regex::Regex;

fn parse_part(part: &str) -> (String, usize) {
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    match re.captures(part) {
        Some(capture) => (
            capture.get(2).unwrap().as_str().to_owned(),
            capture.get(1).unwrap().as_str().parse().unwrap(),
        ),
        _ => panic!("Could not parse '{}'", part),
    }
}

fn parse_input(lines: &Vec<String>) -> HashMap<String, (usize, Vec<(String, usize)>)> {
    let mut result: HashMap<String, (usize, Vec<(String, usize)>)> = HashMap::new();

    let re_main = Regex::new(r"(\d+ \w+(?:, \d+ \w+)*) => (\d+ \w+)").unwrap();

    for line in lines {
        match re_main.captures(line) {
            Some(capture) => {
                let input = capture.get(1).unwrap().as_str();
                let output = parse_part(capture.get(2).unwrap().as_str());
                let mut inputs = Vec::new();
                for in_part in input.split(", ") {
                    inputs.push(parse_part(in_part));
                }
                result.insert(output.0.to_owned(), (output.1, inputs));
            }
            _ => panic!("Could not parse '{}'", line),
        }
    }

    result
}

fn get_what_i_need(
    recipe: &HashMap<String, (usize, Vec<(String, usize)>)>,
    name: &String,
    count: usize,
) -> (usize, HashMap<String, usize>) {
    let mut result: HashMap<String, usize> = HashMap::new();

    let (unit, needs) = recipe.get(name).unwrap();
    let unit_count = count / unit + if count % unit == 0 { 0 } else { 1 };
    let left_over = unit_count * unit - count;

    for (need_name, need_count) in needs {
        let need_count = result.get(need_name).unwrap_or(&0) + need_count * unit_count;
        result.insert(need_name.to_owned(), need_count);
    }

    (left_over, result)
}

fn solve1(lines: &Vec<String>) -> i128 {
    let recipe = parse_input(lines);
    let mut result: HashMap<String, usize> = HashMap::new();
    let mut left_over: HashMap<String, usize> = HashMap::new();
    result.insert(String::from("FUEL"), 1);
    let mut ore_count = 0;

    while result.len() > 0 {
        let mut new_result: HashMap<String, usize> = HashMap::new();
        for (name, count) in result {
            match count as isize - *left_over.get(&name).unwrap_or(&0) as isize {
                count if count > 0 => {
                    let (_left_over, need) = get_what_i_need(&recipe, &name, count as usize);
                    for (need_name, need_count) in need {
                        match need_name.cmp(&String::from("ORE")) {
                            std::cmp::Ordering::Equal => {
                                ore_count += need_count;
                            }
                            _ => {
                                new_result.insert(
                                    need_name.clone(),
                                    new_result.get(&need_name).unwrap_or(&0) + need_count,
                                );
                            }
                        }
                    }
                    match _left_over {
                        lo if lo == 0 => left_over.remove(&name),
                        _ => left_over.insert(name, _left_over),
                    };
                }
                count => {
                    left_over.insert(name, -count as usize);
                }
            }
        }
        result = new_result;
    }

    ore_count as i128
}

fn get_left_over_key(left_over: &HashMap<String, usize>) -> String {
    let mut keys: Vec<&String> = left_over.keys().collect();
    keys.sort();
    keys.iter()
        .map(|k| format!("{}:{}", k, left_over.get(*k).unwrap()))
        .join(",")
}

fn solve2(lines: &Vec<String>) -> i128 {
    let recipe = parse_input(lines);
    let mut result: HashMap<String, usize> = HashMap::new();
    let mut left_over: HashMap<String, usize> = HashMap::new();
    let mut ore_count = 0;

    let mut left_over_hash: HashMap<String, (usize, usize)> = HashMap::new();
    left_over_hash.insert(get_left_over_key(&left_over), (0, 0));

    let mut fuel_count = 0;
    loop {
        fuel_count += 1;
        println!(
            "FUEL {} ORE_COUNT: {}, KEY: {:?}",
            fuel_count,
            ore_count,
            get_left_over_key(&left_over)
        );
        result.insert(String::from("FUEL"), 1);

        while result.len() > 0 {
            let mut new_result: HashMap<String, usize> = HashMap::new();
            for (name, count) in result {
                match count as isize - *left_over.get(&name).unwrap_or(&0) as isize {
                    count if count > 0 => {
                        let (_left_over, need) = get_what_i_need(&recipe, &name, count as usize);
                        for (need_name, need_count) in need {
                            match need_name.cmp(&String::from("ORE")) {
                                std::cmp::Ordering::Equal => {
                                    ore_count += need_count;
                                }
                                _ => {
                                    new_result.insert(
                                        need_name.clone(),
                                        new_result.get(&need_name).unwrap_or(&0) + need_count,
                                    );
                                }
                            }
                        }
                        match _left_over {
                            lo if lo == 0 => left_over.remove(&name),
                            _ => left_over.insert(name, _left_over),
                        };
                    }
                    count => {
                        left_over.insert(name, -count as usize);
                    }
                }
            }
            result = new_result;
        }
        let left_over_key = get_left_over_key(&left_over);
        match left_over_hash.get(&left_over_key) {
            Some((old_fuel_count, old_ore_count)) => {
                println!(
                    "{:?} -> {:?}",
                    (old_fuel_count, old_ore_count),
                    (fuel_count, ore_count)
                );
                let ore_diff = ore_count - old_ore_count;
                let fuel_diff = fuel_count - old_fuel_count;
                let repeats = (1000000000000 - old_ore_count) / ore_diff;
                let mut fuel = old_fuel_count + repeats * fuel_diff;
                let ore_left = 1000000000000 - old_ore_count - ore_diff * repeats;

                let best_match = left_over_hash
                    .values()
                    .filter(|(_fuel_count, ore_count)| *ore_count <= ore_left)
                    .max_by(|v1, v2| v1.0.cmp(&v2.0))
                    .unwrap();

                fuel += best_match.0;

                return fuel as i128;
            }
            _ => {
                left_over_hash.insert(left_over_key, (fuel_count, ore_count));
            }
        }
    }
}

const REF: u8 = 4;

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
