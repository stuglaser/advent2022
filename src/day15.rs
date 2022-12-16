use std::cmp::max;

use itertools::Itertools;

use crate::utils::Pt;

const DAY: i32 = 15;

// Inclusive
fn empty_at_y(m: &(Pt, Pt), y: i32) -> Option<(i32, i32)> {
    let sensor = &m.0;
    let beacon = &m.1;
    let mdist = sensor.l1_to(beacon);

    let r = mdist - (y - sensor.y).abs();
    if r <= 0 {
        return None;
    }

    if y == beacon.y {
        if beacon.x == sensor.x - r {
            Some((sensor.x - r + 1, sensor.x + r))
        } else if beacon.x == sensor.x + r {
            Some((sensor.x - r, sensor.x + r - 1))
        } else {
            unreachable!()
        }
    } else {
        Some((sensor.x - r, sensor.x + r))
    }
}

fn find_unseen(sensed: &Vec<(Pt, i32)>, lo: &Pt, hi: &Pt) -> Option<Pt> {
    // println!("Searching [{}..{}], [{}..{}]", lo.x, hi.x, lo.y, hi.y);
    for (sensor, l1) in sensed {
        if sensor.l1_to_coords(lo.x, lo.y) <= *l1
            && sensor.l1_to_coords(lo.x, hi.y) <= *l1
            && sensor.l1_to_coords(hi.x, lo.y) <= *l1
            && sensor.l1_to_coords(hi.x, hi.y) <= *l1
        {
            // Fully covered
            return None;
        }
    }

    if lo == hi {
        return Some(lo.clone());
    }

    if (hi.x - lo.x) > (hi.y - lo.y) {
        // Split on x
        let mid = (lo.x + hi.x) / 2;
        find_unseen(sensed, lo, &Pt::at(mid, hi.y))
            .or_else(|| find_unseen(sensed, &Pt::at(mid + 1, lo.y), hi))
    } else {
        // Split on y
        let mid = (lo.y + hi.y) / 2;
        find_unseen(sensed, &Pt::at(lo.x, lo.y), &Pt::at(hi.x, mid))
            .or_else(|| find_unseen(sensed, &Pt::at(lo.x, mid + 1), hi))
    }
}

pub fn day15(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let re = regex::Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
    )
    .unwrap();

    let measurements = {
        let mut measurements = Vec::<(Pt, Pt)>::with_capacity(64);
        for line in input_str.lines() {
            let caps = re.captures(line).unwrap();
            measurements.push((
                Pt::at(caps[1].parse().unwrap(), caps[2].parse().unwrap()),
                Pt::at(caps[3].parse().unwrap(), caps[4].parse().unwrap()),
            ));
        }
        measurements
    };

    let probe_y = if test_mode { 10 } else { 2000000 };

    let empty_ranges: Vec<(i32, i32)> = measurements
        .iter()
        .filter_map(|m| empty_at_y(m, probe_y))
        .sorted()
        .collect();

    let mut top = i32::MIN;
    let mut part1 = 0;
    for (lo, hi) in empty_ranges {
        part1 += hi - max(lo, top) + 1;
        top = hi + 1;
    }

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 26 } else { 4961647 });

    let limit = if test_mode { 20 } else { 4000000 };

    let sensed: Vec<_> = measurements
        .iter()
        .map(|(s, b)| (s.clone(), s.l1_to(b)))
        .collect();
    let distress = find_unseen(&sensed, &Pt::at(0, 0), &Pt::at(limit, limit)).unwrap();
    let part2 = 4000000 * (distress.x as usize) + distress.y as usize;

    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 56000011 } else { 12274327017867 });
}

const TEST_EXAMPLE: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
