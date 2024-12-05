use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn add_order(order: &mut HashMap<i64, HashSet<i64>>, a: i64, b: i64) {
    if !order.entry(a).or_insert_with(HashSet::new).insert(b) {
        return;
    }
    if let Some(greater_than_b) = order.get(&b).cloned() {
        for c in greater_than_b {
            add_order(order, a, c);
        }
    }
}

fn get_key(order: &HashMap<i64, HashSet<i64>>, num: i64) -> i64 {
    order.get(&num).map(|x| x.len() as i64).unwrap_or(0)
}
fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut is_order = true;
    let mut order: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut answer = 0;

    for line in input.lines() {
        if line.is_empty() {
            is_order = false;

            for (k, v) in &order {
                println!("{}: {:?}", k, v.len());
            }
            continue;
        }

        if is_order {
            let numbers: Vec<_> = line.split('|').map(|x| x.parse::<i64>().unwrap()).collect();
            add_order(&mut order, numbers[0], numbers[1]);
        } else {
            let mut numbers: Vec<_> = line.split(',').map(|x| x.parse::<i64>().unwrap()).collect();

            let mut valid = true;
            for i in 0..numbers.len() - 1 {
                let k1 = get_key(&order, numbers[i]);
                let k2 = get_key(&order, numbers[i + 1]);
                if k2 <= k1 {
                    valid = false;
                    break;
                }
            }

            if !valid {
                numbers
                    .sort_by(|a, b| get_key(&order, *a).cmp(&get_key(&order, *b)).then(a.cmp(b)));
                let middle_index = numbers.len() / 2;
                answer += numbers[middle_index];
            }
        }
    }
    println!("Answer: {}", answer);
}
