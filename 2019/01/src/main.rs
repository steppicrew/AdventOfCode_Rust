use file;

const REF: u8 = 0;

fn solve1(lines: &Vec<String>) -> i128 {
    let mut sum = 0;
    for line in lines {
        let mass = line.parse::<i128>().unwrap();
        sum += mass / 3 - 2;
    }

    sum
}

fn solve2(lines: &Vec<String>) -> i128 {
    let mut sum = 0;
    for line in lines {
        let mut line_sum: i128 = 0;
        let mass = line.parse::<i128>().unwrap();
        line_sum += mass / 3 - 2;

        let mut mass = line_sum;
        while mass > 0 {
            mass = mass / 3 - 2;
            if mass > 0 {
                line_sum += mass;
            }
        }
        sum += line_sum;
    }

    sum
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
