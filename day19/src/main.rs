use std::{collections::HashSet, fs::read_to_string};

fn possible_single(set: &HashSet<&str>, pattern: &str) -> bool {
    set.contains(pattern)
}
fn count(set: &HashSet<&str>, pattern: &str) -> u64 {
    let mut dp = Vec::new();
    dp.resize(pattern.len() + 1, 0);
    dp[0] = 1;
    for (i, _ch) in pattern.bytes().enumerate() {
        let mut c = 0;
        for j in 0..=i {
            if possible_single(set, &pattern[j..=i]) {
                c += dp[j];
            }
        }
        dp[i + 1] = c;
    }

    *dp.last().unwrap()
}

fn main() {
    let input = read_to_string("input.txt").expect("Error reading input file");

    let mut lines = input.lines();
    let mut set = HashSet::new();

    for pattern in lines.next().unwrap().split(", ").collect::<Vec<_>>() {
        println!("{}", pattern);
        set.insert(pattern);
    }

    lines.next(); // empty line

    let mut answer = 0;
    for pattern in lines {
        answer += count(&set, pattern);
    }
    println!("{}", answer);
}
