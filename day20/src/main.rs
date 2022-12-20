use std::collections::VecDeque;
use std::fs;
use std::thread;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();

    let numbers: Vec<isize> = input.lines().map(|s| s.parse::<isize>().unwrap()).collect();

    let mut part1 = 0isize;
    let mut part2 = 0isize;

    thread::scope(|s| {
        let thread1 = s.spawn(|| solution(&numbers, 1, 1));
        part2 = solution(&numbers, 811589153, 10);
        part1 = thread1.join().unwrap();
    });

    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

fn solution(numbers: &[isize], mult: isize, rep: usize) -> isize {
    let len_minus1 = numbers.len() - 1;

    let numbers: Vec<(isize, bool, usize)> = numbers
        .iter()
        .cloned()
        .map(|val| val * mult)
        .map(|val| (val, val > 0, val.unsigned_abs() % len_minus1))
        .collect();

    let mut indices: VecDeque<_> = (0..=len_minus1).collect();

    for _ in 0..rep {
        for idx in 0..=len_minus1 {
            // rotate until the current entry is first in the VecDeque
            let pos = indices.iter().position(|idx1| *idx1 == idx).unwrap();
            indices.rotate_left(pos);

            let to_move = indices.pop_front().unwrap();
            let metadata = numbers[to_move];

            if metadata.1 {
                indices.rotate_left(metadata.2);
            } else {
                indices.rotate_right(metadata.2);
            }
            indices.push_front(to_move);
        }
    }

    let pos = indices.iter().position(|idx| numbers[*idx].0 == 0).unwrap();
    indices.rotate_left(pos);

    numbers[indices[1000 % indices.len()]].0
        + numbers[indices[2000 % indices.len()]].0
        + numbers[indices[3000 % indices.len()]].0
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
        let numbers: Vec<isize> = INPUT.lines().map(|s| s.parse::<isize>().unwrap()).collect();

        assert_eq!(solution(&numbers, 1, 1), 3);
    }
    #[test]
    fn example2() {
        let numbers: Vec<isize> = INPUT.lines().map(|s| s.parse::<isize>().unwrap()).collect();

        assert_eq!(solution(&numbers, 811589153, 10), 1623178306);
    }
}
