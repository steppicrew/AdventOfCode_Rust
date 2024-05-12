use file;
mod intcode;

const REF: u8 = 0;

fn solve1(lines: &Vec<String>) -> i128 {
    let mut intcode = intcode::Intcode::new(&lines[0], Some(vec![1]));
    let output = intcode.run();
    output.unwrap().pop().unwrap()
}

fn solve2(lines: &Vec<String>) -> i128 {
    let mut intcode = intcode::Intcode::new(&lines[0], Some(vec![2]));
    let output = intcode.run();
    output.unwrap().pop().unwrap()
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
