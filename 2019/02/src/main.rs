use file;

const REF: u8 = 0;

fn parse_line(line: &String) -> Vec<usize> {
    line.split(",")
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

fn solve1(lines: &Vec<String>) -> i128 {
    let mut values = parse_line(&lines[0]);
    values[1] = 12;
    values[2] = 2;
    let mut i = 0;
    loop {
        let opcode = values[i];
        if opcode == 99 {
            return values[0].try_into().unwrap();
        }
        let value1 = &values[values[i + 1]];
        let value2 = &values[values[i + 2]];
        let result = match opcode {
            1 => value1 + value2,
            2 => value1 * value2,
            _ => 0,
        };

        let result_pos = values[i + 3];

        values[result_pos] = result;
        i += 4
    }
}

fn solve2(lines: &Vec<String>) -> i128 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut values = parse_line(&lines[0]);
            values[1] = noun;
            values[2] = verb;
            let mut i = 0;
            loop {
                let opcode = values[i];
                if opcode == 99 {
                    break;
                }
                let value1 = &values[values[i + 1]];
                let value2 = &values[values[i + 2]];
                let result = match opcode {
                    1 => value1 + value2,
                    2 => value1 * value2,
                    _ => 0,
                };

                let result_pos = values[i + 3];

                values[result_pos] = result;
                i += 4
            }
            if values[0] == 19690720 {
                return (100 * noun + verb).try_into().unwrap();
            }
        }
    }
    0
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
