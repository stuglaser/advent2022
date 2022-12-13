use std::cmp::Ordering;

const DAY: i32 = 13;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Elt {
    Num(i32),
    List(Vec<Elt>),
}

impl PartialOrd for Elt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_list(input: &str) -> (&str, Vec<Elt>) {
    let mut list = Vec::with_capacity(16);
    let mut s = input;
    loop {
        if s.chars().next().unwrap() == ']' {
            break;
        }

        let elt;
        (s, elt) = parse_element(s);
        list.push(elt);

        if s.chars().next().unwrap() == ',' {
            s = &s[1..];
            // This parser allows trailing commas
        }
    }
    (s, list)
}

fn parse_element(input: &str) -> (&str, Elt) {
    match input.chars().next().unwrap() {
        '[' => {
            let (s, list) = parse_list(&input[1..]);
            assert_eq!(s.chars().next().unwrap(), ']');
            (&s[1..], Elt::List(list))
        }
        '0'..='9' => {
            let mut not_numeric_idx = usize::MAX;
            for (idx, ch) in input.char_indices() {
                if ch < '0' || ch > '9' {
                    not_numeric_idx = idx;
                    break;
                }
            }
            let num = input[..not_numeric_idx].parse().unwrap();
            (&input[not_numeric_idx..], Elt::Num(num))
        }
        _ => unreachable!(),
    }
}

fn parse_packet(input: &str) -> Elt {
    let (s, elt) = parse_element(input);
    assert_eq!(s, "");
    elt
}

impl Ord for Elt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Elt::Num(lnum), Elt::Num(rnum)) => lnum.cmp(rnum),
            (Elt::List(llist), Elt::List(rlist)) => {
                for (lelt, relt) in std::iter::zip(llist, rlist) {
                    let c = lelt.cmp(relt);
                    if c != Ordering::Equal {
                        return c;
                    }
                }
                // None triggered, so it's based on list length
                llist.len().cmp(&rlist.len())
            }
            (Elt::Num(lnum), Elt::List(rlist)) => {
                // Manually spelling this out to avoid allocating new Vecs.
                if let Some(rfirst) = rlist.first() {
                    let c = Elt::Num(*lnum).cmp(rfirst);
                    if c != Ordering::Equal {
                        return c;
                    }
                }
                1.cmp(&rlist.len())
            }
            (Elt::List(llist), Elt::Num(rnum)) => {
                if let Some(lfirst) = llist.first() {
                    let c = lfirst.cmp(&Elt::Num(*rnum));
                    if c != Ordering::Equal {
                        return c;
                    }
                }
                llist.len().cmp(&1)
            }
        }
    }
}

pub fn day13(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let packets = {
        let mut packets = Vec::<(Elt, Elt)>::with_capacity(1024);
        let mut lines = input_str.lines();

        loop {
            let p1 = parse_packet(lines.next().unwrap());
            let p2 = parse_packet(lines.next().unwrap());
            packets.push((p1, p2));

            if lines.next() == None {
                break;
            }
        }

        packets
    };

    let mut part1 = 0;
    for (i, (left, right)) in packets.iter().enumerate() {
        // if left.cmp(right) == Ordering::Less {
        //     part1 += i + 1;
        // }
        if left < right {
            part1 += i + 1;
        }
    }

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 13 } else { 5252 });

    let mut all_packets = Vec::with_capacity(2 + 2 * packets.len());
    for (left, right) in packets {
        all_packets.push(left);
        all_packets.push(right);
    }

    let div1 = Elt::List(vec![Elt::List(vec![Elt::Num(2)])]);
    let div2 = Elt::List(vec![Elt::List(vec![Elt::Num(6)])]);

    all_packets.push(div1.clone());
    all_packets.push(div2.clone());

    all_packets.sort();

    let mut part2 = 1;
    for (i, pack) in all_packets.iter().enumerate() {
        if pack == &div1 || pack == &div2 {
            part2 *= i + 1;
        }
    }

    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 140 } else { 20592 });
}

const TEST_EXAMPLE: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
