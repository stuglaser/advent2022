use std::{cmp::min, collections::VecDeque};

use crate::utils::{Grid, Pt};

const DAY: i32 = 12;

pub fn day12(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let (map, start_pos, end_pos) = {
        let mut height = 0;
        let mut start_pos = Pt::at(0, 0);
        let mut end_pos = Pt::at(0, 0);
        let mut data = Vec::with_capacity(2048);
        for (i, line_str) in input_str.lines().enumerate() {
            for (j, ch) in line_str.as_bytes().iter().enumerate() {
                data.push(match ch {
                    b'S' => {
                        start_pos.set(j as i32, i as i32);
                        b'a' - b'a'
                    }
                    b'E' => {
                        end_pos.set(j as i32, i as i32);
                        b'z' - b'a'
                    }
                    value @ b'a'..=b'z' => value - b'a',
                    _ => unreachable!(),
                });
            }
            height = i + 1;
        }
        (
            Grid {
                rows: height,
                cols: data.len() / height,
                data,
            },
            start_pos,
            end_pos,
        )
    };

    let mut queue = VecDeque::with_capacity(map.rows * map.cols);
    queue.push_back((0, end_pos.clone()));
    let mut seen = Grid::filled(map.rows, map.cols, false);
    let part1;
    let mut part2 = i32::MAX;
    loop {
        let (steps, pt) = queue.pop_front().unwrap();
        if pt == start_pos {
            part1 = steps;
            part2 = min(part2, steps);
            break;
        }

        if !seen[&pt] {
            let height = map[&pt];
            if height == 0 {
                part2 = min(part2, steps);
            }

            if pt.x > 0 {
                let neigh = pt.plus(-1, 0);
                if !seen[&neigh] && map[&neigh] + 1 >= height {
                    queue.push_back((steps + 1, neigh));
                }
            }
            if pt.x + 1 < map.cols as i32 {
                let neigh = pt.plus(1, 0);
                if !seen[&neigh] && map[&neigh] + 1 >= height {
                    queue.push_back((steps + 1, neigh));
                }
            }
            if pt.y > 0 {
                let neigh = pt.plus(0, -1);
                if !seen[&neigh] && map[&neigh] + 1 >= height {
                    queue.push_back((steps + 1, neigh));
                }
            }
            if pt.y + 1 < map.rows as i32 {
                let neigh = pt.plus(0, 1);
                if !seen[&neigh] && map[&neigh] + 1 >= height {
                    queue.push_back((steps + 1, neigh));
                }
            }
            seen[&pt] = true;
        }
    }

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 31 } else { 394 });

    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 29 } else { 388 });
}

const TEST_EXAMPLE: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
