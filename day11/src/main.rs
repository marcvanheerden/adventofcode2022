use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;

const NPRIMES: usize = 9;
const PRIMES: [usize; NPRIMES] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let part2 = solution(&input, 10000);
    println!("Part2: {}", part2);
}

fn solution(input: &str, steps: usize) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| Monkey::from_str(s).unwrap())
        .collect();

    let length = monkeys.len();

    for _ in 0..steps {
        for idx in 0..monkeys.len() {
            let items = monkeys[idx].take_turn();

            for item in items.into_iter() {
                monkeys[item.dest].push(item.val);
            }
        }
    }

    let mut counters: Vec<_> = monkeys.iter().map(|m| m.counter).collect();
    counters.sort_unstable();
    counters[length - 1] * counters[length - 2]
}

#[derive(Clone, Copy)]
struct BigInt {
    components: [usize; NPRIMES],
}

impl BigInt {
    fn new(val: usize) -> BigInt {
        let components: [usize; NPRIMES] = PRIMES.map(|p| val % p);
        BigInt { components }
    }

    fn add(&self, other: &BigInt) -> BigInt {
        let mut components = self.components;
        for idx in 0..NPRIMES {
            components[idx] = (self.components[idx] + other.components[idx]) % PRIMES[idx];
        }

        BigInt { components }
    }

    fn mult(&self, other: &BigInt) -> BigInt {
        let mut components = self.components;
        for idx in 0..NPRIMES {
            components[idx] = (self.components[idx] * other.components[idx]) % PRIMES[idx];
        }

        BigInt { components }
    }

    fn is_div(&self, candidate: usize) -> bool {
        for (idx, prime) in PRIMES.into_iter().enumerate() {
            if candidate == prime {
                return self.components[idx] == 0;
            }
        }
        false
    }
}

#[derive(Clone, Copy)]
enum Token {
    Old,
    Literal(BigInt),
}

impl FromStr for Token {
    type Err = ();
    fn from_str(input: &str) -> Result<Token, Self::Err> {
        if input == "old" {
            return Ok(Token::Old);
        }

        let val = input.parse::<usize>().unwrap();
        Ok(Token::Literal(BigInt::new(val)))
    }
}

impl Token {
    fn get(&self, old_val: BigInt) -> BigInt {
        match self {
            Self::Old => old_val,
            Self::Literal(val) => *val,
        }
    }
}

#[derive(Clone, Copy)]
enum Infix {
    Add(Token, Token),
    Mult(Token, Token),
}

impl FromStr for Infix {
    type Err = ();
    fn from_str(input: &str) -> Result<Infix, Self::Err> {
        let rhs: Vec<_> = input
            .split_once(" = ")
            .unwrap()
            .1
            .split_whitespace()
            .collect();

        let left = Token::from_str(rhs[0]).unwrap();
        let right = Token::from_str(rhs[2]).unwrap();

        match rhs[1] {
            "+" => Ok(Infix::Add(left, right)),
            "*" => Ok(Infix::Mult(left, right)),
            _ => panic!(),
        }
    }
}

struct Throw {
    val: BigInt,
    dest: usize,
}

struct Monkey {
    items: VecDeque<BigInt>,
    test: usize,
    infix: Infix,
    if_true: usize,
    if_false: usize,
    counter: usize,
}

impl FromStr for Monkey {
    type Err = ();
    fn from_str(input: &str) -> Result<Monkey, Self::Err> {
        let mut lines = input.lines();

        lines.next();

        // items
        let items_lines = lines.next().unwrap();
        let items_str = items_lines.split_once(": ").unwrap().1;
        let items: VecDeque<BigInt> = items_str
            .split(", ")
            .map(|s| s.trim().parse().unwrap())
            .map(BigInt::new)
            .collect();

        // operation
        let infix = Infix::from_str(lines.next().unwrap()).unwrap();

        // test
        let test = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        // if true
        let if_true = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        // if false
        let if_false = lines
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Ok(Monkey {
            items,
            test,
            infix,
            if_true,
            if_false,
            counter: 0,
        })
    }
}

impl Monkey {
    fn take_turn(&mut self) -> Vec<Throw> {
        let mut output = Vec::new();

        while !self.items.is_empty() {
            self.counter += 1;
            let mut worry = self.items.pop_front().unwrap();

            worry = match self.infix {
                Infix::Add(x, y) => x.get(worry).add(&y.get(worry)),
                Infix::Mult(x, y) => x.get(worry).mult(&y.get(worry)),
            };

            let dest = if worry.is_div(self.test) {
                self.if_true
            } else {
                self.if_false
            };

            output.push(Throw { val: worry, dest });
        }
        output
    }

    fn push(&mut self, new_val: BigInt) {
        self.items.push_back(new_val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn example() {
        assert_eq!(solution(INPUT, 10000), 2713310158);
    }
}
