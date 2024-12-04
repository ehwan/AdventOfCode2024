use std::fs::read_to_string;

fn check_xmas(rows: &[&str], i: usize, j: usize) -> i32 {
    // right down

    let mut right_down = false;
    if rows[i].as_bytes()[j] == b'M'
        && rows[i + 1].as_bytes()[j + 1] == b'A'
        && rows[i + 2].as_bytes()[j + 2] == b'S'
    {
        right_down = true;
    } else if rows[i].as_bytes()[j] == b'S'
        && rows[i + 1].as_bytes()[j + 1] == b'A'
        && rows[i + 2].as_bytes()[j + 2] == b'M'
    {
        right_down = true;
    }

    if !right_down {
        return 0;
    }

    if rows[i + 2].as_bytes()[j] == b'M'
        && rows[i + 1].as_bytes()[j + 1] == b'A'
        && rows[i + 0].as_bytes()[j + 2] == b'S'
    {
        return 1;
    } else if rows[i + 2].as_bytes()[j] == b'S'
        && rows[i + 1].as_bytes()[j + 1] == b'A'
        && rows[i + 0].as_bytes()[j + 2] == b'M'
    {
        return 1;
    }
    return 0;
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let rows: Vec<_> = input.lines().collect();

    let mut answer = 0;
    for i in 0..rows.len() - 2 {
        for j in 0..rows[i].len() - 2 {
            answer += check_xmas(&rows, i, j);
        }
    }
    println!("Answer: {}", answer);
}
