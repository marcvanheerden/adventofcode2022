use std::collections::VecDeque;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();

    let numbers: Vec<(usize, isize)> = input
        .lines()
        .enumerate()
        .map(|(idx, s)| (idx, s.parse::<isize>().unwrap()))
        .collect();

    let part1 = solution(&numbers, 1, 1);
    let part2 = solution(&numbers, 811589153, 10);
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

fn solution(numbers: &[(usize, isize)], mult: isize, rep: usize) -> isize {
    let mut numbers: VecDeque<_> = numbers
        .iter()
        .cloned()
        .map(|(idx, val)| (idx, val * mult))
        .collect();

    for _ in 0..rep {
        for idx in 0..numbers.len() {
            while numbers[0].0 != idx {
                numbers.rotate_left(1);
            }

            let to_move = numbers.pop_front().unwrap();
            if to_move.1 < 0 {
                numbers.rotate_right(to_move.1.unsigned_abs() % numbers.len());
            } else {
                numbers.rotate_left(to_move.1.unsigned_abs() % numbers.len());
            }
            numbers.push_front(to_move);
        }
    }

    while numbers[0].1 != 0 {
        numbers.rotate_left(1);
    }

    numbers[1000 % numbers.len()].1
        + numbers[2000 % numbers.len()].1
        + numbers[3000 % numbers.len()].1
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn example() {
        let numbers: Vec<(usize, isize)> = INPUT
            .lines()
            .enumerate()
            .map(|(idx, s)| (idx, s.parse::<isize>().unwrap()))
            .collect();

        assert_eq!(solution(&numbers, 1, 1), 3);
    }
    #[test]
    fn example2() {
        let numbers: Vec<(usize, isize)> = INPUT
            .lines()
            .enumerate()
            .map(|(idx, s)| (idx, s.parse::<isize>().unwrap()))
            .collect();

        assert_eq!(solution(&numbers, 811589153, 10), 1623178306);
    }
}
