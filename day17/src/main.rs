use std::{collections::HashSet, fs::read_to_string};

fn parse_register(line: &str) -> i64 {
    line.split(':').nth(1).unwrap().trim().parse().unwrap()
}

fn combo(op: u8, a: i64, b: i64, c: i64) -> i64 {
    match op {
        0..=3 => op as i64,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!("Invalid operand: {}", op),
    }
}

fn execute(program: &[u8], mut register_a: i64, mut register_b: i64, mut register_c: i64) -> bool {
    let mut counter = 0;
    let mut outputs = Vec::new();

    while let Some(op) = program.get(counter).copied() {
        let Some(operand) = program.get(counter + 1).copied() else {
            unreachable!("No operand for {}", op);
        };
        counter += 2;
        match op {
            // adv
            0 => {
                let num = register_a;
                let denom = 1i64 << combo(operand, register_a, register_b, register_c);
                register_a = num / denom;
            }
            // bxl
            1 => {
                register_b = register_b ^ operand as i64;
            }
            // bst
            2 => {
                let operand = combo(operand, register_a, register_b, register_c);
                register_b = (operand % 8) as i64;
            }
            // jnz
            3 => {
                if register_a != 0 {
                    counter = operand as usize;
                }
            }
            // bxc
            4 => {
                let res = register_b ^ register_c;
                register_b = res;
            }
            //
            5 => {
                let operand = combo(operand, register_a, register_b, register_c) % 8;
                outputs.push(operand);
            }
            6 => {
                let num = register_a;
                let denom = 1i64 << combo(operand, register_a, register_b, register_c);
                register_b = num / denom;
            }
            7 => {
                let num = register_a;
                let denom = 1i64 << combo(operand, register_a, register_b, register_c);
                register_c = num / denom;
            }
            _ => unreachable!("Invalid opcode: {}", op),
        }
    }
    if let Some(z) = outputs.get(0) {
        println!("{}", z);
    } else {
        println!("None");
    }
    outputs.len() == program.len()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut line_iter = input.lines();
    let register_a = parse_register(line_iter.next().unwrap());
    let register_b = parse_register(line_iter.next().unwrap());
    let register_c = parse_register(line_iter.next().unwrap());
    line_iter.next();

    let program = line_iter.next().unwrap().split(':').nth(1).unwrap().trim();
    let program = program
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    /*
    This code is extremely hard-coded. It is not designed to solve any general inputs.
    B = A & 0b111 ; 2, 4
    B = B ^ 0b101 ; 1, 5
    C = A >> B    ; 7, 5
    B = B^0b110   ; 1, 6
    A = A/8       ; 0, 3
    B = (B^C) & 0b111 ; 4, 0
    OUT B         ; 5, 5
    jnz A, 0      ; 3, 0

    B = (A & 0b111)
    B = B ^ 0b101
    */

    const A_MAP: [(u64, u64); 8] = [
        (0b011, 5),
        (0b010, 4),
        (0b001, 7),
        (0b000, 6),
        (0b111, 1),
        (0b110, 0),
        (0b101, 3),
        (0b100, 2),
    ];

    #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
    struct RegisterA {
        bits: [Option<bool>; 64],
    }
    impl RegisterA {
        fn new() -> Self {
            Self { bits: [None; 64] }
        }

        fn set_bit(&self, offset: usize, mut val: u64) -> Option<Self> {
            let mut new_bits = self.bits;
            for i in 0..3 {
                let v = (val & 1) != 0;
                if new_bits[offset + i] == Some(!v) {
                    return None;
                } else {
                    new_bits[offset + i] = Some(v);
                }
                val >>= 1;
            }
            Some(Self { bits: new_bits })
        }
        fn to_u64(&self) -> u64 {
            let mut res = 0;
            for i in 0..64 {
                if let Some(true) = self.bits[i] {
                    res |= 1 << i;
                }
            }
            res
        }
    }

    let mut bfs = HashSet::new();
    let mut pong = HashSet::new();
    bfs.insert(RegisterA::new());
    for (p_idx, p) in program.into_iter().enumerate() {
        println!("p_idx: {}, p: {}", p_idx, p);
        println!("bfs.len(): {}", bfs.len());
        let offset = p_idx * 3;
        pong.clear();
        for (map_idx, (xor, shift)) in A_MAP.into_iter().enumerate() {
            let shift = offset as u64 + shift;
            let make = (p as u64) ^ xor;

            for b in bfs.iter() {
                let Some(b) = b.set_bit(offset, map_idx as u64) else {
                    continue;
                };
                let Some(new_b) = b.set_bit(shift as usize, make) else {
                    continue;
                };
                pong.insert(new_b);
            }
        }
        std::mem::swap(&mut bfs, &mut pong);
    }

    let mut numbers = bfs.into_iter().map(|b| b.to_u64()).collect::<Vec<_>>();
    numbers.sort();

    for n in numbers {
        println!("{}", n);
    }
}
