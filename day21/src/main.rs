use std::collections::HashMap;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let part1 = solution1(&input);
    let part2 = solution2(&input);
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
}

fn solution1(input: &str) -> isize {
    let mut solved: HashMap<&str, isize> = HashMap::new();
    let mut to_solve: Vec<(&str, Op, &str, &str)> = Vec::new();

    for line in input.lines() {
        let split = line.split_once(':').unwrap();

        let name = split.0;

        if split.1.len() < 6 {
            solved.insert(name, split.1.trim().parse::<isize>().unwrap());
        } else {
            let split2: Vec<&str> = split.1.split_whitespace().collect();
            let op = match split2[1] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mult,
                "/" => Op::Div,
                _ => panic!(),
            };

            to_solve.push((name, op, split2[0], split2[2]));
        }
    }

    while !solved.contains_key("root") {
        for idx in 0..to_solve.len() {
            if solved.contains_key(to_solve[idx].2) & solved.contains_key(to_solve[idx].3) {
                let left = solved.get(to_solve[idx].2).unwrap();
                let right = solved.get(to_solve[idx].3).unwrap();
                let answer = match to_solve[idx].1 {
                    Op::Add => left + right,
                    Op::Sub => left - right,
                    Op::Mult => left * right,
                    Op::Div => left / right,
                    _ => panic!(),
                };

                solved.insert(to_solve[idx].0, answer);
                to_solve.remove(idx);
                break;
            }
        }
    }

    *solved.get("root").unwrap()
}

fn solution2(input: &str) -> isize {
    let mut solved: HashMap<&str, isize> = HashMap::new();
    let mut to_solve: Vec<(&str, Op, &str, &str)> = Vec::new();

    for line in input.lines() {
        let split = line.split_once(':').unwrap();

        let name = split.0;

        if split.1.len() < 6 {
            solved.insert(name, split.1.trim().parse::<isize>().unwrap());
        } else {
            let split2: Vec<&str> = split.1.split_whitespace().collect();
            let mut op = match split2[1] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mult,
                "/" => Op::Div,
                _ => panic!(),
            };

            if name == "root" {
                op = Op::Eq;
            }

            to_solve.push((name, op, split2[0], split2[2]));
        }
    }

    let mut newt: Vec<(isize, isize)> = Vec::new();

    // use newton-raphson algorithm to solve equation
    for guess in [1, 100] {
        newt.push((guess, test(guess, &to_solve, &solved)))
    }

    for idx in 1..10 {
        let dx = (newt[idx].0 - newt[idx - 1].0) as f64;
        let dy = (newt[idx].1 - newt[idx - 1].1) as f64;

        let next_x = (newt[idx].0 as f64 - (newt[idx].1 as f64 / (dy / dx))) as isize;

        newt.push((next_x, test(next_x, &to_solve, &solved)));
        if newt.last().unwrap().1 == 0 {
            break;
        }
    }

    newt.last().unwrap().0
}

fn test(cand: isize, to_solve: &[(&str, Op, &str, &str)], solved: &HashMap<&str, isize>) -> isize {
    let mut to_solve: Vec<_> = to_solve.to_vec();
    let mut solved = solved.clone();

    let human = solved.get_mut("humn").unwrap();
    *human = cand;
    while !solved.contains_key("root") {
        for idx in 0..to_solve.len() {
            if solved.contains_key(to_solve[idx].2) & solved.contains_key(to_solve[idx].3) {
                let left = solved.get(to_solve[idx].2).unwrap();
                let right = solved.get(to_solve[idx].3).unwrap();
                let answer = match to_solve[idx].1 {
                    Op::Add => left + right,
                    Op::Sub => left - right,
                    Op::Mult => left * right,
                    Op::Div => left / right,
                    Op::Eq => return left - right,
                };

                solved.insert(to_solve[idx].0, answer);
                to_solve.remove(idx);
                break;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn example() {
        assert_eq!(solution1(INPUT), 152);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 302);
    }
}
