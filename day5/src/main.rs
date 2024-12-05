use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn add_order(order: &mut HashMap<i64, HashSet<i64>>, a: i64, b: i64) {
    order.entry(a).or_insert_with(HashSet::new).insert(b);
}

fn cmp_key(order: &HashMap<i64, HashSet<i64>>, a: i64, b: i64) -> std::cmp::Ordering {
    if a == b {
        return std::cmp::Ordering::Equal;
    }

    if let Some(greater_than_a) = order.get(&a) {
        if greater_than_a.contains(&b) {
            return std::cmp::Ordering::Less;
        } else {
            return std::cmp::Ordering::Greater;
        }
    } else {
        return std::cmp::Ordering::Greater;
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut is_order = true;
    let mut order: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut answer = 0;

    for line in input.lines() {
        if line.is_empty() {
            is_order = false;
            continue;
        }

        if is_order {
            let numbers: Vec<_> = line.split('|').map(|x| x.parse::<i64>().unwrap()).collect();
            add_order(&mut order, numbers[0], numbers[1]);
        } else {
            let mut numbers: Vec<_> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

            let mut valid = true;
            for i in 0..numbers.len() - 1 {
                if cmp_key(&order, numbers[i], numbers[i + 1]) != std::cmp::Ordering::Less {
                    valid = false;
                    break;
                }
            }

            if !valid {
                numbers.sort_by(|a, b| cmp_key(&order, *a, *b));
                let middle_index = numbers.len() / 2;
                answer += numbers[middle_index];
            }
        }
    }
    println!("Answer: {}", answer);
}
