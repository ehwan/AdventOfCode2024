use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn smallest_integer_dxdy(mut dx: i64, mut dy: i64) -> (i64, i64) {
    if dx == 0 {
        return (0, if dy > 0 { 1 } else { -1 });
    }
    if dy == 0 {
        return (if dx > 0 { 1 } else { -1 }, 0);
    }

    let x_sign = if dx > 0 { 1 } else { -1 };
    let y_sign = if dy > 0 { 1 } else { -1 };
    dx = dx.abs();
    dy = dy.abs();

    let g = gcd(dx, dy);
    dx /= g;
    dy /= g;

    (x_sign * dx, y_sign * dy)
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut M = 0;
    let mut N = 0;
    let mut antenas: HashMap<u8, Vec<(i64, i64)>> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        M = i as i64;
        N = line.len() as i64;
        for (j, &ch) in line.as_bytes().iter().enumerate() {
            if ch != b'.' {
                antenas.entry(ch).or_default().push((i as i64, j as i64));
            }
        }
    }
    M += 1;

    let mut antinodes = HashSet::new();

    for (_, antenas) in antenas.iter() {
        for i in 0..antenas.len() {
            for j in i + 1..antenas.len() {
                let (x1, y1) = antenas[i];
                let (x2, y2) = antenas[j];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let (dx, dy) = smallest_integer_dxdy(dx, dy);

                let mut x = x1;
                let mut y = y1;
                while (0..N).contains(&x) && (0..M).contains(&y) {
                    x -= dx;
                    y -= dy;
                }
                x += dx;
                y += dy;
                while (0..N).contains(&x) && (0..M).contains(&y) {
                    antinodes.insert((x, y));
                    x += dx;
                    y += dy;
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
