use std::fs::read;

use std::collections::LinkedList;

enum Block {}

fn main() {
    let mut input = read("input1.txt").unwrap();
    let mut list = LinkedList::new();
    for x in input.iter_mut() {
        *x -= b'0';
    }
    let mut moved = Vec::new();
    moved.resize(input.len(), false);

    let mut i = 0;
    let mut answer = 0;

    let mut right_block = input.len() - 1;
    if right_block % 2 != 0 {
        right_block -= 1;
    }

    for left_block in 0..input.len() {
        if left_block % 2 == 0 {
            // file block
            let id = left_block / 2;
            let l = input[left_block];
            if moved[left_block] {
                i += l as u64;
            } else {
                for _ in 0..l {
                    answer += id as u64 * i;
                    i += 1;
                }
            }
        } else {
            // empty block
            let l = input[left_block];
            while right_block > left_block && input[right_block] > l {
                right_block -= 2;
            }
            if right_block > left_block {
                let id = right_block / 2;
                println!("id: {}, i: {}, l: {}", id, i, l);
                let i_next = i + l as u64;
                let l = input[right_block];
                println!("l: {}", l);
                for _ in 0..l {
                    answer += id as u64 * i;
                    i += 1;
                }
                i = i_next;
                moved[right_block] = true;
                right_block -= 2;
            } else {
                i = i + l as u64;
            }
        }
    }

    println!("Answer: {}", answer);
}
