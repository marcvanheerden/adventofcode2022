use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let mut input = fs::read_to_string("input10000").unwrap();
    input.push('\n');
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input, 3));
}

fn part1(cals: &str) -> u32 {
    let mut max = 0;
    let mut run = 0;

    for line in cals.lines() {
        if line.is_empty() {
            if run > max {
                max = run;
            }
            run = 0;
            continue;
        }

        run += line.parse::<u32>().unwrap();
    }

    max
}

fn part2(cals: &str, top_n: usize) -> u32 {
    let mut top = BinaryHeap::new();
    let mut run = 0;

    for line in cals.lines() {
        if line.is_empty() {
            if top.len() < top_n {
                top.push(Reverse(run));
            } else if top.peek().unwrap() > &Reverse(run) {
                top.pop();
                top.push(Reverse(run));
            }
            run = 0;
            continue;
        }

        run += line.parse::<u32>().unwrap();
    }

    top.iter()
        .map(|u| {
            let Reverse(v) = u;
            v
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 24000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT, 3), 45000);
    }
}
