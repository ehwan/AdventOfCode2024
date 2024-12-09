use std::fs::read;

struct Pos<'a> {
    i: usize,
    block_i: usize,
    block_local_i: usize,
    input: &'a [u8],
}
impl<'a> Pos<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            i: 0,
            block_i: 0,
            block_local_i: 0,
            input,
        }
    }
    fn new_tail(input: &'a [u8]) -> Self {
        let mut i = 0;
        for &n in input.iter() {
            i += n as usize;
        }
        i -= 1;
        println!("last: {}", input.last().unwrap());
        let mut ret = Self {
            i,
            block_i: input.len() - 1,
            block_local_i: *input.last().unwrap() as usize - 1,
            input,
        };
        while !ret.is_file() {
            ret.to_prev();
        }
        ret
    }

    fn is_file(&self) -> bool {
        self.block_i % 2 == 0
    }
    fn position(&self) -> usize {
        self.i
    }
    fn file_id(&self) -> Option<usize> {
        if self.is_file() {
            Some(self.block_i / 2)
        } else {
            None
        }
    }

    fn to_next(&mut self) {
        if self.block_local_i == self.input[self.block_i] as usize - 1 {
            self.block_i += 1;
            while self.input[self.block_i] == 0 {
                self.block_i += 1;
            }
            self.block_local_i = 0;
        } else {
            self.block_local_i += 1;
        }
        self.i += 1;
    }
    fn to_prev(&mut self) {
        if self.block_local_i == 0 {
            self.block_i -= 1;
            while self.input[self.block_i] == 0 {
                self.block_i -= 1;
            }
            self.block_local_i = self.input[self.block_i] as usize - 1;
        } else {
            self.block_local_i -= 1;
        }
        self.i -= 1;
    }
}

fn main() {
    let mut input = read("input.txt").unwrap();
    for x in input.iter_mut() {
        if !(b'0'..=b'9').contains(x) {
            break;
        }
        *x -= b'0';
    }
    let l = input.len();

    let mut first = Pos::new(&input[..l - 1]);
    let mut last = Pos::new_tail(&input[..l - 1]);

    let mut answer = 0;
    while first.i <= last.i {
        if first.is_file() {
            let i = first.i as u64;
            let id = first.file_id().unwrap() as u64;
            answer += i * id;
            first.to_next();
        } else {
            let i = first.i as u64;
            let id = last.file_id().unwrap() as u64;
            answer += i * id;

            first.to_next();
            last.to_prev();
            while !last.is_file() {
                last.to_prev();
            }
        }
    }
    println!("");
    println!("{}", answer);
}
