use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

const DIDJ: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let map = input.lines().collect::<Vec<_>>();

    let mut dist = Vec::new();
    dist.resize_with(map.len(), || {
        let mut v = Vec::new();
        v.resize(map[0].len(), (0u64, 0u64));
        v
    });

    let mut start_i = 0;
    let mut start_j = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, ch) in row.bytes().enumerate() {
            if ch == b'S' {
                start_i = i;
                start_j = j;
                break;
            }
        }
    }

    let mut bfs = Vec::new();
    bfs.push((start_i, start_j, 1));
}
