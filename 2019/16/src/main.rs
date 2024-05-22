use core::borrow;
use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashSet, ops::RangeInclusive};

use file;

type ResultType = usize;

fn parse_line(line: &String) -> Vec<i128> {
    line.chars()
        .map(|c| c.to_digit(10).unwrap() as i128)
        .collect()
}

const PATTERN: [i128; 4] = [0, 1, 0, -1];
const PATTERN_LENGTH: usize = PATTERN.len();

fn fft(input: Vec<i128>) -> Vec<i128> {
    let mut result: Vec<i128> = Vec::new();
    for (i, _) in input.iter().enumerate() {
        let mut sum = 0;
        for (j, value) in input.iter().enumerate() {
            let position = ((1 + j) / (i + 1)) % PATTERN_LENGTH;
            sum += *value * PATTERN[position];
        }
        result.push(sum.abs() % 10);
    }
    result
}

fn fft2(
    input: Vec<i128>,
    plus_slices: &Vec<Rc<RefCell<Vec<(usize, usize)>>>>,
    minus_slices: &Vec<Rc<RefCell<Vec<(usize, usize)>>>>,
) -> Vec<i128> {
    (0..input.len())
        .map(|i| {
            if i % 10000 == 0 {
                println!("SubLoop {}", i);
            }
            let plus: i128 = plus_slices[i]
                .borrow()
                .iter()
                .map(|range| &input[range.0..=range.1])
                .flat_map(|s| s.iter())
                .sum();
            let minus: i128 = minus_slices[i]
                .borrow()
                .iter()
                .map(|range| &input[range.0..=range.1])
                .flat_map(|s| s.iter())
                .sum();
            (plus - minus).abs() % 10
        })
        .collect()
}

fn solve1(lines: &Vec<String>) -> ResultType {
    let mut input = parse_line(&lines[0]);
    for _ in 0..100 {
        input = fft(input);
    }

    (0..8)
        .map(|i| input[i])
        .reduce(|acc, i| acc * 10 + i)
        .unwrap() as ResultType
}

fn solve2(lines: &Vec<String>) -> ResultType {
    let line = lines[0].repeat(10000);
    let mut input = parse_line(&line);

    let offset = (0..7)
        .map(|i| input[i])
        .reduce(|acc, i| acc * 10 + i)
        .unwrap() as usize;

    println!("Building slices...");
    let mut plus_slices: Vec<Rc<RefCell<Vec<(usize, usize)>>>> = Vec::new();
    let mut minus_slices: Vec<Rc<RefCell<Vec<(usize, usize)>>>> = Vec::new();
    let len = line.len() - 1;
    for i in 0..line.len() {
        plus_slices.push(Rc::new(RefCell::new(
            (0..line.len())
                .skip(i)
                .step_by((i + 1) * 4)
                .map(|index| (index, (index + i).min(len)))
                .collect(),
        )));
        minus_slices.push(Rc::new(RefCell::new(
            (0..line.len())
                .skip(3 * i + 2)
                .step_by((i + 1) * 4)
                .map(|index| (index, (index + i).min(len)))
                .collect(),
        )));
    }
    println!("Done building slices.");

    for i in 0..100 {
        println!("Loop {}", i);
        input = fft2(input, &plus_slices, &minus_slices);
    }

    (offset..(offset + 8))
        .map(|i| input[i])
        .reduce(|acc, i| acc * 10 + i)
        .unwrap() as ResultType
}

const REF: u8 = 5;

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
