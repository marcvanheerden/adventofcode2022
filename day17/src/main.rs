use fxhash::FxHashSet;
use std::cmp::max;
use std::collections::VecDeque;
use std::fs;

const ROCKS: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##";

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let max_heights = solution1(&input, 100000, 7);
    let part2 = forecast(&max_heights, 1000000000000, 2000, 2000);
    println!("Part1: {:?}\nPart2: {:?}", max_heights[2021], part2);
}

fn forecast(vals: &[usize], target: usize, start_max: usize, period_max: usize) -> usize {
    let (cycle_start, cycle_period) = find_cycle(vals, start_max, period_max).unwrap();
    let step = vals[cycle_start + cycle_period] - vals[cycle_start];
    let fill_in = ((target - cycle_start) / cycle_period) * step;
    vals[(target - cycle_start) % cycle_period + cycle_start - 1] + fill_in
}

fn find_cycle(vals: &[usize], start_max: usize, period_max: usize) -> Option<(usize, usize)> {
    let mut last_gap = 0usize;
    let mut current_gap: usize;
    let mut reset: bool;
    let mut idx: usize;
    let mut answer = None;

    'a: for start in 0..=start_max {
        'b: for period in 2..=period_max {
            if (start + 3 * period) > (vals.len() - 1) {
                break;
            }
            idx = 0;
            reset = true;
            while (start + (idx + 1) * period) < vals.len() {
                current_gap = vals[(start + (idx + 1) * period)] - vals[start + idx * period];
                if reset {
                    reset = false;
                } else if current_gap != last_gap {
                    continue 'b;
                }
                last_gap = current_gap;
                idx += 1;
            }
            answer = Some((start, period));
            break 'a;
        }
    }

    answer
}

fn parse_rocks(input: &str) -> VecDeque<Vec<(usize, usize)>> {
    let mut output = VecDeque::new();

    for rock_pattern in input.split("\n\n") {
        let mut repres = Vec::new();

        for (y, line) in rock_pattern.lines().rev().enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr == '#' {
                    repres.push((x + 3, y));
                }
            }
        }
        output.push_back(repres);
    }

    output
}

fn display(settled: &FxHashSet<(usize, usize)>, current: &[(usize, usize)]) {
    let settled_max = settled.iter().map(|(_, y)| y).max().unwrap();
    let current_max = current.iter().map(|(_, y)| y).max().unwrap();
    let max = max(settled_max, current_max);

    let mut lines = Vec::new();

    for row in 0..=*max {
        let mut output = "".to_owned();
        for col in 1..=7usize {
            if settled.contains(&(col, row)) {
                output.push('#');
            } else if current.contains(&(col, row)) {
                output.push('@');
            } else {
                output.push('.');
            }
        }
        lines.push(output);
    }

    for line in lines.into_iter().rev() {
        println!("{}", line);
    }
}

fn solution1(moves: &str, nrocks: usize, width: usize) -> Vec<usize> {
    let mut mover: VecDeque<char> = moves.chars().filter(|&c| c != '\n').collect();
    let mut rock_counter = 0usize;

    // track settled rocks and insert floor
    let mut settled: FxHashSet<_> = (1..=width).map(|u| (u, 0)).collect();

    let mut rocks = parse_rocks(ROCKS);
    let mut max_height = settled.iter().map(|(_, y)| y).max().unwrap();
    let mut current_rock: Vec<_> = rocks[0]
        .clone()
        .into_iter()
        .map(|(x, y)| (x, y + max_height + 4))
        .collect();
    rocks.rotate_left(1);

    let mut max_heights = Vec::new();

    while rock_counter < nrocks {
        // horizontal move
        let adj = match mover[0] {
            '<' => -1,
            '>' => 1,
            _ => {
                dbg!(&mover[0]);
                panic!()
            }
        };
        mover.rotate_left(1);

        let new_pos: Vec<_> = current_rock
            .iter()
            .map(|(x, y)| (x.checked_add_signed(adj).unwrap(), *y))
            .collect();

        let collision = new_pos.iter().any(|(x, y)| {
            // hits side wall
            if (*x < 1) | (*x > width) {
                return true;
            }

            // hits settled rocks
            settled.contains(&(*x, *y))
        });

        if !collision {
            current_rock = new_pos;
        }

        // vertical move
        let new_pos: Vec<_> = current_rock.iter().map(|(x, y)| (*x, *y - 1)).collect();

        let collision = new_pos.iter().any(|(x, y)| settled.contains(&(*x, *y)));

        if collision {
            // write current rock to settled
            for piece in current_rock {
                settled.insert(piece);
            }
            rock_counter += 1;

            // spawn a new rock
            max_height = settled.iter().map(|(_, y)| y).max().unwrap();
            max_heights.push(*max_height);

            current_rock = rocks[0]
                .clone()
                .into_iter()
                .map(|(x, y)| (x, y + max_height + 4))
                .collect();
            rocks.rotate_left(1);

            // shrinking the hashset by removing irrelevant blocks
            if settled.len() > 1000 {
                let cutoff = max_height - 100;
                settled.retain(|(_, y)| y > &cutoff);
            }
        } else {
            current_rock = new_pos;
        }
    }

    max_heights
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOVES: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn example() {
        assert_eq!(solution1(MOVES, 2022, 7)[2021], 3068);
    }

    #[test]
    fn example2() {
        let max_heights = solution1(MOVES, 100000, 7);
        assert_eq!(
            forecast(&max_heights, 1000000000000, 2000, 3000),
            1514285714288
        );
    }

    #[test]
    fn find_cycle_test() {
        assert_eq!(
            find_cycle(&[1, 2, 3, 4, 6, 10, 15, 16, 17, 22], 5, 3),
            Some((3, 2))
        );
    }
}
