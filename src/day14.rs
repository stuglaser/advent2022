use itertools::Itertools;
use std::cmp::max;

use crate::utils::Grid;

const DAY: i32 = 14;

#[inline]
fn lohi<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if b < a {
        (b, a)
    } else {
        (a, b)
    }
}

pub fn day14(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let rocks = {
        let mut rocks: Vec<Vec<(i32, i32)>> = Vec::with_capacity(64);

        for line in input_str.lines() {
            let path = line
                .split(" -> ")
                .map(|coord_str| {
                    let mut pieces = coord_str.split(",");
                    let x = pieces.next().unwrap().parse().unwrap();
                    let y = pieces.next().unwrap().parse().unwrap();
                    (x, y)
                })
                .collect();

            rocks.push(path);
        }
        rocks
    };

    let y_max = {
        let mut y_max = i32::MIN;
        for rock in &rocks {
            for pt in rock {
                y_max = max(y_max, pt.1);
            }
        }
        y_max
    };

    let x0 = 500 - y_max - 3;

    let mut grid = Grid::<u8>::filled((y_max + 3) as usize, (2 * (y_max + 3)) as usize, 0);

    for rock in &rocks {
        for (p1, p2) in rock.iter().tuple_windows() {
            if p1.0 == p2.0 {
                // Along Y
                let (y_lo, y_hi) = lohi(p1.1, p2.1);
                for y in y_lo..=y_hi {
                    grid[(y as usize, (p1.0 - x0) as usize)] = 1;
                }
            } else {
                // Along X
                let (x_lo, x_hi) = lohi(p1.0, p2.0);
                for x in x_lo..=x_hi {
                    grid[(p1.1 as usize, (x - x0) as usize)] = 1;
                }
            }
        }
    }

    // Line along the bottom
    let rows = grid.rows;
    for c in 0..grid.cols {
        grid[(rows - 1, c)] = 1;
    }

    let drop_col = (500 - x0) as usize;

    let mut grains = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    'outer: loop {
        let mut sand_r = 0usize;
        let mut sand_c = drop_col;

        if grid[(sand_r, sand_c)] > 0 {
            part2 = grains;
            break;
        }

        loop {
            if grid[(sand_r + 1, sand_c)] == 0 {
                sand_r += 1;
            } else if grid[(sand_r + 1, sand_c - 1)] == 0 {
                sand_r += 1;
                sand_c -= 1;
            } else if grid[(sand_r + 1, sand_c + 1)] == 0 {
                sand_r += 1;
                sand_c += 1;
            } else {
                // Blocked
                grid[(sand_r, sand_c)] = 2;

                if part1 == 0 && sand_r == grid.rows - 2 {
                    part1 = grains;
                }
                grains += 1;
                break;
            }

            if sand_r + 1 == grid.rows {
                break 'outer;
            }
        }
    }

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 24 } else { 979 });

    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 93 } else { 29044 });
}

const TEST_EXAMPLE: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
