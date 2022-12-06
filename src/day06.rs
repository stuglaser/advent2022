const DAY: i32 = 6;

fn first_unique(input: &[u8], len: usize) -> i32 {
    let mut active = vec![0u8; 26];
    let mut active_entries = 0;
    for i in 0..input.len() {
        // Tracks input[i]
        let b = input[i] - b'a';
        if active[b as usize] == 0 {
            active_entries += 1;
        }
        active[b as usize] += 1;

        if i >= len {
            // Untracks input[i - len]
            let a = input[i - len] - b'a';
            if active[a as usize] == 1 {
                active_entries -= 1;
            }
            active[a as usize] -= 1;

            // Check
            if active_entries == len {
                return i as i32 + 1;
            }
        }
    }
    -1
}

pub fn day06(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let input = input_str.as_bytes();

    let part1 = first_unique(input, 4);
    let part2 = first_unique(input, 14);

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 7 } else { 1275 });

    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 19 } else { 3605 });
}

const TEST_EXAMPLE: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
