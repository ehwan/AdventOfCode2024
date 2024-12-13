use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut lines_iter = input.lines();

    let mut answer = 0;
    while let Some(button_a_line) = lines_iter.next() {
        let button_b_line = lines_iter.next().unwrap();
        let prize_line = lines_iter.next().unwrap();
        lines_iter.next();

        let button_a = {
            let split = button_a_line
                .split(' ')
                .skip(2)
                .take(2)
                .map(|s| {
                    let s = if *s.as_bytes().last().unwrap() == b',' {
                        &s[2..s.len() - 1]
                    } else {
                        &s[2..]
                    };
                    s.parse::<i64>().unwrap()
                })
                .collect::<Vec<_>>();
            (split[0], split[1])
        };
        let button_b = {
            let split = button_b_line
                .split(' ')
                .skip(2)
                .take(2)
                .map(|s| {
                    let s = if *s.as_bytes().last().unwrap() == b',' {
                        &s[2..s.len() - 1]
                    } else {
                        &s[2..]
                    };
                    s.parse::<i64>().unwrap()
                })
                .collect::<Vec<_>>();
            (split[0], split[1])
        };

        const PRIZE_OFFSET: i64 = 10000000000000;
        let prize = {
            let split = prize_line
                .split(' ')
                .skip(1)
                .take(2)
                .map(|s| {
                    let s = if *s.as_bytes().last().unwrap() == b',' {
                        &s[2..s.len() - 1]
                    } else {
                        &s[2..]
                    };
                    s.parse::<i64>().unwrap()
                })
                .collect::<Vec<_>>();
            (split[0] + PRIZE_OFFSET, split[1] + PRIZE_OFFSET)
        };

        /*
        button_a * t + button_b * s = prize
        (t, s >= 0, integer)

        ( ax bx ) ( t ) = ( px )
        ( ay by ) ( s )   ( py )

        assume there are no parallel lines; Deterninant != 0

        ( t ) = 1/Det ( by -bx ) ( px )
        ( s )         (-ay  ax ) ( py )
        */

        let mut det = button_a.0 * button_b.1 - button_a.1 * button_b.0;
        if det == 0 {
            unreachable!("No solution");
        }

        let mut tt = button_b.1 * prize.0 - button_b.0 * prize.1;
        let mut ss = -button_a.1 * prize.0 + button_a.0 * prize.1;
        if det < 0 {
            det = -det;
            tt = -tt;
            ss = -ss;
        }

        if tt < 0 || ss < 0 {
            continue;
        }

        if tt % det != 0 || ss % det != 0 {
            continue;
        }

        tt /= det;
        ss /= det;

        let tokens = tt * 3 + ss;
        answer += tokens;
    }

    println!("Answer: {}", answer);
}
