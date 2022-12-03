use itertools::Itertools;
use std::fs;

const UPPERCASE_START: u8 = 96;
const LOWERCASE_START: u8 = 64;
const ALPHABET_LEN: u32 = 26;
const SEG_SIZE: usize = 3;
const MAX_ASCII: usize = 123;

fn main() {
    let input = fs::read_to_string("input10000").unwrap();
    let answer = solution(&input);
    println!("Part1: {} \nPart2: {}", answer.0, answer.1);
}

fn prior(item: char) -> u32 {
    let ascii = item as u8;
    if ascii > UPPERCASE_START {
        return (ascii - UPPERCASE_START) as u32;
    }

    ((ascii - LOWERCASE_START) as u32) + ALPHABET_LEN
}

fn solution(contents: &str) -> (u32, u32) {
    let mut buffer = [[false; MAX_ASCII]; SEG_SIZE];
    let mut total_pt1 = 0u32;
    let mut total_pt2 = 0u32;

    for chunk in &contents.lines().chunks(SEG_SIZE) {
        for (idx, line) in chunk.enumerate() {
            // part 1
            let div = line.len() / 2;
            for (idx, chr) in line.chars().enumerate() {
                if idx < div {
                    buffer[0][chr as usize] = true;
                } else if buffer[0][chr as usize] {
                    total_pt1 += prior(chr);
                    break;
                }
            }
            buffer[0] = [false; 123];

            // part 2
            if idx < (SEG_SIZE - 1) {
                for chr in line.chars() {
                    buffer[idx + 1][chr as usize] = true;
                }
            } else {
                for chr in line.chars() {
                    if buffer[1][chr as usize] & buffer[2][chr as usize] {
                        total_pt2 += prior(chr);
                        break;
                    }
                }
            }
        }
        buffer = [[false; MAX_ASCII]; SEG_SIZE];
    }
    (total_pt1, total_pt2)
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
        assert_eq!(solution(INPUT), (157, 70));
    }
}
