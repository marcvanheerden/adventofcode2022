use std::collections::HashMap;
use std::fs;
use std::thread;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let (solved, to_solve) = pre_process(&input);

    let mut part1 = 0isize;
    let mut part2 = 0isize;

    thread::scope(|s| {
        let thread1 = s.spawn(|| solution1(&solved, &to_solve));
        part2 = solution2(&solved, &to_solve);
        part1 = thread1.join().unwrap();
    });

    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

type Equation<'a> = (&'a str, Op, &'a str, &'a str);

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
}

fn pre_process(input: &str) -> (HashMap<&str, isize>, Vec<Equation>) {
    let parts: Vec<_> = input.lines().map(|l| l.split_once(':').unwrap()).collect();

    let solved: HashMap<&str, isize> = parts
        .iter()
        .filter(|(_, s2)| s2.len() < 6)
        .map(|(s1, s2)| (*s1, s2.trim().parse::<isize>().unwrap()))
        .collect();

    let to_solve: Vec<_> = parts
        .into_iter()
        .filter(|(_, s2)| s2.len() >= 6)
        .map(|(name, s2)| {
            let split2: Vec<&str> = s2.split_whitespace().collect();
            let op = match split2[1] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mult,
                "/" => Op::Div,
                _ => panic!(),
            };

            (name, op, split2[0], split2[2])
        })
        .collect();

    (solved, to_solve)
}

fn solution1(solved: &HashMap<&str, isize>, to_solve: &[Equation]) -> isize {
    let mut to_solve = to_solve.to_vec();
    let mut solved = solved.clone();

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

fn solution2(solved: &HashMap<&str, isize>, to_solve: &[Equation]) -> isize {
    let mut to_solve = to_solve.to_vec();
    let solved = solved.clone();

    // replace root monkey operation
    for equation in to_solve.iter_mut() {
        if equation.0 == "root" {
            equation.1 = Op::Eq;
        }
    }

    // use newton-raphson algorithm to solve equation
    let mut newt: Vec<(isize, isize)> = Vec::new();
    for guess in [1, 100] {
        newt.push((guess, calculate(guess, &to_solve, &solved)))
    }

    for idx in 1..10 {
        let dx = (newt[idx].0 - newt[idx - 1].0) as f64;
        let dy = (newt[idx].1 - newt[idx - 1].1) as f64;

        let next_x = (newt[idx].0 as f64 - (newt[idx].1 as f64 / (dy / dx))) as isize;

        newt.push((next_x, calculate(next_x, &to_solve, &solved)));
        if newt.last().unwrap().1 == 0 {
            break;
        }
    }

    newt.last().unwrap().0
}

fn calculate(cand: isize, to_solve: &[Equation], solved: &HashMap<&str, isize>) -> isize {
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
        let (solved, to_solve) = pre_process(INPUT);
        assert_eq!(solution1(&solved, &to_solve), 152);
    }

    #[test]
    fn example2() {
        let (solved, to_solve) = pre_process(INPUT);
        assert_eq!(solution2(&solved, &to_solve), 302);
    }
}
