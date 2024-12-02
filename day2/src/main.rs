use std::collections::HashSet;
use std::fs::read_to_string;

fn is_safe(nums: &[i32]) -> bool {
    if nums.len() < 2 {
        return true;
    }

    // nums.len() >= 2

    // Option<i32> : tail number
    // bool : is single number deleted
    // Option<bool> : is increasing, `None` means not determined
    let mut bfs = HashSet::new();
    let mut pong = HashSet::new();

    bfs.insert((None, false, None));

    for n in nums.iter().copied() {
        pong.clear();
        for (tail, deleted, is_inc) in bfs.drain() {
            // insert
            match tail {
                None => {
                    pong.insert((Some(n), deleted, None));
                }
                Some(tail) => {
                    let diff = n - tail;
                    if diff > 0 && diff <= 3 {
                        if is_inc == None || is_inc == Some(true) {
                            pong.insert((Some(n), deleted, Some(true)));
                        }
                    } else if diff < 0 && diff >= -3 {
                        if is_inc == None || is_inc == Some(false) {
                            pong.insert((Some(n), deleted, Some(false)));
                        }
                    }
                }
            }

            // skip
            if !deleted {
                pong.insert((tail, true, is_inc));
            }
        }

        std::mem::swap(&mut bfs, &mut pong);
    }
    !bfs.is_empty()
}
fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut safe_count = 0;
    for line in input.lines() {
        let digits: Result<Vec<_>, _> = line
            .split_whitespace()
            .map(|digit_string| digit_string.parse::<i32>())
            .collect();
        let digits = digits.unwrap();
        if is_safe(&digits) {
            safe_count += 1;
        }
    }

    println!("Safe count: {}", safe_count);
}
