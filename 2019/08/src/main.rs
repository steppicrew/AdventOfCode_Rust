use file;

const REF: u8 = 0;

const WIDTH: usize = match REF {
    0 => 25,
    1 => 3,
    2 => 2,
    _ => panic!("Wrong REF"),
};
const HEIGHT: usize = match REF {
    0 => 6,
    1 => 2,
    2 => 2,
    _ => panic!("Wrong REF"),
};

fn get_layers(lines: &Vec<String>) -> Vec<String> {
    let line = &lines[0];
    let mut result: Vec<String> = Vec::new();
    let mut i = 0;
    while line.len() > i {
        result.push(line[i..i + WIDTH * HEIGHT].to_owned());
        i += WIDTH * HEIGHT;
    }
    result
}

fn count(layer: &String, needle: char) -> usize {
    let mut count = 0;
    for c in layer.chars() {
        match c {
            c if c == needle => count += 1,
            _ => {}
        }
    }
    count
}

fn solve1(lines: &Vec<String>) -> i128 {
    let layers = get_layers(lines);
    let mut min_0 = count(&layers[0], '0');

    let mut min_layer_index = 0;
    for (i, layer) in layers[1..].iter().enumerate() {
        match count(&layer, '0') {
            c if c < min_0 => {
                min_0 = c;
                min_layer_index = i + 1;
            }
            _ => {}
        }
    }
    (count(&layers[min_layer_index], '1') * count(&layers[min_layer_index], '2')) as i128
}

fn solve2(lines: &Vec<String>) -> i128 {
    let layers = get_layers(lines);
    let mut image = vec![' '; WIDTH * HEIGHT];
    for layer in layers {
        for (i, c) in layer.chars().enumerate() {
            match image[i] {
                ' ' => match c {
                    '0' | '1' => image[i] = c,
                    _ => {}
                },
                _ => {}
            }
        }
    }
    let image = image.iter().collect::<String>();
    for line in 0..HEIGHT {
        println!(
            "{}",
            image[line * WIDTH..(line + 1) * WIDTH]
                .to_owned()
                .replace("0", " ")
                .replace("1", "█")
        )
    }
    0
}

fn main() {
    let filename = file!();

    let lines = file::readinput(filename, REF);
    file::writeoutput(filename, 1, REF, solve1(&lines));
    file::writeoutput(filename, 2, REF, solve2(&lines));
}
