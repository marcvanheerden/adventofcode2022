use fxhash::FxHashSet;
use std::cmp::{max, Ordering};
use std::fs;
use std::str::FromStr;
use std::thread;

fn main() {
    let input: String = fs::read_to_string("day09_input1000").unwrap();
    let moves: Vec<_> = input
        .lines()
        .map(|l| Vector::from_str(l).unwrap())
        .collect();

    let mut part1 = 0usize;
    let mut part2 = 0usize;

    thread::scope(|s| {
        let thread1 = s.spawn(|| solution(&moves, 2));
        part2 = solution(&moves, 10);
        part1 = thread1.join().unwrap();
    });

    println!("Part1: {:?} \nPart2: {:?}", part1, part2);
}

// Direction options
enum Dir {
    U,
    D,
    L,
    R,
}

// Vector with direction and magnitude
struct Vector {
    dir: Dir,
    mag: u32,
}

// Convert a single line into a Vector
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

// update a tail knot position based on it's head position
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

fn solution(moves: &[Vector], knots: usize) -> usize {
    let mut positions = vec![(0, 0); knots];
    let mut tail_pos = FxHashSet::default();

    for vect in moves.iter() {
        for _ in 0..vect.mag {
            // move head
            match vect.dir {
                Dir::U => positions[0].0 += 1,
                Dir::D => positions[0].0 -= 1,
                Dir::L => positions[0].1 -= 1,
                Dir::R => positions[0].1 += 1,
            }

            // move tail(s)
            for idx in 1..knots {
                positions[idx] = update_pos(&positions[idx - 1], &positions[idx]);
            }

            // record tail position
            tail_pos.insert(positions[knots - 1]);
        }
    }

    tail_pos.len()
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

        let part1 = solution(&moves, 2);
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

        let part2 = solution(&moves, 10);
        assert_eq!(part2, 36);
    }
}
