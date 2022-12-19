use std::cmp::max;
use std::collections::HashMap;
use std::fs;
use std::thread;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let mut part1 = 0usize;
    let mut part2 = 0usize;

    thread::scope(|s| {
        let thread1 = s.spawn(|| {
            solution1(&input)
        });
        part2 = solution2(&input);
        part1 = thread1.join().unwrap();
    });

    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

#[derive(Debug)]
struct Task {
    location: usize,
    time: usize,
    score: usize,
    open_valves: Vec<usize>,
}

#[derive(Debug)]
struct DoubleTask {
    location: (usize, usize), // (my location, elephant location)
    time: usize,
    score: usize,
    open_valves: Vec<usize>,
}

fn solution2(input: &str) -> usize {
    let map = get_map(input);

    let mut useful_valves = HashMap::new();

    for (key, val) in &map {
        let key_ = *key;
        if val.0 > 0 {
            useful_valves.insert(key_, val.0);
        }
    }

    let mut overall_best_score = 0usize;

    // task format (position, time, score, open_valves)
    let mut tasks = vec![DoubleTask {
        location: (6565, 6565), // These are both AA
        time: 26,
        score: 0,
        open_valves: Vec::new(),
    }];

    loop {
        let best_score = tasks.iter().map(|t| t.score).max().unwrap();
        overall_best_score = max(overall_best_score, best_score);

        // filter out paths that could never catch up to the score leader
        tasks.retain(|t| {
            let mut potential_score = 0usize;
            for (key, val) in useful_valves.clone() {
                if !t.open_valves.contains(&key) {
                    potential_score += val * (t.time - 1);
                }
            }

            (t.score + potential_score) > best_score
        });

        tasks.sort_by_key(|k| k.score);
        tasks = tasks.into_iter().rev().take(1000).collect();

        let mut new_tasks: Vec<DoubleTask> = Vec::new();
        for _ in 0..tasks.len() {
            let task = tasks.pop().unwrap();
            let node_me = map.get(&task.location.0).unwrap();
            let node_el = map.get(&task.location.1).unwrap();

            // both pass through without opening a valve
            for location_me in &node_me.1 {
                for location_el in &node_el.1 {
                    let new_task = DoubleTask {
                        location: (*location_me, *location_el),
                        time: task.time - 1,
                        score: task.score,
                        open_valves: task.open_valves.clone(),
                    };
                    new_tasks.push(new_task);
                }
            }

            let can_open_me = (node_me.0 > 0) & !task.open_valves.contains(&task.location.0);
            let can_open_el = (node_el.0 > 0) & !task.open_valves.contains(&task.location.1);

            // neither me nor the elephant are at usable valves
            if !can_open_me & !can_open_el {
                continue;
            }

            // elephant opens a valve, I move to a new location
            if can_open_el {
                let mut new_open_valves = task.open_valves.clone();
                new_open_valves.push(task.location.1);
                let time = task.time - 1;
                let score = task.score + node_el.0 * time;

                for location in node_me.1.iter() {
                    let new_task = DoubleTask {
                        location: (*location, task.location.1),
                        time,
                        score,
                        open_valves: new_open_valves.clone(),
                    };
                    new_tasks.push(new_task);
                }
            }

            // I open a valve, elephant moves to a new location
            if can_open_me {
                let mut new_open_valves = task.open_valves.clone();
                new_open_valves.push(task.location.0);
                let time = task.time - 1;
                let score = task.score + node_me.0 * time;

                for location in node_el.1.iter() {
                    let new_task = DoubleTask {
                        location: (task.location.0, *location),
                        time,
                        score,
                        open_valves: new_open_valves.clone(),
                    };
                    new_tasks.push(new_task);
                }
            }

            // both the elephant and I open valves
            if can_open_me & can_open_el & (task.location.0 != task.location.1) {
                let mut new_open_valves = task.open_valves.clone();
                new_open_valves.push(task.location.0);
                new_open_valves.push(task.location.1);
                let time = task.time - 1;
                let score = task.score + (node_me.0 + node_el.0) * time;

                let new_task = DoubleTask {
                    location: (task.location.0, task.location.1),
                    time,
                    score,
                    open_valves: new_open_valves.clone(),
                };
                new_tasks.push(new_task);
            }
        }
        if new_tasks.is_empty() {
            break;
        }
        tasks.append(&mut new_tasks);
    }

    overall_best_score
}

fn solution1(input: &str) -> usize {
    let map = get_map(input);

    let mut useful_valves = HashMap::new();

    for (key, val) in &map {
        let key_ = *key;
        if val.0 > 0 {
            useful_valves.insert(key_, val.0);
        }
    }

    let mut overall_best_score = 0usize;

    // task format (position, time, score, open_valves)
    let mut tasks = vec![Task {
        location: 6565, // this is AA
        time: 30,
        score: 0,
        open_valves: Vec::new(),
    }];

    loop {
        let best_score = tasks.iter().map(|t| t.score).max().unwrap();
        overall_best_score = max(overall_best_score, best_score);

        // filter out paths that could never catch up to the score leader
        tasks.retain(|t| {
            let mut potential_score = 0usize;
            for (key, val) in useful_valves.clone() {
                if !t.open_valves.contains(&key) {
                    potential_score += val * (t.time - 1);
                }
            }

            (t.score + potential_score) > best_score
        });

        tasks.sort_by_key(|k| k.score);
        tasks = tasks.into_iter().rev().take(1000).collect();

        let mut new_tasks: Vec<Task> = Vec::new();
        for _ in 0..tasks.len() {
            let task = tasks.pop().unwrap();
            let node = map.get(&task.location).unwrap();

            // pass through without opening a valve
            for location in &node.1 {
                let new_task = Task {
                    location: *location,
                    time: task.time - 1,
                    score: task.score,
                    open_valves: task.open_valves.clone(),
                };
                new_tasks.push(new_task);
            }

            // for all children add tasks with opening valve
            // if valve has value and is not already open
            if (node.0 > 0) & !task.open_valves.contains(&task.location) {
                let mut new_open_valves = task.open_valves.clone();
                new_open_valves.push(task.location);
                let time = task.time - 1;
                let score = task.score + node.0 * time;

                for location in node.1.iter() {
                    let new_task = Task {
                        location: *location,
                        time: time - 1,
                        score,
                        open_valves: new_open_valves.clone(),
                    };
                    new_tasks.push(new_task);
                }
            }
        }
        if new_tasks.is_empty() {
            break;
        }
        tasks.append(&mut new_tasks);
    }

    overall_best_score
}

type Map = HashMap<usize, (usize, Vec<usize>)>;

fn get_map(input: &str) -> Map {
    let mut valves: Map = HashMap::new();

    for line in input.lines() {
        let splits: Vec<_> = line.split_whitespace().collect();
        let name = splits[1].bytes().take(2).collect::<Vec<u8>>();
        let name = (name[0] as usize) * 100 + name[1] as usize;
        let mut moves: Vec<usize> = Vec::new();

        for split in splits.iter().rev() {
            match split.len() {
                2 | 3 => {
                    let name: Vec<u8> = split.bytes().take(2).collect();
                    moves.push((name[0] as usize) * 100 + name[1] as usize);
                }
                _ => break,
            }
        }

        let mut rate = "".to_string();

        for chr in splits[4].chars() {
            if chr.is_ascii_digit() {
                rate.push(chr);
            }
        }

        valves.insert(name, (rate.parse().unwrap(), moves));
    }

    valves
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn example() {
        assert_eq!(solution1(INPUT), 1651);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 1707);
    }
}
