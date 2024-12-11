use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum ChangeResult {
    One(u64),
    Two(u64, u64),
}

fn change(num: u64) -> ChangeResult {
    if num == 0 {
        ChangeResult::One(1)
    } else {
        let mut digit_count = 0;

        {
            let mut n = num;
            while n != 0 {
                digit_count += 1;
                n /= 10;
            }
        }
        if digit_count % 2 == 0 {
            let half_digit = digit_count / 2;
            let mut right_half = 0;
            let mut base = 1;
            let mut n = num;
            for _ in 0..half_digit {
                right_half += (n % 10) * base;
                n /= 10;
                base *= 10;
            }
            let left_half = n;

            ChangeResult::Two(left_half, right_half)
        } else {
            ChangeResult::One(num * 2024)
        }
    }
}

#[test]
fn test_change() {
    assert_eq!(change(0), ChangeResult::One(1));
    assert_eq!(change(1), ChangeResult::One(2024));
    assert_eq!(change(2), ChangeResult::One(4048));
    assert_eq!(change(14), ChangeResult::Two(1, 4));
    assert_eq!(change(141), ChangeResult::One(141 * 2024));
    assert_eq!(change(1456), ChangeResult::Two(14, 56));
}

struct Machine {
    cache: HashMap<(u64, usize), HashMap<u64, usize>>,
}

impl Machine {
    fn new() -> Self {
        Self {
            cache: Default::default(),
        }
    }

    fn solve(&mut self, number: u64, count: usize) -> HashMap<u64, usize> {
        match self.cache.get(&(number, count)) {
            Some(x) => return x.clone(),
            None => match count {
                0 => HashMap::from([(number, 1)]),
                1 => match change(number) {
                    ChangeResult::One(x) => HashMap::from([(x, 1)]),
                    ChangeResult::Two(x, y) => {
                        if x == y {
                            HashMap::from([(x, 2)])
                        } else {
                            HashMap::from([(x, 1), (y, 1)])
                        }
                    }
                },
                _ => {
                    let res = self.solve(number, count - 1);
                    let mut res2 = HashMap::new();
                    for (n, count) in res.iter() {
                        match change(*n) {
                            ChangeResult::One(x) => {
                                *res2.entry(x).or_insert(0) += *count;
                            }
                            ChangeResult::Two(x, y) => {
                                *res2.entry(x).or_insert(0) += *count;
                                *res2.entry(y).or_insert(0) += *count;
                            }
                        }
                    }
                    self.cache.insert((number, count), res2.clone());
                    res2
                }
            },
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let numbers: Vec<u64> = input
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut answer = 0;
    let mut machine = Machine::new();
    for n in numbers.iter() {
        let res = machine.solve(*n, 75);
        for (_, count) in res.into_iter() {
            answer += count;
        }
    }
    println!("Answer: {}", answer);
}
