use image::ImageBuffer;
use std::{fs::read_to_string, io::Write};

const W: i64 = 101;
const H: i64 = 103;

/// parse "x=%d,%d" form into (x, y)
fn parse_vec2(s: &str) -> (i64, i64) {
    let rhs = s
        .split('=')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (rhs[0], rhs[1])
}

fn print_robots(robots: &[((i64, i64), (i64, i64))], iter: u64) {
    let mut img = ImageBuffer::new(W as u32, H as u32);

    for ((x, y), _) in robots {
        img.put_pixel(*x as u32, *y as u32, image::Rgb([255u8, 255u8, 255u8]));
    }

    let name = format!("robots_{:05}.png", iter);

    img.save(name).unwrap();
}

fn move_robots(robots: &mut Vec<((i64, i64), (i64, i64))>) {
    for ((x, y), (vx, vy)) in robots.iter_mut() {
        *x += *vx;
        *y += *vy;

        *x = (*x + W) % W;
        *y = (*y + H) % H;
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    println!("Warning: This program will generate thousands of images.");
    print!("To proceed, enter \"yes\": ");
    std::io::stdout().flush().unwrap();
    let mut yes_string = String::new();
    std::io::stdin().read_line(&mut yes_string).unwrap();
    if yes_string.trim() != "yes" {
        return;
    }

    let mut robots = input
        .lines()
        .map(|line| {
            let mut line_it = line.split(' ');

            let pos = parse_vec2(line_it.next().unwrap());
            let vel = parse_vec2(line_it.next().unwrap());

            (pos, vel)
        })
        .collect::<Vec<_>>();

    let mut iter = 0;
    while iter < 10000 {
        println!("Iteration: {}", iter);
        print_robots(robots.as_slice(), iter);

        iter += 1;
        move_robots(&mut robots);
    }
}
