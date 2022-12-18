use std::{
    cmp::{max, min},
    collections::VecDeque,
};

use rustc_hash::FxHashSet;

use crate::utils::Pt3;

const DAY: i32 = 18;

struct Neighbor6Iterator<'a> {
    point: &'a Pt3,
    at: u8,
}

fn neighbors6(point: &Pt3) -> Neighbor6Iterator {
    Neighbor6Iterator { point, at: 0 }
}

impl<'a> Iterator for Neighbor6Iterator<'a> {
    type Item = Pt3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at == 6 {
            None
        } else {
            self.at += 1;
            Some(match self.at - 1 {
                0 => self.point.plus_coords(1, 0, 0),
                1 => self.point.plus_coords(-1, 0, 0),
                2 => self.point.plus_coords(0, 1, 0),
                3 => self.point.plus_coords(0, -1, 0),
                4 => self.point.plus_coords(0, 0, 1),
                5 => self.point.plus_coords(0, 0, -1),
                _ => unreachable!(),
            })
        }
    }
}

pub fn day18(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    fn parse_point(s: &str) -> Pt3 {
        let mut pieces = s.split(",");
        let x = pieces.next().unwrap().parse().unwrap();
        let y = pieces.next().unwrap().parse().unwrap();
        let z = pieces.next().unwrap().parse().unwrap();
        Pt3::new(x, y, z)
    }

    let points = input_str.lines().map(parse_point).collect::<FxHashSet<_>>();

    let mut exposed = 0usize;
    for point in &points {
        for n in neighbors6(point) {
            if !points.contains(&n) {
                exposed += 1;
            }
        }
    }

    let part1 = exposed;
    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 64 } else { 3500 });

    let mut lo = Pt3::new(i32::MAX, i32::MAX, i32::MAX);
    let mut hi = Pt3::new(i32::MIN, i32::MIN, i32::MIN);
    for p in &points {
        lo.x = min(lo.x, p.x);
        lo.y = min(lo.y, p.y);
        lo.z = min(lo.z, p.z);
        hi.x = max(hi.x, p.x);
        hi.y = max(hi.y, p.y);
        hi.z = max(hi.z, p.z);
    }
    lo = lo.plus_coords(-1, -1, -1);
    hi = hi.plus_coords(1, 1, 1);

    let start = Pt3::new(lo.x, (lo.y + hi.y) / 2, (lo.z + hi.z) / 2);
    let mut queue = VecDeque::<Pt3>::with_capacity(((hi.y - lo.y) * (hi.z - lo.z)) as usize);
    let mut air = FxHashSet::<Pt3>::with_capacity_and_hasher(
        ((hi.x - lo.x) * (hi.y - lo.y) * (hi.z - lo.z)) as usize,
        Default::default(),
    );
    queue.push_back(start.clone());
    air.insert(start);
    while !queue.is_empty() {
        let p = queue.pop_front().unwrap();
        for n in neighbors6(&p) {
            if lo.x <= n.x
                && n.x <= hi.x
                && lo.y <= n.y
                && n.y <= hi.y
                && lo.z <= n.z
                && n.z <= hi.z
                && !points.contains(&n)
                && !air.contains(&n)
            {
                air.insert(n.clone());
                queue.push_back(n);
            }
        }
    }

    // Got all the air now.

    let mut exposed = 0usize;
    for point in &points {
        for n in neighbors6(point) {
            if air.contains(&n) {
                exposed += 1;
            }
        }
    }

    let part2 = exposed;
    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 58 } else { 2048 });
}

const TEST_EXAMPLE: &'static str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
