use std::collections::BinaryHeap;

use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::utils::ByFirst;

const DAY: i32 = 16;

#[derive(Debug)]
struct Room {
    name: String,
    flow: i32,
    tunnels: Vec<String>,
}

fn compute_optimistic_flow(
    rooms: &FxHashMap<String, Room>,
    remain: &FxHashSet<&String>,
    time_left: i32,
) -> i32 {
    let flows = remain
        .iter()
        .map(|r| rooms.get(*r).unwrap().flow)
        .sorted()
        .collect::<Vec<_>>();

    let mut optimistic = 0;
    let mut time_left = time_left;
    for flow in flows.iter().rev() {
        time_left -= 2; // Move and turn on
        if time_left <= 0 {
            break;
        }

        optimistic += time_left * flow;
    }
    optimistic
}

// Gotta watch what the elephant does.
fn compute_optimistic_flow_with_friend(
    rooms: &FxHashMap<String, Room>,
    remain: &FxHashSet<&String>,
    a_time_left: i32,
    b_time_left: i32,
) -> i32 {
    let flows = remain
        .iter()
        .map(|r| rooms.get(*r).unwrap().flow)
        .sorted()
        .collect::<Vec<_>>();

    let mut optimistic = 0;
    let mut a_time_left = a_time_left;
    let mut b_time_left = b_time_left;
    for flow in flows.iter().rev() {
        if a_time_left <= 2 && b_time_left <= 2 {
            break;
        }

        if a_time_left >= b_time_left {
            // A turns it on
            a_time_left -= 2;
            optimistic += a_time_left * flow;
        } else {
            // B turns it on
            b_time_left -= 2;
            optimistic += b_time_left * flow;
        }
    }
    optimistic
}

pub fn day16(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    let re =
        regex::Regex::new(r"Valve ([^ ]+) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();

    let mut rooms = FxHashMap::with_capacity_and_hasher(128, Default::default());
    for line in input_str.lines() {
        let caps = re.captures(line).unwrap();
        let tunnels = caps[3].split(", ").map(|s| s.to_string()).collect();

        let name: String = caps[1].into();
        rooms.insert(
            name.clone(),
            Room {
                name,
                flow: caps[2].parse().unwrap(),
                tunnels,
            },
        );
    }
    let rooms = rooms;

    // println!("Rooms: {:?}", rooms);

    // All paths
    let mut dist = FxHashMap::<(String, String), i32>::with_capacity_and_hasher(
        rooms.len() * rooms.len(),
        Default::default(),
    );
    for (a, a_room) in rooms.iter() {
        dist.insert((a.clone(), a.clone()), 0);
        for (b, _) in rooms.iter() {
            if a != b {
                if a_room.tunnels.iter().find(|dest| *dest == b) == None {
                    dist.insert((a.clone(), b.clone()), i32::MAX / 2);
                    dist.insert((b.clone(), a.clone()), i32::MAX / 2);
                } else {
                    dist.insert((a.clone(), b.clone()), 1);
                    dist.insert((b.clone(), a.clone()), 1);
                }
            }
        }
    }

    for (r, _) in rooms.iter() {
        for (u, _) in rooms.iter() {
            for (v, _) in rooms.iter() {
                // TODO: string copies
                let longcut = *dist.get(&(u.to_string(), v.to_string())).unwrap();
                let shortcut = dist.get(&(u.to_string(), r.to_string())).unwrap()
                    + dist.get(&(r.to_string(), v.to_string())).unwrap();
                if shortcut < longcut {
                    dist.insert((u.to_string(), v.to_string()), shortcut);
                    dist.insert((v.to_string(), u.to_string()), shortcut);
                }
            }
        }
    }

    let matter = rooms
        .iter()
        .filter_map(|(s, r)| if r.flow > 0 { Some(s) } else { None })
        .collect::<FxHashSet<_>>();
    // println!("Matter: {:?}", matter);


    #[derive(Debug)]
    struct Remember<'a> {
        room: String,
        time_left: i32,
        sofar: i32,
        rooms_left: FxHashSet<&'a String>,

        followed: Vec<String>, // TODO: delete me
    }
    let mut heap: BinaryHeap<ByFirst<(i32, Remember)>> = BinaryHeap::new();
    heap.push(ByFirst((
        i32::MAX,
        Remember {
            room: "AA".to_string(),
            time_left: 30,
            sofar: 0,
            rooms_left: matter.clone(),
            followed: Vec::new(),
        },
    )));

    let mut best_sofar = 0;
    loop {
        let ByFirst((_theoretical, at)) = heap.pop().unwrap();

        let mut next_followed = at.followed.clone();
        next_followed.push(at.room.clone());

        if at.sofar > best_sofar {
            best_sofar = at.sofar;
        }

        if at.rooms_left.is_empty() || at.time_left <= 2 {
            break;
        }

        for next in &at.rooms_left {
            let mut next_left = at.rooms_left.clone();
            next_left.remove(next);

            let time_left_after_move =
                at.time_left - dist.get(&(at.room.clone(), (*next).clone())).unwrap();
            if time_left_after_move > 2 {
                let time_left_after_open = time_left_after_move - 1;
                let next_sofar = at.sofar + time_left_after_open * rooms.get(*next).unwrap().flow;

                let theoretical_flow = next_sofar
                    + compute_optimistic_flow(&rooms, &next_left, time_left_after_open);

                heap.push(ByFirst((
                    theoretical_flow,
                    Remember {
                        room: (*next).clone(),
                        time_left: time_left_after_open,
                        sofar: next_sofar,
                        rooms_left: next_left,
                        followed: next_followed.clone(),
                    },
                )));
            }
        }
    }

    let part1 = best_sofar;
    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 1651 } else { 2330 });

    // Part 2, with an elephant

    #[derive(Debug)]
    struct Remember2<'a> {
        me_at: String,
        me_time_left: i32,

        elph_at: String,
        elph_time_left: i32,

        sofar: i32, // Flow released so far.
        rooms_left: FxHashSet<&'a String>,

        me_followed: Vec<String>, // TODO: delete me
        elph_followed: Vec<String>,
    }

    let mut heap: BinaryHeap<ByFirst<(i32, Remember2)>> = BinaryHeap::new();
    heap.push(ByFirst((
        i32::MAX,
        Remember2 {
            me_at: "AA".to_string(),
            me_time_left: 26,
            elph_at: "AA".to_string(),
            elph_time_left: 26,
            sofar: 0,
            rooms_left: matter.clone(),
            me_followed: Vec::new(),
            elph_followed: Vec::new(),
        },
    )));

    let mut best_sofar = 0;
    loop {
        let ByFirst((_theoretical, at)) = heap.pop().unwrap();

        if at.sofar > best_sofar {
            best_sofar = at.sofar;
        }

        if at.rooms_left.is_empty() || (at.me_time_left <= 2 && at.elph_time_left <= 2) {
            break;
        }

        for next in &at.rooms_left {
            let mut next_left = at.rooms_left.clone();
            next_left.remove(next);

            if at.me_time_left >= at.elph_time_left {
                // I move
                let mut followed = at.me_followed.clone();
                followed.push((*next).clone());

                let time_left_after_move =
                    at.me_time_left - dist.get(&(at.me_at.clone(), (*next).clone())).unwrap();
                if time_left_after_move > 2 {
                    let time_left_after_open = time_left_after_move - 1;
                    let next_sofar =
                        at.sofar + time_left_after_open * rooms.get(*next).unwrap().flow;

                    let theoretical_flow = next_sofar
                        + compute_optimistic_flow_with_friend(
                            &rooms,
                            &next_left,
                            time_left_after_open,
                            at.elph_time_left,
                        );

                    heap.push(ByFirst((
                        theoretical_flow,
                        Remember2 {
                            me_at: (*next).clone(),
                            me_time_left: time_left_after_open,
                            elph_at: at.elph_at.clone(),
                            elph_time_left: at.elph_time_left,
                            sofar: next_sofar,
                            rooms_left: next_left,

                            me_followed: followed,
                            elph_followed: at.elph_followed.clone(),
                        },
                    )));
                }
            } else {
                // Elephant moves
                let mut followed = at.elph_followed.clone();
                followed.push((*next).clone());

                let time_left_after_move =
                    at.elph_time_left - dist.get(&(at.elph_at.clone(), (*next).clone())).unwrap();
                if time_left_after_move > 2 {
                    let time_left_after_open = time_left_after_move - 1;
                    let next_sofar =
                        at.sofar + time_left_after_open * rooms.get(*next).unwrap().flow;

                    let theoretical_flow = next_sofar
                        + compute_optimistic_flow_with_friend(
                            &rooms,
                            &next_left,
                            at.me_time_left,
                            time_left_after_open,
                        );

                    heap.push(ByFirst((
                        theoretical_flow,
                        Remember2 {
                            me_at: at.me_at.clone(),
                            me_time_left: at.me_time_left,
                            elph_at: (*next).clone(),
                            elph_time_left: time_left_after_open,
                            sofar: next_sofar,
                            rooms_left: next_left,
                            me_followed: at.me_followed.clone(),
                            elph_followed: followed,
                        },
                    )));
                }
            }
        }
    }

    let part2 = best_sofar;
    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 1707 } else { 2675 });
}

const TEST_EXAMPLE: &'static str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
