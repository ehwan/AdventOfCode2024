use std::fs::read_to_string;

fn move_obstacle(board: &mut Vec<Vec<u8>>, x: i32, y: i32, dx: i32, dy: i32) -> (i32, i32) {
    assert!(board[y as usize][x as usize] == b'[' && board[y as usize][x as usize + 1] == b']');

    // move along x-axis
    if dx == 1 {
        match board[y as usize][x as usize + 2] {
            b'#' => {
                // Wall
                // do nothing
                (x, y)
            }
            b'.' => {
                // Empty
                // move forward

                board[y as usize][x as usize] = b'.';
                board[y as usize][x as usize + 1] = b'[';
                board[y as usize][x as usize + 2] = b']';

                (x + 1, y)
            }
            b'[' => {
                // Obstacle

                // try move obstacle
                let moved = move_obstacle(board, x + 2, y, dx, dy);
                if moved != (x + 2, y) {
                    // obstacle moved
                    // move forward
                    board[y as usize][x as usize] = b'.';
                    board[y as usize][x as usize + 1] = b'[';
                    board[y as usize][x as usize + 2] = b']';
                    (x + 1, y)
                } else {
                    // obstacle can't move
                    // do nothing
                    (x, y)
                }
            }
            _ => unreachable!(
                "Invalid character: {}",
                board[y as usize][x as usize + 2] as char
            ),
        }
    } else if dx == -1 {
        match board[y as usize][x as usize - 1] {
            b'#' => {
                // Wall
                // do nothing
                (x, y)
            }
            b'.' => {
                // Empty
                // move forward

                board[y as usize][x as usize - 1] = b'[';
                board[y as usize][x as usize] = b']';
                board[y as usize][x as usize + 1] = b'.';

                (x - 1, y)
            }
            b']' => {
                // Obstacle

                // try move obstacle
                let moved = move_obstacle(board, x - 2, y, dx, dy);
                if moved != (x - 2, y) {
                    // obstacle moved
                    // move forward
                    board[y as usize][x as usize - 1] = b'[';
                    board[y as usize][x as usize] = b']';
                    board[y as usize][x as usize + 1] = b'.';
                    (x - 1, y)
                } else {
                    // obstacle can't move
                    // do nothing
                    (x, y)
                }
            }
            _ => unreachable!(
                "Invalid character: {}",
                board[y as usize][x as usize - 1] as char
            ),
        }
    } else {
        // '[' pos
        let board0 = board.clone();
        match board[(y + dy) as usize][x as usize] {
            b'#' => return (x, y),
            b'.' => {}
            b'[' => {
                let moved = move_obstacle(board, x, y + dy, dx, dy);
                if moved == (x, y + dy) {
                    *board = board0;
                    return (x, y);
                }
            }
            b']' => {
                let moved = move_obstacle(board, x - 1, y + dy, dx, dy);
                if moved == (x - 1, y + dy) {
                    *board = board0;
                    return (x, y);
                }
            }
            _ => unreachable!(
                "Invalid character: {}",
                board[(y + dy) as usize][x as usize] as char
            ),
        }
        // ']' pos
        match board[(y + dy) as usize][(x + 1) as usize] {
            b'#' => {
                *board = board0;
                return (x, y);
            }
            b'.' => {}
            b'[' => {
                let moved = move_obstacle(board, x + 1, y + dy, dx, dy);
                if moved == (x + 1, y + dy) {
                    *board = board0;
                    return (x, y);
                }
            }
            _ => unreachable!(
                "Invalid character: {}",
                board[(y + dy) as usize][x as usize] as char
            ),
        }

        // move forward
        board[y as usize][x as usize] = b'.';
        board[y as usize][x as usize + 1] = b'.';
        board[(y + dy) as usize][x as usize] = b'[';
        board[(y + dy) as usize][x as usize + 1] = b']';
        (x, y + dy)
    }
}
fn move_character(board: &mut Vec<Vec<u8>>, x: i32, y: i32, dx: i32, dy: i32) -> (i32, i32) {
    match board[(y + dy) as usize][(x + dx) as usize] {
        b'#' => {
            // Wall
            // do nothing
            (x, y)
        }
        b'.' => {
            // Empty
            // move forward

            board[(y + dy) as usize][(x + dx) as usize] = b'@';
            board[y as usize][x as usize] = b'.';

            (x + dx, y + dy)
        }
        b'[' => {
            // Obstacle

            // try move obstacle
            let moved = move_obstacle(board, x + dx, y + dy, dx, dy);
            if moved != (x + dx, y + dy) {
                // obstacle moved
                // move forward
                board[(y + dy) as usize][(x + dx) as usize] = b'@';
                board[y as usize][x as usize] = b'.';
                (x + dx, y + dy)
            } else {
                // obstacle can't move
                // do nothing
                (x, y)
            }
        }
        b']' => {
            // Obstacle

            // try move obstacle
            let moved = move_obstacle(board, x + dx - 1, y + dy, dx, dy);
            if moved != (x + dx - 1, y + dy) {
                // obstacle moved
                // move forward
                board[(y + dy) as usize][(x + dx) as usize] = b'@';
                board[y as usize][x as usize] = b'.';
                (x + dx, y + dy)
            } else {
                // obstacle can't move
                // do nothing
                (x, y)
            }
        }
        _ => {
            unreachable!(
                "Invalid character: {}",
                board[(y + dy) as usize][(x + dx) as usize] as char
            );
        }
    }
}

fn print_board(board: &Vec<Vec<u8>>) {
    for row in board.iter() {
        for ch in row.iter() {
            print!("{}", *ch as char);
        }
        println!();
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut line_iter = input.lines();
    let mut board = Vec::new();
    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        board.push(
            line.bytes()
                .flat_map(|ch| match ch {
                    b'#' => [b'#'; 2].into_iter(),
                    b'.' => [b'.'; 2].into_iter(),
                    b'O' => [b'[', b']'].into_iter(),
                    b'@' => [b'@', b'.'].into_iter(),
                    _ => unreachable!("Invalid character: {}", ch as char),
                })
                .collect::<Vec<_>>(),
        );
    }

    let mut x = 0;
    let mut y = 0;
    for (yy, row) in board.iter().enumerate() {
        for (xx, ch) in row.iter().enumerate() {
            if *ch == b'@' {
                x = xx as i32;
                y = yy as i32;
            }
        }
    }

    // println!("Initial position: ({}, {})", x, y);
    // print_board(&board);

    for commands in line_iter {
        for ch in commands.bytes() {
            match ch {
                b'<' => {
                    // Move left
                    (x, y) = move_character(&mut board, x, y, -1, 0);
                }
                b'>' => {
                    // Move right
                    (x, y) = move_character(&mut board, x, y, 1, 0);
                }
                b'^' => {
                    // Move up
                    (x, y) = move_character(&mut board, x, y, 0, -1);
                }
                b'v' => {
                    // Move down
                    (x, y) = move_character(&mut board, x, y, 0, 1);
                }
                _ => unreachable!("Invalid command: {}", ch as char),
            }

            // println!("Command: {}", ch as char);
            // print_board(&board);
        }
    }

    let mut answer = 0;
    for (yy, row) in board.iter().enumerate() {
        for (xx, ch) in row.iter().enumerate() {
            if *ch == b'[' {
                let gps = yy * 100 + xx;
                answer += gps;
            }
        }
    }
    println!("Answer: {}", answer);
}
