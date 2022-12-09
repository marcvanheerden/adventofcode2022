use std::cmp::{max, Ordering};
use std::collections::HashSet;
use std::fs;
use std::str::FromStr;
use std::thread;

enum Dir {
    U,
    D,
    L,
    R,
}

struct Vector {
    dir: Dir,
    mag: u32,
}

impl FromStr for Vector {
    type Err = ();
    fn from_str(input: &str) -> Result<Vector, Self::Err> {
        let splits = input.split_once(' ').unwrap();

        let dir = match splits.0 {
            "U" => Dir::U,
            "D" => Dir::D,
            "L" => Dir::L,
            "R" => Dir::R,
            _ => panic!("unknown input"),
        };

        let mag = splits.1.parse::<u32>().expect("incorrect input");

        Ok(Vector { dir, mag })
    }
}

fn main() {
    //big_input();
    let input: String = fs::read_to_string("day09_input1000").unwrap();
    let moves: Vec<_> = input
        .lines()
        .map(|l| Vector::from_str(l).unwrap())
        .collect();

    let mut part1 = 0u32;
    let mut part2 = 0u32;

    thread::scope(|s| {
        let thread1 = s.spawn(|| solution1(&moves));
        part2 = solution2(&moves, 10);
        part1 = thread1.join().unwrap();
    });

    println!("Part1: {:?} \nPart2: {:?}", part1, part2);
}

fn update_pos(refer: &(i32, i32), curr: &(i32, i32)) -> (i32, i32) {
    let y_dist: i32 = refer.0 - curr.0;
    let x_dist: i32 = refer.1 - curr.1;
    let mut output = *curr;
    if max(y_dist.abs(), x_dist.abs()) > 1 {
        match refer.0.cmp(&curr.0) {
            Ordering::Less => output.0 -= 1,
            Ordering::Equal => (),
            Ordering::Greater => output.0 += 1,
        }
        match refer.1.cmp(&curr.1) {
            Ordering::Less => output.1 -= 1,
            Ordering::Equal => (),
            Ordering::Greater => output.1 += 1,
        }
    }
    output
}

fn solution1(moves: &[Vector]) -> u32 {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_pos = HashSet::new();

    for vect in moves.iter() {
        for _ in 0..vect.mag {
            // move head
            match vect.dir {
                Dir::U => head.0 += 1,
                Dir::D => head.0 -= 1,
                Dir::L => head.1 -= 1,
                Dir::R => head.1 += 1,
            };
            tail = update_pos(&head, &tail);

            tail_pos.insert(tail);
        }
    }

    tail_pos.len() as u32
}

fn solution2(moves: &[Vector], knots: usize) -> u32 {
    let mut positions = vec![(0, 0); knots];
    let mut tail_pos = HashSet::new();

    for vect in moves.iter() {
        for _ in 0..vect.mag {
            // move head
            match vect.dir {
                Dir::U => positions[0].0 += 1,
                Dir::D => positions[0].0 -= 1,
                Dir::L => positions[0].1 -= 1,
                Dir::R => positions[0].1 += 1,
            }
            for idx in 1..knots {
                positions[idx] = update_pos(&positions[idx - 1], &positions[idx]);
            }

            tail_pos.insert(positions[knots - 1]);
        }
    }

    tail_pos.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let moves: Vec<_> = input
            .lines()
            .map(|l| Vector::from_str(l).unwrap())
            .collect();

        let part1 = solution1(&moves);
        assert_eq!(part1, 13);
    }

    #[test]
    fn example2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let moves: Vec<_> = input
            .lines()
            .map(|l| Vector::from_str(l).unwrap())
            .collect();

        let part2 = solution2(&moves, 10);
        assert_eq!(part2, 36);
    }
}
