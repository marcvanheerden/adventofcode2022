use std::collections::HashSet;
use std::fs;
use std::thread;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let occupied = get_rock_paths(&input);

    let mut part1 = 0usize;
    let mut part2 = 0usize;

    thread::scope(|s| {
        let thread1 = s.spawn(|| solution1(&occupied));
        part2 = solution2(&occupied);
        part1 = thread1.join().unwrap();
    });
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

fn get_rock_paths(input: &str) -> HashSet<(usize, usize)> {
    let mut occupied: HashSet<(usize, usize)> = HashSet::new();

    let rock_paths: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|s| {
                    let unparsed = s.trim().split_once(',').unwrap();
                    (unparsed.0.parse().unwrap(), unparsed.1.parse().unwrap())
                })
                .collect()
        })
        .collect();

    for path in rock_paths.into_iter() {
        for idx in 0..(path.len() - 1) {
            let start = path[idx];
            let end = path[idx + 1];

            if start.0 < end.0 {
                for x in start.0..=end.0 {
                    occupied.insert((x, start.1));
                }
            } else if end.0 < start.0 {
                for x in end.0..=start.0 {
                    occupied.insert((x, start.1));
                }
            } else if start.1 < end.1 {
                for y in start.1..=end.1 {
                    occupied.insert((start.0, y));
                }
            } else {
                for y in end.1..=start.1 {
                    occupied.insert((start.0, y));
                }
            }
        }
    }

    occupied
}

fn solution1(occupied: &HashSet<(usize, usize)>) -> usize {
    let mut occupied = occupied.clone();
    let sand_origin = (500usize, 0usize);
    let max_depth = occupied.iter().cloned().map(|(_, y)| y).max().unwrap();
    let mut total_sand = 0usize;
    let mut sand_path = vec![sand_origin];
    let mut part: (usize, usize);

    loop {
        part = *sand_path.last().unwrap();
        if part.1 == max_depth + 1 {
            break;
        } else if !occupied.contains(&(part.0, part.1 + 1)) {
            part.1 += 1;
            sand_path.push(part);
        } else if !occupied.contains(&(part.0 - 1, part.1 + 1)) {
            part.1 += 1;
            part.0 -= 1;
            sand_path.push(part);
        } else if !occupied.contains(&(part.0 + 1, part.1 + 1)) {
            part.1 += 1;
            part.0 += 1;
            sand_path.push(part);
        } else {
            occupied.insert(part);
            total_sand += 1;
            sand_path.pop();
        }
    }

    total_sand
}
fn solution2(occupied: &HashSet<(usize, usize)>) -> usize {
    let mut occupied = occupied.clone();
    let sand_origin = (500usize, 0usize);
    let max_depth = occupied.iter().cloned().map(|(_, y)| y).max().unwrap();
    let mut total_sand = 0usize;
    let mut sand_path = vec![sand_origin];
    let mut part: (usize, usize);

    while !occupied.contains(&sand_origin) {
        part = *sand_path.last().unwrap();
        if part.1 == max_depth + 1 {
            occupied.insert(part);
            total_sand += 1;
            sand_path.pop();
        } else if !occupied.contains(&(part.0, part.1 + 1)) {
            part.1 += 1;
            sand_path.push(part);
        } else if !occupied.contains(&(part.0 - 1, part.1 + 1)) {
            part.1 += 1;
            part.0 -= 1;
            sand_path.push(part);
        } else if !occupied.contains(&(part.0 + 1, part.1 + 1)) {
            part.1 += 1;
            part.0 += 1;
            sand_path.push(part);
        } else {
            occupied.insert(part);
            total_sand += 1;
            sand_path.pop();
        }
    }

    total_sand
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn example() {
        let map = get_rock_paths(INPUT);
        assert_eq!(solution1(&map), 24);
    }
    #[test]
    fn example2() {
        let map = get_rock_paths(INPUT);
        assert_eq!(solution2(&map), 93);
    }
}
