use std::cmp::max;

use rustc_hash::FxHashMap;

use crate::utils::Grid;

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

const COLORS: [char; 6] = ['·', '#', '@', '▒', '◉', '▢'];
// fn fmt_cave(cave: &Grid<u8>) -> String {
//     let mut out = String::with_capacity(cave.rows * (cave.cols + 1));
//     for r in (0..cave.rows).rev() {
//         for c in 0..cave.cols {
//             out.push(COLORS[cave[(r, c)] as usize]);
//         }
//         out.push('\n');
//     }
//     out
// }

#[derive(Clone)]
struct State {
    cave: Grid<u8>,
    top: usize,
}

impl State {
    fn new() -> Self {
        Self {
            cave: Grid::filled(1, 7, 0),
            top: 0,
        }
    }
}

fn collides(cave: &Grid<u8>, rock: &Grid<u8>, r: i32, c: i32) -> bool {
    if r < 0 || c < 0 || c as usize + rock.cols > cave.cols {
        return true;
    }
    for rock_r in 0..rock.rows {
        for rock_c in 0..rock.cols {
            if rock[(rock_r, rock_c)] > 0 && cave[(r as usize + rock_r, c as usize + rock_c)] > 0 {
                return true;
            }
        }
    }
    false
}

fn blit(cave: &mut Grid<u8>, rock: &Grid<u8>, r: usize, c: usize, val: u8) {
    for rock_r in 0..rock.rows {
        for rock_c in 0..rock.cols {
            if rock[(rock_r, rock_c)] > 0 {
                cave[(r as usize + rock_r, c as usize + rock_c)] = val;
            }
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
    state.cave.data.reserve(7 * num_rocks * 3);
    for i in 0..num_rocks {
        let mut drop_c: i32 = 2;
        let mut drop_r: i32 = state.top as i32 + 3;

        let rock = rock_iter.next().unwrap();

        extend_rows_to(&mut state.cave, drop_r as usize + rock.rows, 0);

        loop {
            // Sideways
            let blow = blow_iter.next().unwrap();
            if !collides(&state.cave, &rock, drop_r, drop_c + blow as i32) {
                drop_c += blow as i32;
            }

            // Down
            if drop_r >= 0 && collides(&state.cave, &rock, drop_r - 1, drop_c) {
                // Places the rock
                // blit(&mut state.cave, &rock, drop_r as usize, drop_c as usize, 1);
                blit(
                    &mut state.cave,
                    &rock,
                    drop_r as usize,
                    drop_c as usize,
                    (i % (COLORS.len() - 1) + 1) as u8,
                );
                state.top = max(state.top, drop_r as usize + rock.rows);

                // println!("[{i}] Placed:\n{}", fmt_cave(&state.cave));
                break;
            } else {
                drop_r -= 1;
            }
        }
        // println!("{}", fmt_cave(&cave));
    }
}

fn hash_crown_occupancy(state: &State, rows: usize) -> usize {
    let start_row = state.top - rows;
    let start_idx = start_row * state.cave.cols;
    let end_idx = state.top * state.cave.cols;

    let mut hash: usize = 432086524387577;
    for d in &state.cave.data[start_idx..end_idx] {
        if *d > 0 {
            hash = hash.overflowing_mul(8274393).0;
            hash = hash.overflowing_add(91751).0;
        } else {
            hash = hash.overflowing_mul(22971).0;
            hash = hash.overflowing_add(99377).0;
        }
    }
    hash
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

    // In part 2, we look for a repeated section, and then simulate the looping.

    // Really we should search for a full blockage, but probably this value is big enough.
    const ASSUME_TRIMMABLE: usize = 50;

    // fn trim_to_top(state: &mut State, rows: usize) {
    //     let discard_rows = state.top - rows;

    //     println!(
    //         "Discarding {} with rows {} and top {}",
    //         discard_rows, state.cave.rows, state.top
    //     );
    //     state.cave.data = state.cave.data
    //         [(discard_rows * state.cave.cols)..(state.top * state.cave.cols)]
    //         .to_vec();
    //     state.top -= discard_rows;
    //     state.cave.rows = rows;
    //     // Dequeue would speed this up.
    // }

    struct Breadcrumb {
        iters: usize,
        top: usize,
    }
    let mut breadcrumbs =
        FxHashMap::<usize, Breadcrumb>::with_capacity_and_hasher(32, Default::default());

    // Looks for where the repititions begin.
    let mut iter_count = 0;
    let mut state = State::new();
    let mut loop_iter_increase = 0;
    let mut loop_top_increase = 0;
    for _k in 0..1000 {
        // println!("Looking for loop {}", k);

        drop_rocks(&mut state, looping_after, &mut rock_iter, &mut blow_iter);
        iter_count += looping_after;

        let crown_hash = hash_crown_occupancy(&state, ASSUME_TRIMMABLE);
        if let Some(seen) = breadcrumbs.get(&crown_hash) {
            loop_iter_increase = iter_count - seen.iters;
            loop_top_increase = state.top - seen.top;
            break;
        }

        breadcrumbs.insert(
            crown_hash,
            Breadcrumb {
                iters: iter_count,
                top: state.top,
            },
        );
    }

    println!(
        "Found loop, with d-iters = {}, d-top = {}",
        loop_iter_increase, loop_top_increase
    );

    const ITERATIONS: usize = 1_000_000_000_000;

    // Now we fake some repetitions
    let mut fake_iter_increase = 0;
    let mut fake_top_increase = 0;
    while iter_count + fake_iter_increase + loop_iter_increase < ITERATIONS {
        fake_iter_increase += loop_iter_increase;
        fake_top_increase += loop_top_increase;
    }

    // And then do any extra iterations
    let left_iterations = ITERATIONS - (iter_count + fake_iter_increase);
    drop_rocks(
        &mut state,
        left_iterations,
        &mut rock_iter.clone(),
        &mut blow_iter.clone(),
    );

    let part2 = state.top + fake_top_increase;
    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(
        part2,
        if test_mode {
            1514285714288
        } else {
            1602881844347
        }
    );
}

const TEST_EXAMPLE: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
