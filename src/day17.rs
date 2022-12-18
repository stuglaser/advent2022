use std::{
    cmp::max,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

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

const WALLS: u16 = 0b1_0000_0001;

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

#[derive(Clone)]
struct State {
    // cave: Grid<u8>,
    cave: Vec<u16>,
    top: usize,
    last_rock: usize,
    last_blow: usize,
}

impl State {
    fn new() -> Self {
        Self {
            // cave: Grid::filled(1, 7, 0),
            cave: vec![],
            top: 0,
            last_rock: usize::MAX,
            last_blow: usize::MAX,
        }
    }
}

fn collides(cave: &Vec<u16>, rock: &Vec<u16>, r: i32, c: i32) -> bool {
    if r < 0 {
        return true;
    }

    let offset = 1 + c;

    for rock_r in 0..rock.len() {
        let cave_r = r as usize + rock_r;
        if (rock[rock_r] << offset) & cave[cave_r] > 0 {
            return true;
        }
    }

    false
}

fn blit(cave: &mut Vec<u16>, rock: &Vec<u16>, r: usize, c: usize) {
    let shift = 1 + c;
    for rock_r in 0..rock.len() {
        let cave_r = r + rock_r;
        cave[cave_r] |= rock[rock_r] << shift;
    }
}

fn drop_rocks<'a, RockIt, BlowIt>(
    state: &mut State,
    num_rocks: usize,
    rock_iter: &mut RockIt,
    blow_iter: &mut BlowIt,
) where
    RockIt: Iterator<Item = (usize, &'a Vec<u16>)>, //Iterator<Item = &'a Grid<u8>>,
    BlowIt: Iterator<Item = (usize, i8)>,
{
    state.cave.reserve(num_rocks * 3);
    for i in 0..num_rocks {
        let mut drop_c: i32 = 2;
        let mut drop_r: i32 = state.top as i32 + 3;

        let (rock_id, rock) = rock_iter.next().unwrap();
        state.last_rock = rock_id;

        // extend_rows_to(&mut state.cave, drop_r as usize + rock.rows, 0);
        state.cave.resize(drop_r as usize + rock.len(), WALLS);

        loop {
            // Sideways
            let (blow_id, blow) = blow_iter.next().unwrap();
            state.last_blow = blow_id;
            if !collides(&state.cave, &rock, drop_r, drop_c + blow as i32) {
                drop_c += blow as i32;
            }

            // Down
            if drop_r >= 0 && collides(&state.cave, &rock, drop_r - 1, drop_c) {
                // Places the rock
                blit(&mut state.cave, &rock, drop_r as usize, drop_c as usize);
                state.top = max(state.top, drop_r as usize + rock.len());
                break;
            } else {
                drop_r -= 1;
            }
        }
        // println!("{}", fmt_cave(&cave));
    }
}

fn hash_crown_occupancy(state: &State, rows: usize) -> usize {
    let start_row = if rows < state.top {
        state.top - rows
    } else {
        0
    };
    let mut hash = DefaultHasher::new();
    for d in &state.cave[start_row..state.top] {
        d.hash(&mut hash);
    }
    state.last_rock.hash(&mut hash);
    state.last_blow.hash(&mut hash);
    hash.finish() as usize
}

fn fmt_cave(cave: &Vec<u16>) -> String {
    let mut out = String::with_capacity(cave.len() * 10);

    for row in cave.iter().rev() {
        let mut val = *row;
        for _ in 0..9 {
            out.push(match val & 1 {
                0 => '.',
                1 => '#',
                _ => unreachable!(),
            });
            val = val >> 1;
        }
        out.push('\n');
    }

    out
}

pub fn day17(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let looping_after = input_str.len() * ROCK_STRINGS.len();

    // let rocks = ROCK_STRINGS.map(|string| parse_rock(string));

    // LSB is leftwards, and rock[0] is the bottom, so these
    // look backwards and upside down.
    //
    // Got a little to lazy to parse these.
    let brocks = [
        vec![0b1111],
        vec![0b010, 0b111, 0b010],
        vec![0b111, 0b100, 0b100], // vec![0b001, 0b001, 0b111],
        vec![0b1, 0b1, 0b1, 0b1],
        vec![0b11, 0b11],
    ];

    let mut blow_iter = input_str
        .as_bytes()
        .iter()
        .map(|ch| if *ch == b'<' { -1i8 } else { 1 })
        .enumerate()
        .cycle();
    let mut rock_iter = brocks.iter().enumerate().cycle();

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

    struct Breadcrumb {
        iters: usize,
        top: usize,
    }
    let mut breadcrumbs =
        FxHashMap::<usize, Breadcrumb>::with_capacity_and_hasher(32, Default::default());

    // It's important to use the same rock_iter and blow_iter throughout.

    // Looks for where the repititions begin.
    let mut iter_count = 0;
    let mut state = State::new();
    let mut loop_iter_increase = 0;
    let mut loop_top_increase = 0;

    loop {
        drop_rocks(&mut state, 1, &mut rock_iter, &mut blow_iter);
        iter_count += 1;

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

    // println!(
    //     "Found loop at {} (top: {}), with d-iters = {}, d-top = {}",
    //     iter_count, state.top, loop_iter_increase, loop_top_increase
    // );

    const ITERATIONS: usize = 1_000_000_000_000;

    // Now we fake some repetitions
    let num_cycles_to_fake = (ITERATIONS - iter_count) / loop_iter_increase;
    let fake_iter_increase = num_cycles_to_fake * loop_iter_increase;
    let fake_top_increase = num_cycles_to_fake * loop_top_increase;

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
