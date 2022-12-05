const DAY: i32 = 4;

// 34-96  -->  (34, 96)
fn parse_range(input: &str) -> (i32, i32) {
    let (a, b) = input.split_once("-").unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

pub fn day04(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let input: Vec<_> = input_str.lines().map(|line| {
        let (r1, r2) = line.split_once(",").unwrap();
        (parse_range(r1), parse_range(r2))
    }).collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for (elf1, elf2) in input {
        if (elf1.0 <= elf2.0 && elf1.1 >= elf2.1) || (elf2.0 <= elf1.0 && elf2.1 >= elf1.1) {
            part1 += 1;
        }

        if !(elf1.1 < elf2.0 || elf1.0 > elf2.1) {
            part2 += 1;
        }
    }

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 2 } else { 503 });


    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 4 } else { 827 });
}

const TEST_EXAMPLE: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
