use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let part1 = solution1(&input);
    println!("Part1: {:?}\nPart2: {:?}", part1, part1);
}

fn solution1(input: &str) -> usize {
    let map = get_map(input);
    
    let mut useful = HashSet::new();
    let mut visited = HashMap::new();
    for (key, val) in &map {
        let key_ = key.clone();
        if val.0 > 0 {
            useful.insert(key_.clone());
        }
        visited.insert(key_, 0usize);
    }
    
    dfs(&map, "AA", 30, &visited, &useful)
}

type Map = HashMap<String, (usize, Vec<String>)>;

fn dfs(
    map: &Map,
    loc: &str,
    steps: usize,
    visited: &HashMap<String, usize>,
    useful: &HashSet<String>,
) -> usize {
    // assume will always open if flow rate > 0
    

    let monitor_path = [("DD", 29usize), 
                        ("CC", 26),
                        ("AA", 24)];
    
    if monitor_path.contains(&(loc, steps)) {
        dbg!("---------------------------");
        dbg!(loc);
        dbg!(steps);
        dbg!(useful);
        dbg!(visited);
    }

    let mut visited = visited.clone();
    let counter = visited.get_mut(loc).unwrap();
    *counter += 1;

    let mut useful = useful.clone();
    let mut steps = steps.clone();

    let pos = map.get(&loc.to_owned()).unwrap();
    let mut score = 0;

    if (pos.0 > 0) & useful.contains(loc) {
        steps -= 1;
        score += steps * pos.0;
        useful.remove(&loc.to_owned());
    }
    steps -= 1;

    if !useful.is_empty() & (steps > 1) {
        if let Some(downstream) = pos
            .1
            .iter()
            .filter(|s| visited.get(*s).unwrap() < &2)
            .map(|s| dfs(map, s, steps, &visited, &useful))
            .max() {
            score += downstream;
        }
    }

    score
}

fn get_map(input: &str) -> Map {
    let mut valves: HashMap<String, (usize, Vec<String>)> = HashMap::new();

    for line in input.lines() {
        let splits: Vec<_> = line.split_whitespace().collect();
        let name = splits[1].to_owned();
        let mut moves: Vec<String> = Vec::new();

        for split in splits.iter().rev() {
            match split.len() {
                2 => moves.push((*split).to_owned()),
                3 => {
                    moves.push(split.chars().take(2).collect());
                }
                _ => break,
            }
        }

        let mut rate = "".to_string();

        for chr in splits[4].chars() {
            if chr.is_digit(10) {
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
}
