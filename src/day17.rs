use std::{cmp::max, iter::zip};

use rustc_hash::FxHashMap;

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

const COLORS: [char; 6] = ['·', '#', '@', '▒', '◉', '▢'];

fn fmt_cave(cave: &Grid<u8>) -> String {
    let mut out = String::with_capacity(cave.rows * (cave.cols + 1));
    for r in (0..cave.rows).rev() {
        for c in 0..cave.cols {
            out.push(COLORS[cave[(r, c)] as usize]);
        }
        out.push('\n');
    }
    out
}

struct State {
    cave: Grid<u8>,
    bottom: usize,
    top: usize,
}

impl State {
    fn new() -> Self {
        Self {
            cave: Grid::filled(1, 7, 0),
            bottom: 0,
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
                blit(&mut state.cave, &rock, drop_r as usize, drop_c as usize, (i % (COLORS.len() - 1) + 1) as u8);
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
    println!("Looping part is {} high. Approx full height = {}", state_repeated.top, state_repeated.top * (1000000000000 / looping_after));

    let mut state = State::new();

    // Really we should search for a full blockage, but probably this value is big enough.
    const ASSUME_TRIMMABLE: usize = 100;

    fn clone_rows(grid: &Grid<u8>, start: usize, end: usize) -> Grid<u8> {
        Grid{
            rows: end - start,
            cols: grid.cols,
            data: grid.data[(start * grid.cols)..(end * grid.cols)].to_vec()}
    }

    fn hash_occupancy(grid: &Grid<u8>) -> usize {
        let mut hash: usize = 432086524387577;
        for d in &grid.data {
            if *d > 0 {
                hash = hash.overflowing_mul(8274393).0;
                hash = hash.overflowing_add(91832751).0;
            } else {
                hash = hash.overflowing_mul(22971).0;
                hash = hash.overflowing_add(993777).0;
            }
        }
        hash
    }

    struct Effect {
        crown_after: Grid<u8>,
        crown_hash_after: usize,
    }
    let mut successor = FxHashMap::<usize, Effect>::with_capacity_and_hasher(128, Default::default());

    let mut last_crown_hash = 0;
    for k in 0..1000 {
        let last_top = state.top;
        drop_rocks(&mut state, looping_after, &mut rock_iter, &mut blow_iter);

        if successor.contains_key(&last_crown_hash) {
            println!("LOOPING!!!!! after {k}");
            return;
        }

        println!("Round {k}, increased by {}:\n{}", state.top - last_top, fmt_cave(&state.cave));

        let crown = clone_rows(&state.cave, state.top - ASSUME_TRIMMABLE, state.top);
        let crown_hash = hash_occupancy(&crown);
        println!("Crown is: {}\n{}", crown_hash, fmt_cave(&crown));
        // println!("[{k}] increased by {}", state.top - last_top);
        successor.insert(last_crown_hash, Effect{
            crown_after: crown,
            crown_hash_after: crown_hash,
        });

        last_crown_hash = crown_hash;
    }

    // println!("{}", fmt_cave(&state_repeated.cave));

    // let part2 = best_sofar;
    // if print {
    //     println!("Day {}.  Part 2: {}", DAY, part2);
    // }
    // assert_eq!(part2, if test_mode { 99 } else { 99 });
}

const TEST_EXAMPLE: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
