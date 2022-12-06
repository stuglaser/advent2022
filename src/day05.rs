const DAY: i32 = 5;


#[allow(dead_code)]
fn show_stacks(stacks: &Vec<Vec<u8>>) {
    for stack in stacks {
        println!("{}", String::from_utf8_lossy(stack));
    }
}

fn do_move(stacks: &mut Vec<Vec<u8>>, src: i32, dst: i32) {
    let k = stacks[(src - 1) as usize].pop().unwrap();
    stacks[(dst - 1) as usize].push(k);
}

#[derive(Debug)]
struct Action {
    num: i32,
    src: i32,
    dst: i32,
}

pub fn day05(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let (stacks_input, moves) = {
        let mut stacks = Vec::<Vec<u8>>::with_capacity(10);
        let mut moves = Vec::with_capacity(64);

        let mut parsing_containers = true;
        for line_str in input_str.lines() {
            let line = line_str.as_bytes();
            if parsing_containers {
                if line.is_empty() {
                    parsing_containers = false;
                } else if line[1] == b'1' {
                } else {
                    if stacks.is_empty() {
                        stacks.resize((line.len() + 1) / 4, Vec::with_capacity(16));
                    }

                    for (i, stack) in stacks.iter_mut().enumerate() {
                        let container = line[4 * i + 1];
                        if container != b' ' {
                            stack.push(container);
                        }
                    }
                }
            } else {
                let mut pieces = line_str.split(" ");
                pieces.next().unwrap(); // move
                let num = pieces.next().unwrap().parse().unwrap();
                pieces.next().unwrap(); // from
                let src = pieces.next().unwrap().parse().unwrap();
                pieces.next().unwrap(); // to
                let dst = pieces.next().unwrap().parse().unwrap();

                moves.push(Action { num, src, dst });
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }
        (stacks, moves)
    };

    // println!("Parsed stacks:");
    // show_stacks(&stacks_input);
    // println!("\n");

    let mut stacks = stacks_input.clone();
    for action in &moves {
        for _ in 0..action.num {
            do_move(&mut stacks, action.src, action.dst);
        }
    }

    let part1: String = stacks
        .iter()
        .map(|stack| *stack.last().unwrap() as char)
        .collect();

    let mut stacks = stacks_input.clone();
    for action in &moves {
        let idx = stacks[action.src as usize - 1].len() - action.num as usize;

        // Unpleasant copy, since the borrow checker is challenging here.
        let moving = stacks[action.src as usize - 1][idx..].to_owned();
        stacks[action.dst as usize - 1].extend_from_slice(&moving);
        stacks[action.src as usize - 1].truncate(idx);
    }

    let part2: String = stacks
        .iter()
        .map(|stack| *stack.last().unwrap() as char)
        .collect();

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { "CMZ" } else { "GFTNRBZPF" });

    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { "MCD" } else { "VRQWPDSGP" });
}

const TEST_EXAMPLE: &'static str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
