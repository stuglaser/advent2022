use std::{cmp::max, collections::BinaryHeap};

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

fn best_released<'a>(
    rooms: &'a FxHashMap<String, Room>,
    released: &mut FxHashSet<&'a str>,
    loc: &'a str,
    sofar: i32,
    time_left: i32,
) -> i32 {
    println!("At {loc} {time_left} | sofar: {sofar}");
    if time_left == 0 {
        return sofar;
    }

    let room = &rooms[loc];

    let mut best = 0;

    // No release
    for n in &room.tunnels {
        let released = best_released(rooms, released, n, sofar, time_left - 1);
        best = max(best, released);
    }

    if time_left > 1 && !released.contains(loc) {
        // Turn on valve
        released.insert(loc);
        let now_flow = sofar + (time_left - 1) * room.flow;
        for n in &room.tunnels {
            let released = best_released(rooms, released, n, now_flow, time_left - 2);
            best = max(best, released);
        }
        released.remove(loc);
    }
    best
}

fn compute_theoretical_flow(rooms: &FxHashMap<String, Room>, dist: &FxHashMap<(String, String), i32>, remain: &FxHashSet<&String>, time_left: i32) -> i32 {
    let flows = remain.iter().map(|r| rooms.get(*r).unwrap().flow).sorted().collect::<Vec<_>>();

    let mut theoretical = 0;
    let mut time_left = time_left;
    for flow in flows.iter().rev() {
        time_left -= 2;  // Move and turn on
        if time_left <= 0 {
            break;
        }

        theoretical += time_left * flow;
    }
    theoretical
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

    // let mut rooms = Vec::with_capacity(32);
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

    println!("Rooms: {:?}", rooms);

    // let mut released = FxHashSet::with_capacity_and_hasher(rooms.len(), Default::default());
    // let part1 = best_released(&rooms, &mut released, "AA", 0, 30);

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
    println!("Matter: {:?}", matter);

    for a in &matter {
        for b in &matter {
            println!(
                "{} -> {} = {}",
                a,
                b,
                dist.get(&(a.to_string(), b.to_string())).unwrap()
            );
        }
    }

    let start = "AA";

    #[derive(Debug)]
    struct Remember<'a> {
        room: String,
        time_left: i32,
        sofar: i32,
        rooms_left: FxHashSet<&'a String>,

        followed: Vec<String>, // TODO: delete me
    };
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

    // let mut part1 = 0;
    let mut best_sofar = 0;
    loop {
        let ByFirst((theoretical, at)) = heap.pop().unwrap();
        println!("At: {} with {}, {:?}", at.room, theoretical, at);

        let mut next_followed = at.followed.clone();
        next_followed.push(at.room.clone());

        if at.sofar > best_sofar {
            best_sofar = at.sofar;
            println!("BEST: {}", best_sofar);
        }

        if at.rooms_left.is_empty() || at.time_left <= 2 {
            println!("Guess we're done????");
            // part1 = theoretical;
            break;
        }


        for next in &at.rooms_left {
            println!("  Expand {}", next);
            let mut next_left = at.rooms_left.clone();
            next_left.remove(next);

            let time_left_after_move = at.time_left - dist.get(&(at.room.clone(), (*next).clone())).unwrap();
            if time_left_after_move > 2 {
                let time_left_after_open = time_left_after_move - 1;
                let next_sofar = at.sofar + time_left_after_open * rooms.get(*next).unwrap().flow;

                let theoretical_flow = next_sofar + compute_theoretical_flow(&rooms, &dist, &next_left, time_left_after_open);

                println!("  --> {theoretical_flow} :: {next}, {time_left_after_open} left, {next_sofar} flow     remaining: {:?}", next_left);
                heap.push(ByFirst((
                    theoretical_flow,
                    Remember {
                        room: (*next).clone(),
                        time_left: time_left_after_open,
                        sofar: next_sofar,
                        rooms_left: next_left,
                        followed: next_followed.clone(),
                    }
                )));
            }
        }

    }

    // let matter = matter.iter().map(|s| s.to_string()).collect::<Vec<_>>();

    // let mut i = 0;
    // for order in matter.iter().permutations(matter.len()) {
    //     i += 1;
    //     if i % 100_000 == 0 {
    //         println!("Check {:?}", order);
    //     }
    // }

    let part1 = best_sofar;
    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 1651 } else { 2330 });

    // if print {
    //     println!("Day {}.  Part 2: {}", DAY, part2);
    // }
    // assert_eq!(part2, if test_mode { 56000011 } else { 12274327017867 });
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
