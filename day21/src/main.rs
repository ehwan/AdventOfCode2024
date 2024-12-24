use std::fs::read_to_string;

fn numpad_pos(a: u8) -> (i32, i32) {
    match a {
        b'0' => (1, 0),
        b'1' => (0, 1),
        b'2' => (1, 1),
        b'3' => (2, 1),
        b'4' => (0, 2),
        b'5' => (1, 2),
        b'6' => (2, 2),
        b'7' => (0, 3),
        b'8' => (1, 3),
        b'9' => (2, 3),
        b'A' => (2, 0),
        _ => panic!("Invalid numpad position"),
    }
}
fn dirpad_pos(a: u8) -> (i32, i32) {
    match a {
        b'<' => (0, 0),
        b'v' => (1, 0),
        b'>' => (2, 0),
        b'^' => (1, 1),
        b'A' => (2, 2),
        _ => panic!("Invalid dirpad position"),
    }
}

const NUMPAD_EMPTY: (i32, i32) = (0, 0);
const DIRPAD_EMPTY: (i32, i32) = (0, 1);

fn move_single(from: (i32, i32), to: (i32, i32), empty: (i32, i32)) -> Vec<String> {
    let diff = (to.0 - from.0, to.1 - from.1);
    if diff == (0, 0) {
        return vec!["A".to_string()];
    }
    let xch = if diff.0 > 0 { '>' } else { '<' };
    let ych = if diff.1 > 0 { '^' } else { 'v' };

    let mut ret = Vec::new();
    if diff.0 != 0 {
        let mid = (from.0 + diff.0, from.1);
        if mid != empty {
            let mut pass = String::new();
            for _ in 0..diff.0.abs() {
                pass.push(xch);
            }
            for _ in 0..diff.1.abs() {
                pass.push(ych);
            }
            ret.push(pass);
        }
    }

    if diff.1 != 0 {
        let mid = (from.0, from.1 + diff.1);
        if mid != empty {
            let mut pass = String::new();
            for _ in 0..diff.1.abs() {
                pass.push(ych);
            }
            for _ in 0..diff.0.abs() {
                pass.push(xch);
            }
            ret.push(pass);
        }
    }

    ret
}

fn single_pass(input: &str, empty: (i32, i32), get_pos: impl Fn(u8) -> (i32, i32)) -> Vec<String> {
    let pos = get_pos(b'A');
    for c in input.bytes() {
        let new_pos = get_pos(c);
        let pass = move_single(pos, new_pos, empty);

        bfs = bfs
            .into_iter()
            .flat_map(|b| {
                pass.iter().map(move |p| {
                    let mut new_b = b.clone();
                    new_b.push_str(p);
                    new_b.push('A');
                    new_b
                })
            })
            .collect::<Vec<_>>();

        pos = new_pos;
    }

    let mut ret = Vec::new();
    let mut min_len = None;
    for b in bfs {
        if let Some(ml) = min_len {
            if b.len() < ml {
                min_len = Some(b.len());
                ret.clear();
                ret.push(b);
            } else if b.len() == ml {
                ret.push(b);
            }
        } else {
            min_len = Some(b.len());
            ret.push(b);
        }
    }
    ret
}
fn single_numpad(start: u8, input: &str) -> Vec<String> {
    single_pass(start, input, NUMPAD_EMPTY, numpad_pos)
}
fn single_dirpad(start: u8, input: &str) -> Vec<String> {
    single_pass(start, input, DIRPAD_EMPTY, dirpad_pos)
}

fn parse_int(input: &str) -> u64 {
    let mut ret = 0;
    for ch in input.bytes() {
        if !ch.is_ascii_digit() {
            break;
        }
        let d = (ch as u64) - ('0' as u64);
        ret = ret * 10 + d;
    }
    ret
}

fn main() {
    let input = read_to_string("input1.txt").unwrap();

    let mut answer = 0;
    for line in input.lines() {
        let d = parse_int(line);

        let dir1 = single_numpad(b'5', line);

        let dir2 = dir1
            .into_iter()
            .flat_map(|b| single_dirpad(b'A', &b).into_iter())
            .collect::<Vec<_>>();

        let dir3 = dir2
            .into_iter()
            .flat_map(|b| single_dirpad(b'A', &b).into_iter())
            .collect::<Vec<_>>();

        let len = dir3.into_iter().map(|b| b.len()).min().unwrap();

        println!("{} ", len);
    }
}
