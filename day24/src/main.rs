use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, Clone, Copy)]
enum Node<'a> {
    Value(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

fn solve<'a>(nodes: &mut HashMap<&str, Node<'a>>, name: &str) -> bool {
    let n = *nodes.get(name).unwrap();
    let res = match n {
        Node::Value(val) => val,
        Node::And(lhs, rhs) => {
            let lhs = solve(nodes, lhs);
            let rhs = solve(nodes, rhs);
            lhs & rhs
        }
        Node::Or(lhs, rhs) => {
            let lhs = solve(nodes, lhs);
            let rhs = solve(nodes, rhs);
            lhs | rhs
        }
        Node::Xor(lhs, rhs) => {
            let lhs = solve(nodes, lhs);
            let rhs = solve(nodes, rhs);
            lhs ^ rhs
        }
    };
    *nodes.get_mut(name).unwrap() = Node::Value(res);

    res
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut line_iter = input.lines();
    let mut nodes = HashMap::new();
    let mut names = Vec::new();
    for line in &mut line_iter {
        if line.is_empty() {
            break;
        }

        let mut it = line.split(':');
        let name = it.next().unwrap();
        let val = it.next().unwrap().trim().parse::<u8>().unwrap() == 1;

        nodes.insert(name, Node::Value(val));
        names.push(name);
    }

    for line in line_iter {
        let mut it = line.split_whitespace();

        let lhs = it.next().unwrap();
        let op = it.next().unwrap();
        let rhs = it.next().unwrap();
        it.next(); // "->"
        let parent_name = it.next().unwrap();

        names.push(parent_name);
        match op {
            "AND" => {
                nodes.insert(parent_name, Node::And(lhs, rhs));
            }
            "OR" => {
                nodes.insert(parent_name, Node::Or(lhs, rhs));
            }
            "XOR" => {
                nodes.insert(parent_name, Node::Xor(lhs, rhs));
            }
            _ => unreachable!("Unknown operator: {}", op),
        }
    }

    let mut answer: u64 = 0;
    for name in names {
        let res = solve(&mut nodes, name);

        if name.starts_with("z") {
            println!("{}: {}", name, res);
            let bit_num = name[1..].parse::<u8>().unwrap();
            if res {
                answer |= 1 << bit_num;
            }
        }
    }
    println!("Answer: {}", answer);
}
