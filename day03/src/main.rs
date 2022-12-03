use std::fs;
use std::collections::HashSet;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("input10000").unwrap();
    let answer = solution(&input);
    println!("Part1: {} \nPart2: {}", answer, solution2(&input, 3));
}

fn prior(item: char) -> u32 {
    let ascii = item as u8;
    if ascii > 96 {
        return ascii as u32 - 96
    }

    ascii as u32 - 64 + 26
}

fn solution(contents: &str) -> u32 {

    contents.par_lines()
        .map(|s| {
            let length = s.len();
            let mut record = HashSet::new();
            for (idx, chr) in s.chars().enumerate() {
                if idx < (length / 2) {
                    record.insert(chr);
                } else {
                    if record.contains(&chr) {
                        return prior(chr)
                    }
                }
            }
            0u32
        })
        .sum()
}

fn solution2(contents: &str, seg_size: usize) -> u32 {

    let sacks: Vec<&str> = contents.lines().collect();
    let segments: Vec<usize> = (0..(sacks.len() / seg_size)).into_iter().collect();

    segments.par_iter()
            .map(|idx| {
        let mut record = HashSet::new();
        let mut record2 = HashSet::new();

        let group_sacks = &sacks[(idx*seg_size)..((idx + 1) * seg_size)];

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
                return prior(chr)
            }
        }
        0u32
    }).sum()
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
        assert_eq!(solution(INPUT), 157);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT, 3), 70);
    }
}
