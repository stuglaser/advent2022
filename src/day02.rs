const DAY: i32 = 2;

fn to_num(s: &str) -> i32 {
    match s {
        "A" => 0, // Rock
        "B" => 1, // Paper
        "C" => 2, // Scissors
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!(),
    }
}

fn win_score(them: i32, us: i32) -> i32 {
    if them == us {
        3
    } else if us == (them + 1) % 3 {
        6
    } else {
        0
    }
}

pub fn day02(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let lines = input_str.lines();
    let input: Vec<_> = lines
        .map(|line| {
            let mut parts = line.split(" ");
            let them = to_num(parts.next().unwrap());
            let us = to_num(parts.next().unwrap());
            (them, us)
        })
        .collect();

    let mut part1 = 0;
    for round in &input {
        part1 += win_score(round.0, round.1) + round.1 + 1;
    }

    let mut part2 = 0;
    for (them, goal) in &input {
        let us = (them + goal - 1 + 3) % 3;
        part2 += win_score(*them, us) + us + 1;
    }


    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 15 } else { 10624 });


    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 12 } else { 14060 });
}

const TEST_EXAMPLE: &'static str = "A Y
B X
C Z";
