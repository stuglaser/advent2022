use rustc_hash::FxHashSet;

use crate::utils::Grid;

const DAY: i32 = 8;

pub fn day08(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let grid = {
        let mut grid_data = Vec::with_capacity(100 * 100);
        let mut cols = 0;

        for line in input_str.lines() {
            let line_bytes = line.as_bytes();
            cols = line_bytes.len();
            for ch in line_bytes {
                grid_data.push(ch - b'0');
            }
        }

        Grid {
            cols: cols,
            rows: grid_data.len() / cols,
            data: grid_data,
        }
    };
    // println!("{}", grid.fmt_compact());

    let mut visible = FxHashSet::<(usize, usize)>::with_capacity_and_hasher(256, Default::default());

    // Horizontal
    for i in 1..(grid.rows - 1) {
        // Left -> Right
        let mut tallest = grid[(i, 0)];
        for j in 1..(grid.cols - 1) {
            let tree = grid[(i, j)];
            if tree > tallest {
                visible.insert((i, j));
                tallest = tree;
            }
        }

        // Right -> Left
        let mut tallest = grid[(i, grid.cols - 1)];
        for j in (1..(grid.cols - 1)).rev() {
            let tree = grid[(i, j)];
            if tree > tallest {
                visible.insert((i, j));
                tallest = tree;
            }
        }
    }

    // Vertical
    for j in 1..(grid.cols - 1) {
        // Top -> bottom
        let mut tallest = grid[(0, j)];
        for i in 1..(grid.rows - 1) {
            let tree = grid[(i, j)];
            if tree > tallest { 
                visible.insert((i, j));
                tallest = tree;
            }
        }
        
        // Bottom -> Top
        let mut tallest = grid[(grid.rows - 1, j)];
        for i in (1..(grid.rows - 1)).rev() {
            let tree = grid[(i, j)];
            if tree > tallest {
                visible.insert((i, j));
                tallest = tree;
            }
        }
    }

    let part1 = visible.len() + 2 * grid.rows + 2 * grid.cols - 4;

    let mut scores = Grid::filled(grid.rows, grid.cols, 1);

    // Horizontal
    for i in 0..grid.rows {
        // Left -> Right
        let mut blockers = [0; 10];
        for j in 0..grid.cols {
            let tree = grid[(i, j)];
            scores[(i, j)] *= j - blockers[tree as usize];

            for k in 0..=tree {
                blockers[k as usize] = j;
            }
        }

        // Right -> Left
        let mut blockers = [grid.cols - 1; 10];
        for j in (0..grid.cols).rev() {
            let tree = grid[(i, j)];
            scores[(i, j)] *= blockers[tree as usize] - j;

            for k in 0..=tree {
                blockers[k as usize] = j;
            }
        }
    }

    // Vertical
    for j in 0..grid.cols {
        // Top -> bottom
        let mut blockers = [0; 10];
        for i in 0..grid.rows {
            let tree = grid[(i, j)];
            scores[(i, j)] *= i - blockers[tree as usize];

            for k in 0..=tree {
                blockers[k as usize] = i;
            }
        }
        
        // Bottom -> Top
        let mut blockers = [grid.rows - 1; 10];
        for i in (0..grid.rows).rev() {
            let tree = grid[(i, j)];
            scores[(i, j)] *= blockers[tree as usize] - i;

            for k in 0..=tree {
                blockers[k as usize] = i;
            }
        }
    }

    let part2 = *scores.data.iter().max().unwrap() as i32;

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 21 } else { 1854 });

    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 8 } else { 527340 });
}

const TEST_EXAMPLE: &'static str = "30373
25512
65332
33549
35390";
