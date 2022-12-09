use rustc_hash::FxHashSet;

use crate::utils::Pt;

const DAY: i32 = 9;

enum Dir {
    R,
    U,
    L,
    D,
}

fn snap(lead: &Pt, follow: &Pt) -> Pt {
    let move_follow = (lead.x - follow.x).abs() > 1 || (lead.y - follow.y).abs() > 1;
    if move_follow {
        Pt::at(
            follow.x + (lead.x - follow.x).signum(),
            follow.y + (lead.y - follow.y).signum(),
        )
    } else {
        follow.clone()
    }
}

pub fn day09(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let moves = {
        let mut moves = Vec::<(Dir, i32)>::with_capacity(256);
        for line in input_str.lines() {
            let mut pieces = line.split(" ");
            let dir = match pieces.next().unwrap() {
                "R" => Dir::R,
                "L" => Dir::L,
                "U" => Dir::U,
                "D" => Dir::D,
                _ => unreachable!(),
            };
            moves.push((dir, pieces.next().unwrap().parse().unwrap()));
        }
        moves
    };

    let mut head = Pt::at(0, 0);
    let mut tail = Pt::at(0, 0);
    let mut tail_been = FxHashSet::<Pt>::with_capacity_and_hasher(2048, Default::default());

    for (dir, distance) in &moves {
        for _ in 0..*distance {
            match dir {
                Dir::R => head.x += 1,
                Dir::U => head.y += 1,
                Dir::L => head.x -= 1,
                Dir::D => head.y -= 1,
            }

            tail = snap(&head, &tail);

            tail_been.insert(tail.clone());
        }
    }

    let part1 = tail_been.len();
    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 13 } else { 6236 });

    let mut rope = vec![Pt::at(0, 0); 10];
    let mut tail_been = FxHashSet::<Pt>::with_capacity_and_hasher(2048, Default::default());

    for (dir, distance) in &moves {
        for _ in 0..*distance {
            match dir {
                Dir::R => rope[0].x += 1,
                Dir::U => rope[0].y += 1,
                Dir::L => rope[0].x -= 1,
                Dir::D => rope[0].y -= 1,
            }

            for i in 0..(rope.len() - 1) {
                rope[i + 1] = snap(&rope[i], &rope[i + 1]);
            }

            tail_been.insert(rope.last().unwrap().clone());
        }
    }

    let part2 = tail_been.len();
    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 1 } else { 2449 });
}

const TEST_EXAMPLE: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
