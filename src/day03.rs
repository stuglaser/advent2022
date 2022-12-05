const DAY: i32 = 3;

pub fn letter_to_val(letter: char) -> u8 {
    match letter {
        'a'..='z' => letter as u8 - b'a' + 1,
        'A'..='Z' => letter as u8 - b'A' + 27,
        _ => unreachable!(),
    }
}

pub fn day03(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let input: Vec<Vec<u8>> = input_str.lines().map(|line| {
        line.chars().map(letter_to_val).collect()
    }).collect();

    let mut part1 = 0;
    for bag in &input {
        let mut have = [0u8; 53];
        let mid = bag.len() / 2;

        for i in 0..mid {
            have[bag[i] as usize] = 1;
        }

        for i in mid..bag.len() {
            let item = bag[i];
            if have[item as usize] == 1 {
                part1 += item as i32;
                break;
            }
        }
    }

    let mut part2 = 0;
    for chunk in input.chunks(3) {
        let mut have = [0u8; 53];

        for (i, bag) in chunk.iter().enumerate() {
            for item in bag {
                have[*item as usize] = have[*item as usize] | (1 << i);
            }
        }

        for (i, k) in have.iter().enumerate() {
            if *k == 7 {
                part2 += i as i32;
            }
        }
    }

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 157 } else { 7766 });


    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 70 } else { 2415 });
}

const TEST_EXAMPLE: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
