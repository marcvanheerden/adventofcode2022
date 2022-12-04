use rayon::prelude::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("input10000").unwrap();
    let answer = solution(&input);
    println!("Part1: {} \nPart2: {}", answer.0, answer.1);
}

fn solution(intervals: &str) -> (u32, u32) {
    intervals
        .par_lines()
        .map(|s| {
            let mut overlaps1 = 0u32;
            let mut overlaps2 = 0u32;
            let mut elves = s.split(',');
            let mut elf1 = elves.next().unwrap().split('-');
            let mut elf2 = elves.next().unwrap().split('-');

            let elf1_start: u32 = elf1.next().unwrap().parse().unwrap();
            let elf1_end: u32 = elf1.next().unwrap().parse().unwrap();
            let elf2_start: u32 = elf2.next().unwrap().parse().unwrap();
            let elf2_end: u32 = elf2.next().unwrap().parse().unwrap();

            if ((elf1_start <= elf2_start) & (elf1_end >= elf2_end))
                | ((elf2_start <= elf1_start) & (elf2_end >= elf1_end))
            {
                overlaps1 += 1;
            }

            for val in elf2_start..=elf2_end {
                if (elf1_start..=elf1_end).contains(&val) {
                    overlaps2 += 1;
                    break;
                }
            }

            (overlaps1, overlaps2)
        })
        .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn example() {
        assert_eq!(solution(INPUT), (2, 4));
    }
}
