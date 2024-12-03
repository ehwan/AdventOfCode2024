use std::fs::read;

fn parse_digit(s: &[u8], mut i: usize) -> Option<(u64, usize)> {
    let mut num = 0;
    let mut count = 0;

    while let Some(digit) = s.get(i).copied() {
        if digit < b'0' || digit > b'9' {
            break;
        }

        num = num * 10 + (digit - b'0') as u64;
        i += 1;
        count += 1;
    }

    if count > 0 {
        Some((num, i))
    } else {
        None
    }
}

#[test]
fn test_parse_digit() {
    let res1 = parse_digit(b"1234b", 0);
    let res2 = parse_digit(b"12345a", 1);
    let res3 = parse_digit(b"123456", 2);
    let res4 = parse_digit(b"a1234", 0);

    assert_eq!(res1, Some((1234, 4)));
    assert_eq!(res2, Some((2345, 5)));
    assert_eq!(res3, Some((3456, 6)));
    assert_eq!(res4, None);
}

fn main() {
    let input = read("input.txt").unwrap();

    let input: &'_ [u8] = &input;
    let mut answer = 0;
    let mut enabled = true;

    for i in 0..input.len() {
        if input[i..].starts_with("don't()".as_bytes()) {
            enabled = false;
            continue;
        } else if input[i..].starts_with("do()".as_bytes()) {
            enabled = true;
            continue;
        } else if input[i..].starts_with("mul(".as_bytes()) {
            if !enabled {
                continue;
            }

            let Some((num1, idx1)) = parse_digit(input, i + 4) else {
                continue;
            };
            if input.get(idx1) != Some(&b',') {
                continue;
            }

            let Some((num2, idx2)) = parse_digit(input, idx1 + 1) else {
                continue;
            };

            if input.get(idx2) != Some(&b')') {
                continue;
            }

            let result = num1 * num2;
            answer += result;
        }
    }

    println!("{}", answer);
}
