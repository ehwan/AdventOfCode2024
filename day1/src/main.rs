use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in input.lines() {
        let mut digits = line.split_whitespace();
        if let (Some(d1), Some(d2)) = (digits.next(), digits.next()) {
            if let (Ok(d1), Ok(d2)) = (d1.parse::<i32>(), d2.parse::<i32>()) {
                column1.push(d1);
                column2.push(d2);
            }
        }
    }

    // number of appearances of each number in column2
    let mut appearance_count = HashMap::new();
    for d2 in column2.iter() {
        *appearance_count.entry(*d2).or_insert(0) += 1;
    }

    let mut answer: i64 = 0;
    for d1 in column1.iter() {
        let count = appearance_count.get(d1).unwrap_or(&0);
        let score: i64 = (*d1 as i64) * (*count as i64);
        answer += score;
    }

    println!("Answer: {}", answer);
}
