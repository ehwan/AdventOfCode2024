use std::fs::read_to_string;

use std::collections::HashSet;

fn check(to: i64, numbers: &[i64]) -> bool {
    let mut stack: HashSet<i64> = HashSet::new();
    stack.insert(numbers[0]);
    for &n in numbers.iter().skip(1) {
        let mut new_stack = HashSet::new();

        let mut factor = 1;
        if n == 0 {
            factor = 10;
        } else {
            let mut n_ = n;
            while n_ > 0 {
                factor *= 10;
                n_ /= 10;
            }
        }

        for s in stack {
            if let Some(add) = s.checked_add(n) {
                new_stack.insert(add);
            }
            if let Some(mul) = s.checked_mul(n) {
                new_stack.insert(mul);
            }

            if let Some(mul) = s.checked_mul(factor) {
                if let Some(add) = mul.checked_add(n) {
                    new_stack.insert(add);
                }
            }
        }
        stack = new_stack;
    }
    stack.contains(&to)
}
fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut answer = 0;
    for line in input.lines() {
        let colon_idx = line.find(':').unwrap();

        let (calibrate_to, numbers) = line.split_at(colon_idx);
        let calibrate_to: i64 = calibrate_to.parse().unwrap();

        let ns = numbers[1..]
            .trim()
            .split(' ')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        if check(calibrate_to, &ns) {
            answer += calibrate_to;
        }
    }
    println!("Answer: {}", answer);
}
