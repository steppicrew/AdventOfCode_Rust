use std::collections::HashMap;

use file;
use itertools::Itertools;

const REF: u8 = 0;

fn solve1(lines: &Vec<String>) -> i128 {
    let parts: Vec<&str> = lines[0].split("-").collect();
    let min = parts[0];
    let max = parts[1].to_string();
    let mut number = min.to_owned();
    let mut count = 0;
    loop {
        let mut last_digit = '0';
        let mut new_number: Vec<char> = Vec::new();
        let mut duplicate = false;
        for digit in number.chars() {
            new_number.push(if digit > last_digit {
                last_digit = digit;
                digit
            } else {
                duplicate = true;
                last_digit
            });
        }

        number = new_number.iter().join("");

        if number > max {
            return count;
        }
        if duplicate {
            count += 1;
        }
        number = (number.parse::<u32>().unwrap() + 1).to_string()
    }
}

fn solve2(lines: &Vec<String>) -> i128 {
    let parts: Vec<&str> = lines[0].split("-").collect();
    let min = parts[0];
    let max = parts[1].to_string();
    let mut number = min.to_owned();
    let mut count = 0;
    loop {
        let mut last_digit = '0';
        let mut new_number: Vec<char> = Vec::new();
        let mut num_count: HashMap<char, u8> = HashMap::new();
        for digit in number.chars() {
            new_number.push(if digit > last_digit {
                last_digit = digit;
                num_count.insert(digit, 1);
                digit
            } else {
                num_count.insert(last_digit, num_count.get(&last_digit).unwrap() + 1);
                last_digit
            });
        }

        number = new_number.iter().join("");

        if number > max {
            return count;
        }
        for counts in num_count.values() {
            if *counts == 2 {
                count += 1;
                break;
            }
        }
        number = (number.parse::<u32>().unwrap() + 1).to_string()
    }
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
