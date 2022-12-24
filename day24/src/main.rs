use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::RangeInclusive;
use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let (part1, part2) = solution1(&input);
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

#[derive(Clone, Debug)]
enum Blizzard {
    Up,
    Down,
    Left,
    Right,
}


#[derive(Debug, Clone)]
struct Storm {
    blizzards: HashMap<(usize, usize), Vec<Blizzard>>,
    walls: HashSet<(usize, usize)>,
    row_range: RangeInclusive<usize>,
    col_range: RangeInclusive<usize>,
}

impl Storm {
    fn step(&mut self) {

        let next_pos: Vec<_> = self.blizzards.iter()
            .flat_map(|(key, val)| {
                val.iter()
                   .map(|b| {
                        let mut next = match b {
                            Blizzard::Up => (key.0 - 1, key.1),
                            Blizzard::Down => (key.0 + 1, key.1),
                            Blizzard::Left => (key.0, key.1 - 1),
                            Blizzard::Right => (key.0, key.1 + 1),
                        };
                    
                        next = if self.walls.contains(&next) {
                            match b {
                                Blizzard::Up =>  (self.row_range.end() - 1, next.1),
                                Blizzard::Down => (self.row_range.start() + 1, next.1),
                                Blizzard::Left => (next.0, self.col_range.end() - 1),
                                Blizzard::Right => (next.0, self.col_range.start() + 1),
                            }
                        } else {
                            next
                        };

                        (next, b.clone())
                    })
                    .collect::<Vec<((usize, usize), Blizzard)>>()

            })
            .collect();


        self.blizzards = HashMap::new();

        for pos in next_pos {
            if self.blizzards.contains_key(&pos.0) {
                let entry = self.blizzards.get_mut(&pos.0).unwrap();
                entry.push(pos.1);
            } else {
                self.blizzards.insert(pos.0, vec![pos.1]);
            }
        }
    }
}

fn preprocess(input: &str) -> Storm {
    let blizzards: HashMap<(usize, usize), Vec<Blizzard>> = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(col, chr)| match chr {
                    'v' => Some(((row, col), vec![Blizzard::Down])),
                    '>' => Some(((row, col), vec![Blizzard::Right])),
                    '^' => Some(((row, col), vec![Blizzard::Up])),
                    '<' => Some(((row, col), vec![Blizzard::Left])),
                    '.' => None,
                    '#' => None,
                    _ => panic!(),
                })
                .collect::<Vec<((usize, usize), Vec<Blizzard>)>>()
        })
        .collect();

    let walls: HashSet<(usize, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(col, chr)| match chr {
                    '#' => Some((row, col)),
                    'v' => None, 
                    '>' => None,
                    '^' => None,
                    '<' => None,
                    '.' => None,
                    _ => panic!(),
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let rowmin = walls.iter().map(|(row, _)| row).min().unwrap();
    let rowmax = walls.iter().map(|(row, _)| row).max().unwrap();
    let colmin = walls.iter().map(|(_, col)| col).min().unwrap();
    let colmax = walls.iter().map(|(_, col)| col).max().unwrap();

    let row_range = *rowmin..=*rowmax;
    let col_range = *colmin..=*colmax;

    Storm {
        blizzards,
        walls,
        row_range,
        col_range,
    }
}

fn solution1(input: &str) -> (usize, usize) {

    let mut storm = preprocess(input);

    let mut steps = 0usize;
    let mut stop = (0usize, 0usize);
    let mut positions = VecDeque::new();
    positions.push_front((0usize, 1usize));

    'outer: while !positions.is_empty() {
        storm.step();
        let mut new_positions = VecDeque::new();
        steps += 1;
        
        while !positions.is_empty() {
            let pos = positions.pop_front().unwrap();
            let mut new_pos = vec![pos, (pos.0, pos.1 - 1), (pos.0, pos.1 + 1),
                                    (pos.0 + 1, pos.1)];

            if pos.0 > 0 {
                new_pos.push((pos.0 - 1, pos.1));
            }

            for pos in new_pos.into_iter() {
                if !storm.walls.contains(&pos) & 
                    !storm.blizzards.contains_key(&pos) {
                    if pos.0 == *storm.row_range.end() {
                        stop = pos;
                        break 'outer
                    }
                    new_positions.push_back(pos);
                }
            }
        }
        new_positions = new_positions.into_iter()
            .unique()
            .sorted_by_key(|(row, col)| row * col)
            .rev()
            .take(100)
            .collect();

        positions.append(&mut new_positions);
    }
    
    let first_trip_steps = steps;

    dbg!(steps);
    let mut positions = VecDeque::new();
    positions.push_front(stop);
    
    'outer: while !positions.is_empty() {
        storm.step();
        let mut new_positions = VecDeque::new();
        steps += 1;
        
        while !positions.is_empty() {
            let pos = positions.pop_front().unwrap();
            let mut new_pos = vec![pos, (pos.0, pos.1 - 1), (pos.0, pos.1 + 1),
                                    (pos.0 + 1, pos.1)];

            if pos.0 > 0 {
                new_pos.push((pos.0 - 1, pos.1));
            }

            for pos in new_pos.into_iter() {
                if !storm.walls.contains(&pos) & 
                    !storm.blizzards.contains_key(&pos) & 
                    storm.row_range.contains(&pos.0) &
                    storm.col_range.contains(&pos.1) {
                    if pos.0 == *storm.row_range.start() {
                        stop = pos;
                        break 'outer
                    }
                    new_positions.push_back(pos);
                }
            }
        }
        new_positions = new_positions.into_iter()
            .unique()
            .sorted_by_key(|(row, col)| row * col)
            .take(1000)
            .collect();

        positions.append(&mut new_positions);
    }

    dbg!(steps);
    let mut positions = VecDeque::new();
    positions.push_front(stop);
    
    'outer: while !positions.is_empty() {
        storm.step();
        let mut new_positions = VecDeque::new();
        steps += 1;
        
        while !positions.is_empty() {
            let pos = positions.pop_front().unwrap();
            let mut new_pos = vec![pos, (pos.0, pos.1 - 1), (pos.0, pos.1 + 1),
                                    (pos.0 + 1, pos.1)];

            if pos.0 > 0 {
                new_pos.push((pos.0 - 1, pos.1));
            }

            for pos in new_pos.into_iter() {
                if !storm.walls.contains(&pos) & 
                    !storm.blizzards.contains_key(&pos) {
                    if pos.0 == *storm.row_range.end() {
                        break 'outer
                    }
                    new_positions.push_back(pos);
                }
            }
        }
        new_positions = new_positions.into_iter()
            .unique()
            .sorted_by_key(|(row, col)| row * col)
            .rev()
            .take(1000)
            .collect();

        positions.append(&mut new_positions);
    }
    dbg!(steps);

    (first_trip_steps, steps)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn example() {
        assert_eq!(solution1(INPUT), (18, 54 + 1));
    }
}
