use std::cmp::min;

use rustc_hash::FxHashMap;

const DAY: i32 = 7;

#[derive(Debug)]
struct Dir {
    subdirs: Vec<String>,
    files: Vec<(String, usize)>,
}

impl Dir {
    fn new() -> Self {
        Dir {
            subdirs: Vec::with_capacity(16),
            files: Vec::with_capacity(32),
        }
    }
}

type DirTree = FxHashMap<String, Dir>;

fn walk_for_sizes(tree: &DirTree, path: &str, sum_small: &mut usize) -> usize {
    let dir = tree.get(path).unwrap();

    let mut size = 0;
    for subdir in &dir.subdirs {
        let subdir_path = format!("{}/{}", path, subdir);
        size += walk_for_sizes(tree, &subdir_path, sum_small);
    }

    for (_, fsize) in &dir.files {
        size += fsize;
    }

    if size <= 100000 {
        *sum_small += size;
    }

    size
}

// Returns (total size, smallest_above)
fn find_smallest_above(tree: &DirTree, threshold: usize, path: &str) -> (usize, usize) {
    let dir = tree.get(path).unwrap();

    let mut size = 0;
    let mut smallest_above = usize::MAX;

    for subdir in &dir.subdirs {
        let subdir_path = format!("{}/{}", path, subdir);
        let subdir_result = find_smallest_above(tree, threshold, &subdir_path);

        size += subdir_result.0;
        if subdir_result.1 >= threshold {
            smallest_above = min(smallest_above, subdir_result.1);
        }
    }

    for (_, fsize) in &dir.files {
        size += fsize;
    }

    if size >= threshold {
        smallest_above = min(smallest_above, size);
    }

    (size, smallest_above)
}

pub fn day07(test_mode: bool, print: bool) {
    let file_str = std::fs::read_to_string(format!("inputs/input{:02}.txt", DAY)).unwrap();
    let input_str = if test_mode {
        TEST_EXAMPLE
    } else {
        &file_str.trim_end()
    };

    // State
    let mut cwd = vec!["<unknown>"];
    let mut tree = FxHashMap::<String, Dir>::with_capacity_and_hasher(256, Default::default());

    let mut line_iter = input_str.lines().peekable();

    loop {
        // Command parsing
        let cmd_opt = line_iter.next();
        if let Some(cmd) = cmd_opt {
            let pieces: Vec<&str> = cmd.split(" ").collect();
            if pieces[1] == "ls" {
                // Fallthrough
            } else {
                // cd
                if pieces[2] == "/" {
                    cwd.truncate(0);
                    cwd.push("");
                } else if pieces[2] == ".." {
                    cwd.truncate(cwd.len() - 1);
                } else {
                    cwd.push(pieces[2]);
                }
            }
        } else {
            break;
        }

        let path = cwd.join("/");
        let dir = tree.entry(path).or_insert_with(Dir::new);

        // Output parsing
        loop {
            if let Some(line) = line_iter.peek() {
                if line.chars().next().unwrap() == '$' {
                    break;
                }

                let mut pieces = line.split(" ");
                let p1 = pieces.next().unwrap();
                let p2 = pieces.next().unwrap();
                if p1 == "dir" {
                    dir.subdirs.push(p2.to_string());
                } else {
                    dir.files.push((p2.to_string(), p1.parse().unwrap()));
                }
            } else {
                break;
            }
            line_iter.next();
        }
    }

    let mut part1 = 0;
    let used_size = walk_for_sizes(&tree, "", &mut part1);

    const DISK_SIZE: usize = 70000000;
    const DISK_NEED: usize = 30000000;

    let need_to_delete = DISK_NEED - (DISK_SIZE - used_size);
    let (_, part2) = find_smallest_above(&tree, need_to_delete, "");

    if print {
        println!("Day {}.  Part 1: {}", DAY, part1);
    }
    assert_eq!(part1, if test_mode { 95437 } else { 1428881 });

    if print {
        println!("Day {}.  Part 2: {}", DAY, part2);
    }
    assert_eq!(part2, if test_mode { 24933642 } else { 10475598 });
}

const TEST_EXAMPLE: &'static str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
