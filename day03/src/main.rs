use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;

const UPPERCASE_START: u8 = 96;
const LOWERCASE_START: u8 = 64;

fn main() {
    let input = fs::read_to_string("input10000").unwrap();
    let answer = solution(&input, 3);
    println!("Part1: {} \nPart2: {}", answer.0, answer.1);
}

fn prior(item: char) -> u32 {
    let ascii = item as u8;
    if ascii > UPPERCASE_START {
        return (ascii - UPPERCASE_START) as u32;
    }

    ((ascii - LOWERCASE_START) as u32) + 26
}

fn solution(contents: &str, seg_size: usize) -> (u32, u32) {
    let sacks: Vec<&str> = contents.lines().collect();
    let segments: Vec<usize> = (0..(sacks.len() / seg_size)).into_iter().collect();

    // split out segments and process in parallel
    segments
        .par_iter()
        .map(|idx| {
            let group_sacks = &sacks[(idx * seg_size)..((idx + 1) * seg_size)];

            let part1 = group_sacks
                .iter()
                .map(|s| {
                    let length = s.len();
                    let mut record = HashSet::new();
                    for (idx, chr) in s.chars().enumerate() {
                        if idx < (length / 2) {
                            record.insert(chr);
                        } else if record.contains(&chr) {
                            return prior(chr);
                        }
                    }
                    0u32
                })
                .sum();

            // part 2
            let mut record = HashSet::new();
            let mut record2 = HashSet::new();

            for chr in group_sacks[0].chars() {
                record.insert(chr);
            }

            for chr in group_sacks[1].chars() {
                if record.contains(&chr) {
                    record2.insert(chr);
                }
            }

            for chr in group_sacks[2].chars() {
                if record2.contains(&chr) {
                    return (part1, prior(chr));
                }
            }
            (0u32, 0u32)
        })
        .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    #[test]
    fn priority() {
        assert_eq!(prior('p'), 16);
        assert_eq!(prior('L'), 38);
    }

    #[test]
    fn example() {
        assert_eq!(solution(INPUT, 3), (157, 70));
    }
}
