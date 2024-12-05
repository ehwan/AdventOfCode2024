use std::collections::HashSet;
use std::fs::read_to_string;
fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut is_order = true;
    let mut order = HashSet::new();
    let mut answer = 0;
    for line in input.lines() {
        if line.is_empty() {
            is_order = false;
            continue;
        }

        if is_order {
            let numbers: Vec<_> = line.split('|').map(|x| x.parse::<i64>().unwrap()).collect();
            order.insert((numbers[0], numbers[1]));
        } else {
            let numbers: Vec<_> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

            let mut invalid = false;
            for i in 0..numbers.len() {
                for j in i + 1..numbers.len() {
                    if order.contains(&(numbers[j], numbers[i])) {
                        invalid = true;
                        break;
                    }
                }
                if invalid {
                    break;
                }
            }

            if invalid {
                continue;
            }

            let middle_index = numbers.len() / 2;
            answer += numbers[middle_index];
        }
    }
    println!("Answer: {}", answer);
}
