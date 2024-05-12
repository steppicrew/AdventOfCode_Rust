use std::collections::HashSet;

use file;

const REF: u8 = 0;

fn parse_lines(lines: &Vec<String>) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    result.insert((x, y));
                }
                _ => {}
            }
        }
    }
    result
}

fn get_way_points(a1: (usize, usize), a2: (usize, usize)) -> Vec<(usize, usize)> {
    let dx = a2.0 as isize - a1.0 as isize;
    let dy = a2.1 as isize - a1.1 as isize;
    let ax = dx.abs();
    let ay = dy.abs();
    let mut result = Vec::new();
    let mut ggt = 1;
    for i in 2..=ax.max(ay) {
        if dx % i == 0 && dy % i == 0 {
            ggt = i;
        }
    }
    // println!("dx:{} dy:{} max:{} ggt:{}", dx, dy, ax.max(ay), ggt);
    let dx = dx / ggt;
    let dy = dy / ggt;
    for i in 1..ggt {
        result.push((
            (a1.0 as isize + i * dx) as usize,
            (a1.1 as isize + i * dy) as usize,
        ));
    }

    // println!("{:?} {:?}, {:?}", a1, a2, result);
    result
}

fn solve1(lines: &Vec<String>) -> i128 {
    let asteroids = parse_lines(lines);
    let mut max_count: (usize, (usize, usize)) = (0, (0, 0));
    for a1 in asteroids.clone() {
        let mut count = 0;
        for a2 in asteroids.clone() {
            if a1 == a2 {
                continue;
            }
            let mut found = false;
            for a3 in get_way_points(a1, a2) {
                if asteroids.contains(&a3) {
                    found = true;
                    break;
                }
            }
            if !found {
                count += 1;
            }
        }
        if count > max_count.0 {
            max_count = (count, a1);
        }
    }
    println!("{:?}", max_count);
    max_count.0 as i128
}

fn print_astroids(
    width: usize,
    height: usize,
    asteroids: &HashSet<(usize, usize)>,
    laser_pos: &(usize, usize),
    target: &(usize, usize),
) {
    return;
    for y in 0..height {
        print!("   ");
        for x in 0..width {
            print!(
                "{}",
                match (x, y) {
                    c if c == *laser_pos => "X",
                    c if c == *target => "*",
                    c if asteroids.contains(&c) => "+",
                    _ => "·",
                }
            )
        }
        println!("")
    }
}

fn solve2(lines: &Vec<String>) -> i128 {
    let mut asteroids = parse_lines(lines);

    let laser_pos: (usize, usize) = match REF {
        5 => (11, 13),
        6 => (8, 3),
        0 => (30, 34),
        _ => (0, 0),
    };

    let mut count: usize = 0;

    let sorted_asteroids = asteroids
        .clone()
        .into_iter()
        .map(|a| {
            (
                a.0 as isize - laser_pos.0 as isize,
                a.1 as isize - laser_pos.1 as isize,
            )
        })
        .collect::<Vec<(isize, isize)>>();

    let mut bottom_right: Vec<&(isize, isize)> = sorted_asteroids
        .iter()
        .filter(|p| p.0 >= 0 && p.1 >= 0)
        .collect();
    let mut top_right: Vec<&(isize, isize)> = sorted_asteroids
        .iter()
        .filter(|p| p.0 >= 0 && p.1 < 0)
        .collect();
    let mut top_left: Vec<&(isize, isize)> = sorted_asteroids
        .iter()
        .filter(|p| p.0 < 0 && p.1 < 0)
        .collect();
    let mut bottom_left: Vec<&(isize, isize)> = sorted_asteroids
        .iter()
        .filter(|p| p.0 < 0 && p.1 >= 0)
        .collect();

    let sort_fn_down =
        |p1: &&(isize, isize), p2: &&(isize, isize)| (p2.0 * p1.1).cmp(&(p1.0 * p2.1));

    bottom_right.sort_by(sort_fn_down);
    top_right.sort_by(sort_fn_down);
    top_left.sort_by(sort_fn_down);
    bottom_left.sort_by(sort_fn_down);

    println!("top_right {:?}", top_right);
    println!("bottom_right {:?}", bottom_right);
    println!("bottom_left {:?}", bottom_left);
    println!("top_left {:?}", top_left);

    fn fire(
        asteroids: &mut HashSet<(usize, usize)>,
        laser_pos: &(usize, usize),
        rel_target: (isize, isize),
    ) -> Option<(usize, usize)> {
        let target = (
            (laser_pos.0 as isize + rel_target.0) as usize,
            (laser_pos.1 as isize + rel_target.1) as usize,
        );
        for point in get_way_points(*laser_pos, target) {
            if asteroids.contains(&point) {
                asteroids.remove(&point);
                // println!("WAS {:?}", rel_target);
                // println!("Fire: {:?}", point);
                return Some(point);
            }
        }
        if asteroids.contains(&target) {
            asteroids.remove(&target);
            // println!("WAS {:?}", rel_target);
            // println!("Fire: {:?}", target);
            return Some(target);
        }
        None
    }

    asteroids.remove(&laser_pos);

    loop {
        for q in [&top_right, &bottom_right, &bottom_left, &top_left] {
            let mut last_pos: Option<(isize, isize)> = None;
            for p in q {
                match last_pos {
                    Some(last_pos) => {
                        if p.0 * last_pos.1 == p.1 * last_pos.0 {
                            continue;
                        }
                    }
                    None => {}
                }

                match fire(&mut asteroids, &laser_pos, **p) {
                    Some(target) => {
                        count += 1;
                        last_pos = Some((p.0, p.1));
                        println!("{} ({}) {:?}", count, (count - 1) % 9 + 1, target);
                        print_astroids(
                            lines[0].len(),
                            lines.len(),
                            &asteroids,
                            &laser_pos,
                            &target,
                        );
                        if count == 200 {
                            return (target.0 * 100 + target.1) as i128;
                        }
                        // println!("{:?}", asteroids);
                    }
                    None => {}
                }
                if asteroids.len() == 0 {
                    return 0;
                }
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
