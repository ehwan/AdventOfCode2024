use std::{collections::BTreeMap, fs::read_to_string};

const DIDJ: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn solve_one(
    lines: &[&[u8]],
    visited: &mut Vec<Vec<bool>>,
    ch: u8,
    i: usize,
    j: usize,
) -> (u64, u64) {
    let mut area = 1;

    let mut v = Vec::new();
    visited[i][j] = true;
    v.push((i, j));
    let mut pong = Vec::new();

    let mut vertical_side: BTreeMap<_, Vec<_>> = BTreeMap::new();
    let mut horizontal_side: BTreeMap<_, Vec<_>> = BTreeMap::new();

    while !v.is_empty() {
        pong.clear();

        for (i, j) in v.iter() {
            for (di, dj) in DIDJ {
                let next_i = *i as i32 + di;
                let next_j = *j as i32 + dj;

                if next_i < 0 {
                    horizontal_side.entry(0usize).or_default().push((*j, -1));
                    continue;
                } else if next_i >= lines.len() as i32 {
                    horizontal_side
                        .entry(lines.len())
                        .or_default()
                        .push((*j, 1));
                    continue;
                }

                if next_j < 0 {
                    vertical_side.entry(0usize).or_default().push((*i, -1));
                    continue;
                } else if next_j >= lines[0].len() as i32 {
                    vertical_side
                        .entry(lines[0].len())
                        .or_default()
                        .push((*i, 1));
                    continue;
                }

                let next_i = next_i as usize;
                let next_j = next_j as usize;

                if lines[next_i][next_j] == ch {
                    if !visited[next_i][next_j] {
                        visited[next_i][next_j] = true;
                        area += 1;
                        pong.push((next_i, next_j));
                    }
                } else {
                    match (di, dj) {
                        (-1, 0) => {
                            horizontal_side.entry(*i).or_default().push((*j, -1));
                        }
                        (1, 0) => {
                            horizontal_side.entry(*i + 1).or_default().push((*j, 1));
                        }
                        (0, -1) => {
                            vertical_side.entry(*j).or_default().push((*i, -1));
                        }
                        (0, 1) => {
                            vertical_side.entry(*j + 1).or_default().push((*i, 1));
                        }
                        _ => unreachable!("Invalid direction"),
                    }
                }
            }
        }

        std::mem::swap(&mut v, &mut pong);
    }

    let mut sides = 0;

    for (_, mut side) in vertical_side.into_iter() {
        side.sort();
        let mut side_count = 1;
        for i in 0..side.len() {
            if i < side.len() - 1 {
                if side[i + 1].0 != side[i].0 + 1 || side[i + 1].1 != side[i].1 {
                    side_count += 1;
                }
            }
        }
        sides += side_count;
    }
    for (_, mut side) in horizontal_side.into_iter() {
        side.sort();
        let mut side_count = 1;
        for i in 0..side.len() {
            if i < side.len() - 1 {
                if side[i + 1].0 != side[i].0 + 1 || side[i + 1].1 != side[i].1 {
                    side_count += 1;
                }
            }
        }
        sides += side_count;
    }

    (area, sides)
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let lines: Vec<_> = input.lines().map(|line| line.as_bytes()).collect();
    let mut visited = Vec::new();
    visited.resize_with(lines.len(), || {
        let mut v = Vec::new();
        v.resize(lines[0].len(), false);
        v
    });

    let mut answer = 0;
    let mut area_sum = 0;
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if visited[i][j] {
                continue;
            }

            let (area, side) = solve_one(&lines, &mut visited, lines[i][j], i, j);
            area_sum += area;
            let ch = lines[i][j] as char;
            println!("Ch: {}, Area: {}, Sides: {}", ch, area, side);
            answer += area * side;
        }
    }

    println!("Answer: {}", answer);
    println!("Area Sum: {}", area_sum);
}
