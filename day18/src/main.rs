use std::fs::read_to_string;

const W: usize = 71;
const H: usize = 71;

const DXDY: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(mut map: Vec<Vec<i32>>) -> i32 {
    let mut bfs = vec![(0, 0)];
    while !bfs.is_empty() {
        let mut pong = Vec::new();
        let cur_step = map[bfs[0].1 as usize][bfs[0].0 as usize];
        for (x, y) in bfs {
            for (dx, dy) in DXDY {
                let new_x = x + dx;
                let new_y = y + dy;
                if new_x < 0 || new_y < 0 {
                    continue;
                }
                if new_x >= W as i32 || new_y >= H as i32 {
                    continue;
                }

                let new_x = new_x as usize;
                let new_y = new_y as usize;
                if map[new_y][new_x] == -1 {
                    continue;
                }
                if map[new_y][new_x] != i32::MAX {
                    continue;
                }

                map[new_y][new_x] = cur_step + 1;
                pong.push((new_x as i32, new_y as i32));
            }
        }

        bfs = pong;
    }

    map[H - 1][W - 1]
}
fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut map = Vec::new();
    {
        let mut row = Vec::new();
        row.resize(W, i32::MAX);
        map.resize(H, row);
    }

    map[0][0] = 0;
    for (lineid, line) in input.lines().enumerate() {
        let nums = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        map[nums[1]][nums[0]] = -1;

        let res = bfs(map.clone());
        println!("{} ({}, {}): {}", lineid, nums[0], nums[1], res);
        if res == i32::MAX {
            break;
        }
    }
}
