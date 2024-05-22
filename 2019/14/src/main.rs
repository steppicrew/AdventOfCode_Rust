use std::collections::{HashMap, HashSet};

use file;
use regex::Regex;

fn parse_part(part: &str) -> (String, ElemCount) {
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    match re.captures(part) {
        Some(capture) => (
            capture.get(2).unwrap().as_str().to_owned(),
            capture.get(1).unwrap().as_str().parse().unwrap(),
        ),
        _ => panic!("Could not parse '{}'", part),
    }
}

fn parse_input(lines: &Vec<String>) -> HashMap<String, (ElemCount, Vec<(String, ElemCount)>)> {
    let mut result: HashMap<String, (ElemCount, Vec<(String, ElemCount)>)> = HashMap::new();

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

type CacheEntry = (ElemCount, ElemCount, Vec<ElemCount>);
type ElemNum = u8;
type ElemCount = usize;
const ORE_NUM: u8 = 255;

struct Reactor {
    //translate: HashMap<ElemNum, String>,
    translate_back: HashMap<String, ElemNum>,
    recipe: HashMap<ElemNum, (ElemCount, Vec<(ElemNum, ElemCount)>)>,
    // cache: HashMap<String, CacheEntry>,
    dependencies: HashMap<ElemNum, Vec<ElemNum>>,
    // left_over: HashMap<String, usize>,
}

impl Reactor {
    pub fn new(recipe: HashMap<String, (ElemCount, Vec<(String, ElemCount)>)>) -> Reactor {
        //let mut translate = HashMap::new();
        let mut translate_back = HashMap::new();
        //translate.insert(ORE_NUM, String::from("ORE"))));
        translate_back.insert(String::from("ORE"), ORE_NUM);
        for (i, name) in recipe.keys().enumerate() {
            //translate.insert(i as u8, name.clone());
            translate_back.insert(name.clone(), i as u8);
        }

        let mut num_recipe: HashMap<ElemNum, (ElemCount, Vec<(ElemNum, ElemCount)>)> =
            HashMap::new();
        for (name, (unit_count, dependencies)) in recipe {
            num_recipe.insert(
                *translate_back.get(&name).unwrap(),
                (
                    unit_count,
                    dependencies
                        .iter()
                        .map(|d| (*translate_back.get(&d.0).unwrap(), d.1))
                        .collect(),
                ),
            );
        }

        let mut result = Reactor {
            //translate,
            translate_back,
            recipe: num_recipe,
            // cache: HashMap::new(),
            dependencies: HashMap::new(),
            // left_over: HashMap::new(),
        };
        result.build_dependencies();
        result
    }

    fn build_dependencies(&mut self) {
        for (name, (_, _dependecies)) in self.recipe.iter() {
            let mut depencies: HashSet<u8> = HashSet::new();
            let mut test_dependencies: HashSet<u8> = HashSet::new();
            test_dependencies.extend(_dependecies.iter().map(|d| d.0));
            while test_dependencies.len() > 0 {
                let mut new_dependencies: HashSet<u8> = HashSet::new();
                for dependency in test_dependencies {
                    if dependency.cmp(&ORE_NUM).is_ne() {
                        for d in self.recipe.get(&dependency).unwrap().1.iter().map(|d| &d.0) {
                            new_dependencies.insert(*d);
                        }
                    }
                    depencies.insert(dependency);
                }
                test_dependencies = new_dependencies;
            }
            self.dependencies
                .insert(*name, depencies.iter().map(|s| *s).collect());
        }
    }

    pub fn get_num(&self, name: &str) -> ElemNum {
        *self.translate_back.get(&name.to_string()).unwrap()
    }

    pub fn build_cache_key(
        &self,
        count: ElemCount,
        name: ElemNum,
        left_over: &HashMap<ElemNum, ElemCount>,
    ) -> String {
        let dependencies: Vec<String> = self
            .dependencies
            .get(&name)
            .unwrap()
            .iter()
            .map(|d| format!("{}", left_over.get(d).unwrap_or(&0)))
            .collect();
        format!("{}:{}|{}", name, count, dependencies.join(","))
    }

    fn build_cache_value(
        &self,
        ore_count: ElemCount,
        main_left_over: ElemCount,
        name: ElemNum,
        left_over: &HashMap<ElemNum, ElemCount>,
    ) -> CacheEntry {
        let left_overs: Vec<ElemCount> = self
            .dependencies
            .get(&name)
            .unwrap()
            .iter()
            .map(|d| *left_over.get(d).unwrap_or(&0))
            .collect();

        (ore_count, main_left_over, left_overs)
    }

    pub fn produce(
        &self,
        name: ElemNum,
        count: ElemCount,
        left_over: &mut HashMap<ElemNum, ElemCount>,
        cache: &mut HashMap<String, CacheEntry>,
    ) -> ElemCount {
        match left_over.get(&name).unwrap_or(&0) {
            &lo if lo >= count => {
                left_over.insert(name, lo - count);
                return 0;
            }
            &lo => {
                let count = count - lo;
                left_over.insert(name, 0);
                let cache_key = self.build_cache_key(count, name, left_over);
                let ore_count = match cache.get(&cache_key) {
                    Some((ore_count, main_left_over, new_left_over)) => {
                        left_over.insert(name, *main_left_over);
                        for (name, lo) in self
                            .dependencies
                            .get(&name)
                            .unwrap()
                            .iter()
                            .zip(new_left_over)
                        {
                            left_over.insert(*name, *lo);
                        }
                        *ore_count
                    }
                    _ => {
                        let (unit, dependencies) = self.recipe.get(&name).unwrap();
                        let unit_count = count / unit + if count % unit == 0 { 0 } else { 1 };
                        let mut ore_count = 0;
                        for (name, count) in dependencies {
                            ore_count += match name.cmp(&ORE_NUM) {
                                std::cmp::Ordering::Equal => *count * unit_count,
                                _ => self.produce(*name, *count * unit_count, left_over, cache),
                            }
                        }
                        let new_leftover = unit_count * unit - count;
                        cache.insert(
                            cache_key,
                            self.build_cache_value(ore_count, new_leftover, name, left_over),
                        );
                        if new_leftover > 0 {
                            left_over.insert(name, new_leftover);
                        };
                        ore_count
                    }
                };
                ore_count
            }
        }
    }
}

fn solve1(lines: &Vec<String>) -> i128 {
    let reactor = Reactor::new(parse_input(lines));
    let mut left_over: HashMap<ElemNum, ElemCount> = HashMap::new();

    return reactor.produce(
        reactor.get_num("FUEL"),
        1,
        &mut left_over,
        &mut HashMap::new(),
    ) as i128;
}

const ORE_LIMIT: usize = 1000000000000;

fn solve2(lines: &Vec<String>) -> i128 {
    let reactor = Reactor::new(parse_input(lines));
    let fuel = reactor.get_num("FUEL");
    let mut left_over: HashMap<ElemNum, ElemCount> = HashMap::new();
    let mut cache: HashMap<String, CacheEntry> = HashMap::new();
    let mut ore_count = 0;

    let mut left_over_hash: HashMap<String, (usize, usize)> = HashMap::new();
    left_over_hash.insert(reactor.build_cache_key(1, fuel, &left_over), (0, 0));

    let mut fuel_count: usize = 0;
    loop {
        fuel_count += 1;
        ore_count += reactor.produce(fuel, 1, &mut left_over, &mut cache);
        if ore_count > ORE_LIMIT {
            return fuel_count as i128 - 1;
        };

        let left_over_key = reactor.build_cache_key(1, fuel, &left_over);
        match left_over_hash.get(&left_over_key) {
            Some((old_fuel_count, old_ore_count)) => {
                println!(
                    "{:?} -> {:?}",
                    (old_fuel_count, old_ore_count),
                    (fuel_count, ore_count)
                );
                let ore_diff = ore_count - old_ore_count;
                let fuel_diff = fuel_count - old_fuel_count;
                let repeats = (ORE_LIMIT - old_ore_count) / ore_diff;
                let mut fuel = old_fuel_count + repeats * fuel_diff;
                let ore_left = ORE_LIMIT - old_ore_count - ore_diff * repeats;

                let best_match = left_over_hash
                    .values()
                    .filter(|(_fuel_count, ore_count)| *ore_count <= ore_left)
                    .max_by(|v1, v2| v1.0.cmp(&v2.0))
                    .unwrap();

                fuel += best_match.0;

                return fuel as i128;
            }
            _ => {
                if fuel_count % 1000 == 0 {
                    println!(
                        "FUEL {} ORE_COUNT: {}, KEY: {:?}",
                        fuel_count, ore_count, left_over_key
                    );
                };
                left_over_hash.insert(left_over_key, (fuel_count, ore_count));
            }
        }
    }
}

const REF: u8 = 5;

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
