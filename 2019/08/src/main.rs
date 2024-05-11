use std::collections::HashMap;

use file;

const REF: u8 = 0;

fn solve1(lines: &Vec<String>) -> i128 {
    0
}

fn solve2(lines: &Vec<String>) -> i128 {
    0
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
