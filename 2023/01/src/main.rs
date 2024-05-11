use regex::Regex;

use file;

const REF: u8 = 0;

fn solve1(lines: &Vec<String>) -> i128 {
    let mut sum = 0;
    let re_first = Regex::new(r"^[a-z]*(\d)").unwrap();
    let re_last = Regex::new(r"(\d)[a-z]*$").unwrap();
    for line in lines {
        // println!("{}", line);
        let cap = re_first.captures(&line).unwrap();
        let first_digit = cap.get(1).unwrap().as_str().to_string();
        let cap = re_last.captures(&line).unwrap();
        let last_digit = cap.get(1).unwrap().as_str();
        sum += (first_digit + last_digit).parse::<i128>().unwrap()
    }

    sum
}

fn solve2(lines: &Vec<String>) -> i128 {
    fn digit2num(digit: &str) -> i128 {
        match digit {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => digit.parse::<i128>().unwrap(),
        }
    }

    let mut sum = 0;
    let re_first =
        Regex::new(r"^[a-z]*?(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re_last = Regex::new(r"\w*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    for line in lines {
        // println!("{}", line);
        let cap = re_first.captures(&line).unwrap();
        let first_digit = cap.get(1).unwrap().as_str();
        let cap = re_last.captures(&line).unwrap();
        let last_digit = cap.get(1).unwrap().as_str();
        println!(
            "{}: {}",
            line,
            digit2num(first_digit) * 10 + digit2num(last_digit)
        );
        sum += digit2num(first_digit) * 10 + digit2num(last_digit)
    }

    sum
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
