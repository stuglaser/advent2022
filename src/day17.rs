use std::{cmp::max, iter::zip};

use crate::utils::{minmax, Grid};

const DAY: i32 = 17;

#[rustfmt::skip]
const ROCK_STRINGS: [&str; 5] = [
"####",

".#.
###
.#.",

"..#
..#
###",

"#
#
#
#",

"##
##"];

fn parse_rock(string: &str) -> Grid<u8> {
    let lines = string.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let mut grid = Grid::filled(height, width, 0);

    let mut grid_data = Vec::with_capacity(width * height);
    for line in lines.iter().rev() {
        grid_data.extend(line.iter().map(|ch| if *ch == b'#' { 1 } else { 0 }));
    }
    Grid {
        rows: height,
        cols: width,
        data: grid_data,
    }
}

fn extend_rows_to(grid: &mut Grid<u8>, rows: usize, value: u8) {
    grid.data.resize(rows * grid.cols, value);
    grid.rows = rows;
}

fn collides(elev: &[usize], rock: &Grid<u8>, r: i32, c: i32) -> bool {
    if r < 0 || c < 0 || c as usize + rock.cols > elev.len() {
        // Collides with edges
        return true;
    }

    for rock_r in 0..rock.rows {
        for rock_c in 0..rock.cols {
            if rock[(rock_r, rock_c)] > 0 {
                if r as usize + rock_r < elev[c as usize + rock_c] {
                    // Collides with previous pieces
                    return true;
                }
            }
        }
    }
    false
}

fn blit(elev: &mut [usize], rock: &Grid<u8>, r: usize, c: usize, val: u8) {
    for rock_r in 0..rock.rows {
        for rock_c in 0..rock.cols {
            if rock[(rock_r, rock_c)] > 0 {
                let this_elev = r + rock_r + 1;
                elev[c as usize + rock_c] = max(elev[c as usize + rock_c], this_elev);
            }
        }
    }
}

fn fmt_elev(elev: &[usize]) -> String {
    let (min_elev, max_elev) = minmax(elev.iter()).unwrap();
    let mut out = String::with_capacity((elev.len() + 1) * (max_elev - min_elev + 1));

    out.push_str(&format!("{:?}\n", elev));

    let mut row = *max_elev;
    loop {
        for e in elev.iter() {
            out.push(if *e > row { '#' } else { '·' });
        }

        out.push('\n');
        if row == 0 || row == *min_elev {
            break;
        }
        row -= 1;
    }
    out
}

fn visualize(elev: &[usize], rock: &Grid<u8>, rock_r: usize, rock_c: usize) -> String {
    let (min_elev, max_elev) = minmax(elev.iter()).unwrap();
    let max_row = max(*max_elev, rock_r + rock.rows);

    let mut out = String::with_capacity((elev.len() + 1) * (max_row - min_elev + 1));
    for row in (*min_elev..(max_row + 1)).rev() {
        for col in 0..elev.len() {
            let elev_overlaps = elev[col] > row;
            let rock_overlaps = rock_r <= row
                && row < rock_r + rock.rows
                && rock_c <= col
                && col < rock_c + rock.cols
                && rock[(row - rock_r, col - rock_c)] > 0;
            assert!(!(elev_overlaps && rock_overlaps));

            out.push(if elev_overlaps {
                '#'
            } else if rock_overlaps {
                '@'
            } else {
                '·'
            });
        }

        out.push('\n');
    }
    out
}

fn fmt_cave(cave: &Grid<u8>) -> String {
    let mut out = String::with_capacity(cave.rows * (cave.cols + 1));
    for r in (0..cave.rows).rev() {
        for c in 0..cave.cols {
            out.push(match cave[(r, c)] {
                0 => '·',
                1 => '#',
                2 => '@',
                _ => unreachable!(),
            });
        }
        out.push('\n');
    }
    out
}

struct State {
    // cave: Grid<u8>,
    elev: [usize; 7],
    top: usize,
}

impl State {
    fn new() -> Self {
        Self {
            // cave: Grid::filled(1, 7, 0),
            elev: [0; 7],
            top: 0,
        }
    }
}

fn drop_rocks<'a, RockIt, BlowIt>(
    state: &mut State,
    num_rocks: usize,
    rock_iter: &mut RockIt,
    blow_iter: &mut BlowIt,
) where
    RockIt: Iterator<Item = &'a Grid<u8>>,
    BlowIt: Iterator<Item = i8>,
{
    for i in 0..num_rocks {
        let mut drop_c = 2;
        let mut drop_r = state.top as i32 + 3;

        // println!("[{i}] Elevation:\n{}", fmt_elev(&state.elev));

        let rock = rock_iter.next().unwrap();
        // println!("Visualize:\n{}", visualize(&state.elev, rock, drop_r as usize, drop_c as usize));
        loop {
            // Sideways
            let blow = blow_iter.next().unwrap();
            if !collides(&state.elev, rock, drop_r, drop_c + blow as i32) {
                drop_c += blow as i32;
            }
            // println!("[{i}] blown to\n{}", visualize(&state.elev, rock, drop_r as usize, drop_c as usize));

            // Down
            if drop_r == 0 || collides(&state.elev, rock, drop_r - 1, drop_c) {
                // Places the rock
                blit(&mut state.elev, rock, drop_r as usize, drop_c as usize, 1);
                state.top = max(state.top, drop_r as usize + rock.rows);
                break;
            } else {
                drop_r -= 1;
            }

            // println!("[{i}] dropped to\n{}", visualize(&state.elev, rock, drop_r as usize, drop_c as usize));
        }
    }
}

pub fn day17(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let looping_after = input_str.len() * ROCK_STRINGS.len();

    let rocks = ROCK_STRINGS.map(|string| parse_rock(string));

    let mut blow_iter = input_str
        .as_bytes()
        .iter()
        .map(|ch| if *ch == b'<' { -1i8 } else { 1 })
        .cycle();
    let mut rock_iter = rocks.iter().cycle();

    let mut state = State::new();
    drop_rocks(
        &mut state,
        2022,
        &mut rock_iter.clone(),
        &mut blow_iter.clone(),
    );

    // println!("{}", fmt_cave(&state.cave));

    // Up is +row

    let part1 = state.top;
    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 3068 } else { 3206 });

    // In part 2, we drop the repeated part, then figure out how it fits together.
    let mut state_repeated = State::new();
    drop_rocks(
        &mut state_repeated,
        looping_after,
        &mut rock_iter.clone(),
        &mut blow_iter.clone(),
    );
    println!(
        "Looping part is {} high. Approx full height = {}",
        state_repeated.top,
        state_repeated.top * (1000000000000 / looping_after)
    );

    // println!("{}", fmt_cave(&state_repeated.cave));

    // let part2 = best_sofar;
    // if print {
    //     println!("Day {}.  Part 2: {}", DAY, part2);
    // }
    // assert_eq!(part2, if test_mode { 99 } else { 99 });
}

const TEST_EXAMPLE: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

// No longer needed, since the elevation method is more efficient,
// but I liked this code, so I kept it around.
//
// fn collides(cave: &Grid<u8>, rock: &Grid<u8>, r: i32, c: i32) -> bool {
//     if r < 0 || c < 0 || c as usize + rock.cols > cave.cols {
//         return true;
//     }
//     for rock_r in 0..rock.rows {
//         for rock_c in 0..rock.cols {
//             if rock[(rock_r, rock_c)] > 0 && cave[(r as usize + rock_r, c as usize + rock_c)] > 0 {
//                 return true;
//             }
//         }
//     }
//     false
// }
//
// fn blit(cave: &mut Grid<u8>, rock: &Grid<u8>, r: usize, c: usize, val: u8) {
//     for rock_r in 0..rock.rows {
//         for rock_c in 0..rock.cols {
//             if rock[(rock_r, rock_c)] > 0 {
//                 cave[(r as usize + rock_r, c as usize + rock_c)] = val;
//             }
//         }
//     }
// }
//
// fn drop_rocks<'a, RockIt, BlowIt>(
//     state: &mut State,
//     num_rocks: usize,
//     rock_iter: &mut RockIt,
//     blow_iter: &mut BlowIt,
// ) where
//     RockIt: Iterator<Item = &'a Grid<u8>>,
//     BlowIt: Iterator<Item = i8>,
// {
//     state.cave.data.reserve(7 * num_rocks * 3);
//     for i in 0..num_rocks {
//         let mut drop_c: i32 = 2;
//         let mut drop_r: i32 = state.top as i32 + 3;
//
//         let rock = rock_iter.next().unwrap();
//
//         if i % 1_000_000 == 0 {
//             println!("[{}] Dropped from {}, {}", i + 1, drop_r, drop_c);
//         }
//         extend_rows_to(&mut state.cave, drop_r as usize + rock.rows, 0);
//
//         loop {
//             // Sideways
//             let blow = blow_iter.next().unwrap();
//             if !collides(&state.cave, &rock, drop_r, drop_c + blow as i32) {
//                 drop_c += blow as i32;
//             }
//
//             // Down
//             if drop_r >= 0 && collides(&state.cave, &rock, drop_r - 1, drop_c) {
//                 // Place the rock
//                 blit(&mut state.cave, &rock, drop_r as usize, drop_c as usize, 1);
//                 state.top = max(state.top, drop_r as usize + rock.rows);
//                 break;
//             } else {
//                 drop_r -= 1;
//
//             }
//         }
//         // println!("{}", fmt_cave(&cave));
//     }
// }
