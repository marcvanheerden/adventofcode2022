use rayon::prelude::*;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let part1 = solution(&input, &['S']);
    let part2 = solution(&input, &['S', 'a']);
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

fn moves(map: &Map, y: usize, x: usize) -> VecDeque<(usize, usize)> {
    // list possible moves from a given position on a map
    let mut output = VecDeque::new();
    let ht = map.map[y][x];

    if x > 0 {
        output.push_front((y, x - 1));
    }

    if y > 0 {
        output.push_front((y - 1, x));
    }

    if x < (map.width - 1) {
        output.push_front((y, x + 1));
    }

    if y < (map.height - 1) {
        output.push_front((y + 1, x));
    }

    output
        .into_iter()
        .filter(|(y, x)| map.map[*y][*x] <= (ht + 1))
        .collect()
}

fn solution(input: &str, start_chars: &[char]) -> Option<usize> {
    let mut end = (0usize, 0usize);
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut starts: Vec<(usize, usize)> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        map.push(Vec::new());
        for (x, chr) in line.chars().enumerate() {
            if chr == 'E' {
                end = (y, x);
                map[y].push(b'z');
            } else if start_chars.contains(&chr) {
                map[y].push(b'a');
                starts.push((y, x));
            } else {
                map[y].push(chr as u8);
            }
        }
    }

    let height = map.len();
    let width = map[0].len();

    let map = Map { map, height, width };

    let dists: Vec<_> = starts
        .par_iter()
        .map(|start| {
            // breadth-first search
            let mut visited = HashSet::new();
            let mut dist = 0;
            let mut tasks: VecDeque<(usize, usize)> = VecDeque::new();
            tasks.push_front(*start);

            while !tasks.is_empty() {
                let mut new_tasks = VecDeque::new();
                while !tasks.is_empty() {
                    let task = tasks.pop_front().unwrap();
                    if task == end {
                        return dist;
                    }

                    if visited.contains(&task) {
                        continue;
                    }

                    visited.insert(task);
                    new_tasks.append(&mut moves(&map, task.0, task.1))
                }
                dist += 1;

                tasks.append(&mut new_tasks);
            }
            return usize::MAX;
        })
        .filter(|&u| u < usize::MAX)
        .collect();

    if dists.is_empty() {
        None
    } else {
        Some(dists.into_iter().min().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn example() {
        assert_eq!(solution(INPUT, &['S']), Some(31));
        assert_eq!(solution(INPUT, &['S', 'a']), Some(29));
    }
}
