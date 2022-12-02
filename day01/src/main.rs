use rayon::prelude::*;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input1000").unwrap();

    // split and cast to integers in parallel
    let elf_totals: Vec<u32> = input
        .replace("\n\n", "~")
        .par_split('~')
        .map(|s| s.lines().map(|x| x.parse::<u32>().unwrap()).sum())
        .collect();

    let answer = solution(&elf_totals, 3);
    println!("Part1 : {} \nPart2 : {}", answer.0, answer.1);
}

fn solution(cals: &[u32], top_n: usize) -> (u32, u32) {
    let mut top = BinaryHeap::new();

    for val in cals {
        if top.len() < top_n {
            top.push(Reverse(val));
        } else if top.peek().unwrap() > &Reverse(val) {
            top.pop();
            top.push(Reverse(val));
        }
    }

    let Reverse(part1) = top.iter().min().unwrap();

    let part2 = top
        .iter()
        .map(|u| {
            let Reverse(v) = *u;
            v
        })
        .sum();

    (**part1, part2)
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
        let elf_totals: Vec<u32> = INPUT
            .split("\n\n")
            .map(|s| s.lines().map(|x| x.parse::<u32>().unwrap()).sum())
            .collect();

        assert_eq!(solution(&elf_totals, 3), (24000, 45000));
    }
}
