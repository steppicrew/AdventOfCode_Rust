use regex::Regex;
use std::fs;

fn analyze_file(filename: &str, test: u8) -> (String, String, String) {
    let re = Regex::new(r"(\d+)\/(\d+)\/src\/main\.rs").unwrap();
    let caps = re.captures(filename).unwrap();
    let year = caps.get(1).unwrap().as_str();
    let day = caps.get(2).unwrap().as_str();

    let extension = if test > 0 {
        format!("_ref{}.txt", test)
    } else {
        ".txt".to_string()
    };
    (year.to_owned(), day.to_owned(), extension)
}

fn get_abs_path(path: String) -> String {
    "/home/stephan/git/AdventOfCode_Rust/".to_string() + &path
}

pub fn readinput(filename: &str, test: u8) -> Vec<String> {
    let (year, day, extension) = analyze_file(filename, test);
    let input_filepath = get_abs_path(format!("./{}/data/input_{}{}", year, day, extension));

    println!("Reading input from {}", input_filepath);
    let contents =
        fs::read_to_string(input_filepath).expect("Should have been able to read the file");

    contents
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.to_owned())
        .collect()
}

pub fn writeoutput(filename: &str, part: u8, test: u8, result: i128) {
    let (year, day, extension) = analyze_file(filename, test);
    let output_filepath = get_abs_path(format!(
        "./{}/data/result_{}_{}{}",
        year, day, part, extension
    ));
    fs::write(output_filepath, format!("{}", result)).expect("Could not write result");
    println!("{}_{} result: {}", day, part, result)
}
