use std::collections::HashSet;
use std::fs::read_to_string;

const DIDJ: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");
    let map = input.lines().collect::<Vec<_>>();

    let mut scores = Vec::new();
    scores.resize_with(map.len(), || {
        let mut v = Vec::new();
        v.resize(map[0].len(), 0i64);
        v
    });
    let mut positions = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i].as_bytes()[j] == b'9' {
                positions.push((i, j));
                scores[i][j] = 1;
            }
        }
    }
    for search_for in (0..=8).rev() {
        let mut positions_pong = HashSet::new();
        for (i, j) in positions {
            for (di, dj) in DIDJ {
                let i2 = i as i32 + di;
                let j2 = j as i32 + dj;
                if i2 < 0 || i2 >= map.len() as i32 || j2 < 0 || j2 >= map[0].len() as i32 {
                    continue;
                }

                if map[i2 as usize].as_bytes()[j2 as usize] == b'0' + search_for {
                    positions_pong.insert((i2 as usize, j2 as usize));
                    scores[i2 as usize][j2 as usize] += scores[i][j];
                }
            }
        }
        positions = positions_pong.into_iter().collect();
    }

    let mut answer = 0;
    for (i, j) in positions {
        answer += scores[i][j];
    }

    println!("{}", answer);
}
