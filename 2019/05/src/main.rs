use file;

const REF: u8 = 0;

fn parse_line(line: &String) -> Vec<isize> {
    line.split(",")
        .map(|i| i.parse::<isize>().unwrap())
        .collect()
}

fn get_param(values: &Vec<isize>, access_mode: isize, pos: usize) -> isize {
    match access_mode {
        1 => values[pos],
        0 => values[values[pos] as usize],
        _ => 0,
    }
}

fn solve1(lines: &Vec<String>) -> i128 {
    let mut values = parse_line(&lines[0]);
    let mut i = 0;
    let input = 1;
    let mut output = 0;

    loop {
        let opcode = values[i] % 100;
        let access_mode = values[i] / 100;
        if opcode == 99 {
            return output;
        }
        match opcode {
            1 => {
                let result_index = values[i + 3] as usize;
                values[result_index] = get_param(&values, access_mode % 10, i + 1)
                    + get_param(&values, access_mode / 10 % 10, i + 2);
            }
            2 => {
                let result_index = values[i + 3] as usize;
                values[result_index] = get_param(&values, access_mode % 10, i + 1)
                    * get_param(&values, access_mode / 10 % 10, i + 2);
            }
            3 => {
                let result_index = values[i + 1] as usize;
                values[result_index] = input;
            }
            4 => {
                output = get_param(&values, access_mode % 10, i + 1) as i128;
            }
            _ => {}
        };

        i += match opcode {
            1 | 2 => 4,
            3 | 4 => 2,
            _ => 0,
        }
    }
}

fn solve2(lines: &Vec<String>) -> i128 {
    let mut values = parse_line(&lines[0]);
    let mut i = 0;
    let input = 5;
    let mut output = 0;

    loop {
        let opcode = values[i] % 100;
        let access_mode = values[i] / 100;
        if opcode == 99 {
            return output;
        }
        match opcode {
            1 => {
                let result_index = values[i + 3] as usize;
                values[result_index] = get_param(&values, access_mode % 10, i + 1)
                    + get_param(&values, access_mode / 10 % 10, i + 2);
            }
            2 => {
                let result_index = values[i + 3] as usize;
                values[result_index] = get_param(&values, access_mode % 10, i + 1)
                    * get_param(&values, access_mode / 10 % 10, i + 2);
            }
            3 => {
                let result_index = values[i + 1] as usize;
                values[result_index] = input;
            }
            4 => {
                output = get_param(&values, access_mode % 10, i + 1) as i128;
            }
            5 => match get_param(&values, access_mode % 10, i + 1) {
                0 => i += 3,
                _ => i = get_param(&values, access_mode / 10 % 10, i + 2) as usize,
            },
            6 => match get_param(&values, access_mode % 10, i + 1) {
                0 => i = get_param(&values, access_mode / 10 % 10, i + 2) as usize,
                _ => i += 3,
            },
            7 => {
                let result_index = values[i + 3] as usize;
                values[result_index] = match get_param(&values, access_mode % 10, i + 1)
                    < get_param(&values, access_mode / 10 % 10, i + 2)
                {
                    true => 1,
                    false => 0,
                };
            }
            8 => {
                let result_index = values[i + 3] as usize;
                values[result_index] = match get_param(&values, access_mode % 10, i + 1)
                    == get_param(&values, access_mode / 10 % 10, i + 2)
                {
                    true => 1,
                    false => 0,
                };
            }
            _ => {}
        };

        i += match opcode {
            1 | 2 | 7 | 8 => 4,
            3 | 4 => 2,
            _ => 0,
        }
    }
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
