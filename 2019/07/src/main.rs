use file;
mod intcode;
use itertools::Itertools;

const REF: u8 = 0;

fn _solve1(lines: &Vec<String>, phases: Vec<&isize>) -> i128 {
    let mut outputs: Vec<isize> = vec![0];
    for phase in phases {
        let mut intcode =
            intcode::Intcode::new(&lines[0], Some(vec![*phase, outputs.pop().unwrap()]));
        outputs = intcode.run().unwrap();
    }

    outputs.pop().unwrap() as i128
}

fn solve1(lines: &Vec<String>) -> i128 {
    if REF > 0 {
        let phases = match REF {
            1 => vec![&4, &3, &2, &1, &0],
            2 => vec![&0, &1, &2, &3, &4],
            3 => vec![&1, &0, &4, &3, &2],
            _ => vec![&0, &1, &2, &3, &4],
        };
        return _solve1(lines, phases);
    }
    let mut max_output = 0;
    for phases in vec![0, 1, 2, 3, 4].iter().permutations(5).unique() {
        // println!("{:?}", phases);
        max_output = max_output.max(_solve1(lines, phases))
    }
    max_output
}

fn _solve2(line: &String, phases: Vec<&isize>) -> i128 {
    let mut last_output = 0;
    let mut intcode_a = intcode::Intcode::new(&line, Some(vec![*phases[0], 0]));
    let mut intcode_b = intcode::Intcode::new(&line, Some(vec![*phases[1]]));
    let mut intcode_c = intcode::Intcode::new(&line, Some(vec![*phases[2]]));
    let mut intcode_d = intcode::Intcode::new(&line, Some(vec![*phases[3]]));
    let mut intcode_e = intcode::Intcode::new(&line, Some(vec![*phases[4]]));
    loop {
        match intcode_a.run() {
            Some(outputs) => intcode_b.add_inputs(outputs),
            None => break,
        }
        match intcode_b.run() {
            Some(outputs) => intcode_c.add_inputs(outputs),
            None => break,
        }
        match intcode_c.run() {
            Some(outputs) => intcode_d.add_inputs(outputs),
            None => break,
        }
        match intcode_d.run() {
            Some(outputs) => intcode_e.add_inputs(outputs),
            None => break,
        }
        match intcode_e.run() {
            Some(outputs) => {
                last_output = *outputs.last().unwrap() as i128;
                intcode_a.add_inputs(outputs);
            }
            None => break,
        }
    }
    last_output
}

fn solve2(lines: &Vec<String>) -> i128 {
    if REF > 0 {
        let phases = match REF {
            4 => vec![&9, &8, &7, &6, &5],
            5 => vec![&9, &7, &8, &5, &6],
            3 => vec![&1, &0, &4, &3, &2],
            _ => vec![&0, &1, &2, &3, &4],
        };
        return _solve2(&lines[0], phases);
    }
    let mut max_output = 0;
    for phases in vec![5, 6, 7, 8, 9].iter().permutations(5).unique() {
        // println!("{:?}", phases);
        max_output = max_output.max(_solve2(&lines[0], phases))
    }
    max_output
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
