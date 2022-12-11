const DAY: i32 = 11;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug)]
struct Rule {
    op: Op,
    op_value: Option<i32>,
    test_divisible: i32,
    true_monkey: i32,
    false_monkey: i32,
}

pub fn day11(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let mut line_iter = input_str.lines();

    let (starting, rules) = {
        let mut rules = Vec::with_capacity(32);
        let mut has: Vec<Vec<i32>> = Vec::with_capacity(32);

        loop {
            // "Monkey N:"
            line_iter.next().unwrap();

            // "Starting items: 79, 98"
            let starting_str = line_iter.next().unwrap().split_once(": ").unwrap().1;
            let starting: Vec<i32> = starting_str
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();

            // Operation: new = old * 19
            let op_str = line_iter.next().unwrap().split_once(" = ").unwrap().1;
            let mut op_pieces = op_str.split(" ");
            op_pieces.next().unwrap(); // "old"
            let op = match op_pieces.next().unwrap() {
                "+" => Op::Add,
                "*" => Op::Mul,
                _ => unreachable!(),
            };
            let op_value: Option<i32> = match op_pieces.next().unwrap() {
                "old" => None,
                value => Some(value.parse().unwrap()),
            };

            // "Test: divisible by 23"
            let test_str = line_iter.next().unwrap().rsplit_once(" ").unwrap().1;
            let test_divisible: i32 = test_str.parse().unwrap();

            // "  If true: throw to monkey 2"
            let true_monkey: i32 = line_iter
                .next()
                .unwrap()
                .rsplit_once(" ")
                .unwrap()
                .1
                .parse()
                .unwrap();
            let false_monkey: i32 = line_iter
                .next()
                .unwrap()
                .rsplit_once(" ")
                .unwrap()
                .1
                .parse()
                .unwrap();

            rules.push(Rule {
                op,
                op_value,
                test_divisible,
                true_monkey,
                false_monkey,
            });
            has.push(starting);

            // Eats the newline
            if line_iter.next() == None {
                break;
            }
        }

        (has, rules)
    };

    // Part 1

    let mut has = starting.clone();
    let mut inspections = vec![0; rules.len()];

    for _ in 0..20 {
        for i in 0..rules.len() {
            let rule = &rules[i];
            let items = has[i].clone();
            inspections[i] += items.len();

            for mut item in items {
                let rhs = rule.op_value.unwrap_or(item);
                match rule.op {
                    Op::Add => item += rhs,
                    Op::Mul => item *= rhs,
                };

                item /= 3;
                let to_monkey = if (item % rule.test_divisible) == 0 {
                    rule.true_monkey
                } else {
                    rule.false_monkey
                };


                has[to_monkey as usize].push(item);
            }

            has[i].truncate(0);
        }
    }

    let mut top = (0, 0);
    for insp in inspections {
        if insp > top.0 {
            top.1 = top.0;
            top.0 = insp;
        } else if insp > top.1 {
            top.1 = insp;
        }
    }
    let part1 = top.0 * top.1;

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 10605 } else { 58322 });

    // Part 2
    
    let mut has = starting.clone();
    let mut inspections = vec![0; rules.len()];

    let max_modulo: i64 = rules.iter().map(|rule| rule.test_divisible as i64).product();

    for _ in 0..10_000 {
        for i in 0..rules.len() {
            let rule = &rules[i];
            let items = has[i].clone();
            inspections[i] += items.len();

            for item in items {
                let rhs = rule.op_value.unwrap_or(item);
                let item = (match rule.op {
                    Op::Add => (item as i64) + (rhs as i64),
                    Op::Mul => (item as i64) * (rhs as i64),
                } % max_modulo) as i32;

                let to_monkey = if (item % rule.test_divisible) == 0 {
                    rule.true_monkey
                } else {
                    rule.false_monkey
                };

                has[to_monkey as usize].push(item);
            }

            has[i].truncate(0);
        }
    }

    let mut top = (0, 0);
    for insp in inspections {
        if insp > top.0 {
            top.1 = top.0;
            top.0 = insp;
        } else if insp > top.1 {
            top.1 = insp;
        }
    }
    let part2 = top.0 * top.1;
    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 2713310158 } else { 13937702909 });
}

const TEST_EXAMPLE: &'static str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1";
