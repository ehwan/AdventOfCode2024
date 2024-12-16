use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let board = input.lines().collect::<Vec<_>>();
    let mut scores_2d = Vec::new();
    scores_2d.resize_with(board.len(), || {
        let mut l = Vec::new();
        l.resize(board[0].len(), u64::MAX);
        l
    });
    let mut scores = Vec::new();
    scores.resize(4, scores_2d);

    let mut bfs: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut pong = HashSet::new();
    let mut end_pos = (0, 0);
    for (y, line) in board.iter().enumerate() {
        for (x, ch) in line.bytes().enumerate() {
            if ch == b'S' {
                bfs.insert((x, y, 0));
                scores[0][y][x] = 0;
                println!("Start: ({}, {})", x, y);
            } else if ch == b'E' {
                end_pos = (x, y);
            }
        }
    }
    println!("End: ({}, {})", end_pos.0, end_pos.1);

    while !bfs.is_empty() {
        pong.clear();

        for (x, y, dir) in bfs.drain() {
            let score0 = scores[dir][y][x];
            // rotate
            {
                let new_dir = if dir == 3 { 0 } else { dir + 1 };
                let new_score = score0 + 1000;
                if new_score < scores[new_dir][y][x] {
                    scores[new_dir][y][x] = new_score;
                    pong.insert((x, y, new_dir));
                }
            }
            // rotate
            {
                let new_dir = if dir == 0 { 3 } else { dir - 1 };
                let new_score = score0 + 1000;
                if new_score < scores[new_dir][y][x] {
                    scores[new_dir][y][x] = new_score;
                    pong.insert((x, y, new_dir));
                }
            }

            // move
            {
                let (new_x, new_y) = match dir {
                    0 => (x as i64 + 1, y as i64),
                    1 => (x as i64, y as i64 + 1),
                    2 => (x as i64 - 1, y as i64),
                    3 => (x as i64, y as i64 - 1),
                    _ => unreachable!("Invalid direction: {}", dir),
                };
                if new_x < 0 || new_y < 0 {
                    continue;
                }
                if new_x >= scores[0][0].len() as i64 || new_y >= scores[0].len() as i64 {
                    continue;
                }
                if board[new_y as usize].as_bytes()[new_x as usize] == b'#' {
                    continue;
                }
                let new_score = score0 + 1;
                if new_score < scores[dir][new_y as usize][new_x as usize] {
                    scores[dir][new_y as usize][new_x as usize] = new_score;
                    pong.insert((new_x as usize, new_y as usize, dir));
                }
            }
        }
        std::mem::swap(&mut bfs, &mut pong);
    }

    let mut answer = u64::MAX;
    for dir in 0..4 {
        let s = scores[dir][end_pos.1][end_pos.0];
        answer = answer.min(s);
    }
    println!("Answer: {}", answer);
}
