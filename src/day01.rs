use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: i32 = 1;

pub fn day01(_test_mode: bool, print: bool) {
    let reader = BufReader::new(File::open("inputs/input01.txt").expect("Cannot open input file"));

    let mut carrying = Vec::<Vec<i32>>::with_capacity(100);
    carrying.push(Vec::new());
    for line in reader.lines() {
        let text = line.unwrap();
        if text.is_empty() {
            carrying.push(Vec::new());
        } else {
            carrying.last_mut().unwrap().push(text.parse().unwrap());
        }
    }

    let mut part1 = 0;
    let mut top3 = (0i32, 0i32, 0i32);

    for elf in &carrying {
        let this_carry = elf.iter().sum();
        part1 = max(part1, this_carry);

        if this_carry > top3.0 {
            top3.2 = top3.1;
            top3.1 = top3.0;
            top3.0 = this_carry;
        } else if this_carry > top3.1 {
            top3.2 = top3.1;
            top3.1 = this_carry;
        } else if this_carry > top3.2 {
            top3.2 = this_carry;
        }
    }

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, 70720);

    let part2 = top3.0 + top3.1 + top3.2;

    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, 207148);
}
