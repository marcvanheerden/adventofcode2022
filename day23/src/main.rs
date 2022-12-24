use fxhash::FxHashSet;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let (part1, part2) = solution1(&input, 10);
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

#[derive(Clone, Debug)]
enum Dir {
    N,
    S,
    W,
    E,
}

fn next_pos(
    row: &isize,
    col: &isize,
    occupation: &FxHashSet<(isize, isize)>,
    order: &VecDeque<Dir>,
) -> (isize, isize) {
    let nbors = [
        (row - 1, *col),
        (row + 1, *col),
        (*row, col + 1),
        (*row, col - 1),
        (row - 1, col - 1),
        (row + 1, col - 1),
        (row - 1, col + 1),
        (row + 1, col + 1),
    ];

    if nbors.iter().all(|p| !occupation.contains(p)) {
        return (*row, *col);
    }

    for dir in order {
        let candidates = match dir {
            Dir::N => [(row - 1, col - 1), (row - 1, *col), (row - 1, col + 1)],
            Dir::S => [(row + 1, col - 1), (row + 1, *col), (row + 1, col + 1)],
            Dir::W => [(row - 1, col - 1), (*row, col - 1), (row + 1, col - 1)],
            Dir::E => [(row - 1, col + 1), (*row, col + 1), (row + 1, col + 1)],
        };

        if candidates.iter().all(|p| !occupation.contains(p)) {
            return candidates[1];
        }
    }

    (*row, *col)
}

fn duplicates(positions: &[(isize, isize)]) -> FxHashSet<(isize, isize)> {
    let mut all = FxHashSet::default();
    let mut dup = FxHashSet::default();

    for pos in positions.iter() {
        if all.contains(pos) {
            dup.insert(*pos);
        } else {
            all.insert(pos);
        }
    }

    dup
}

fn solution1(input: &str, screenshot: isize) -> (isize, isize) {
    let mut elves: FxHashSet<(isize, isize)> = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(col, chr)| match chr {
                    '#' => Some((row as isize, col as isize)),
                    '.' => None,
                    _ => {
                        panic!()
                    }
                })
                .collect::<Vec<(isize, isize)>>()
        })
        .collect();

    let mut dirs: VecDeque<Dir> = [Dir::N, Dir::S, Dir::W, Dir::E].iter().cloned().collect();
    let mut screenshot_output = 0isize;
    let mut step = 1isize;

    let mut next_positions = Vec::with_capacity(elves.len());

    loop {
        next_positions = elves
            .iter()
            .map(|(row, col)| next_pos(row, col, &elves, &dirs))
            .collect();

        let dups = duplicates(&next_positions);

        let new_elves = elves
            .iter()
            .zip(next_positions.iter())
            .map(|(p0, p1)| if dups.contains(p1) { *p0 } else { *p1 })
            .collect();

        if same(&elves, &new_elves) {
            break;
        } else {
            elves = new_elves;
        }

        if step == screenshot {
            let row_min = elves.iter().map(|e| e.0).min().unwrap();
            let row_max = elves.iter().map(|e| e.0).max().unwrap();
            let col_min = elves.iter().map(|e| e.1).min().unwrap();
            let col_max = elves.iter().map(|e| e.1).max().unwrap();

            screenshot_output =
                (row_max - row_min + 1) * (col_max - col_min + 1) - elves.len() as isize;
        }
        dirs.rotate_left(1);
        step += 1;
    }

    (screenshot_output, step)
}

fn same(hash1: &FxHashSet<(isize, isize)>, hash2: &FxHashSet<(isize, isize)>) -> bool {
    // assumes equal length, only have to check one way

    for val in hash1 {
        if !hash2.contains(val) {
            return false;
        }
    }

    true
}

fn display(elves: &FxHashSet<(isize, isize)>) {
    let row_min = elves.iter().map(|e| e.0).min().unwrap();
    let row_max = elves.iter().map(|e| e.0).max().unwrap();
    let col_min = elves.iter().map(|e| e.1).min().unwrap();
    let col_max = elves.iter().map(|e| e.1).max().unwrap();

    for row in row_min..=row_max {
        let mut row_str = "".to_string();
        for col in col_min..=col_max {
            match elves.contains(&(row, col)) {
                true => row_str.push('#'),
                false => row_str.push('.'),
            };
        }
        println!("{}", row_str);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT2: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    const INPUT: &str = ".....
..##.
..#..
.....
..##.
.....";

    #[test]
    fn example() {
        assert_eq!(solution1(INPUT2, 10), (110, 20));
    }
}
