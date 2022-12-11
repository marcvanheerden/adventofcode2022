use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let part1 = solution(&input, 20, 3);
    let part2 = solution(&input, 100000, 3);
    println!("Part1: {}\nPart2: {}", part1, part2);
}

fn solution(input: &str, steps: usize, deflator: usize) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| Monkey::from_str(s).unwrap())
        .collect();

    let length = monkeys.len();

    for step in 0..steps {
        for idx in 0..monkeys.len() {
            let items = monkeys[idx].take_turn(deflator);
            
            for item in items.into_iter() {
                monkeys[item.dest].push(item.val);
            }
        }
    }

    let mut counters: Vec<_> = monkeys.iter().map(|m| m.counter).collect();
    dbg!(&counters);
    counters.sort_unstable();
    counters[length - 1] * counters[length - 2]
}

struct BigInt {
    components: Vec<Vec<usize>>
}

impl BigInt {
    fn new(val: usize) -> BigInt {

        let mut val = val;
        let mut pfactors = Vec::new();
        
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41,
                      43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

        while val > 1 {
            for prime in primes {
                if (val % prime) == 0 {
                    val /= prime;
                    pfactors.push(prime);
                    break
                }
            }
        }

        BigInt{
            components: vec![pfactors]
        }
    }

    fn add(&mut self, other: BigInt) {
        self.components.append(&mut other.components.clone());
    }

    fn mult(&mut self, other: &mut BigInt) {
        let mut new = Vec::new();

        for comp1 in self.components.clone().iter() {
            for comp2 in other.components.clone().iter() {
                let mut new_val = comp1.clone();
                for val in comp2.iter() {
                    new_val.push(*val);
                }
                new.push(new_val);
            }
        }
        
        self.components = new;
    }

    fn is_div(&self, candidate: usize) -> bool {
        let total_rem: usize = self.components.iter().map(|v| {
            v.iter().fold(1, |acc, x| acc * x) % candidate
        }).sum();

        if (total_rem % candidate) == 0 {
            return true
        }
        false
    }
}


#[derive(Clone, Copy)]
enum Token {
    Old,
    Literal(usize),
}

impl FromStr for Token {
    type Err = ();
    fn from_str(input: &str) -> Result<Token, Self::Err> {
        if input == "old" {
            return Ok(Token::Old);
        }

        let val = input.parse::<usize>().unwrap();
        Ok(Token::Literal(val))
    }
}

impl Token {
    fn get(&self, old_val: usize) -> usize {
        match self {
            Self::Old => old_val,
            Self::Literal(val) => *val
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
    val: usize,
    dest: usize,
}

struct Monkey {
    items: VecDeque<usize>,
    test: usize,
    infix: Infix,
    if_true: usize,
    if_false: usize,
    counter: usize
}

impl FromStr for Monkey {
    type Err = ();
    fn from_str(input: &str) -> Result<Monkey, Self::Err> {
        let mut lines = input.lines();

        lines.next();

        // items
        let items_lines = lines.next().unwrap();
        let items_str = items_lines.split_once(": ").unwrap().1;
        let items: VecDeque<usize> = items_str
            .split(", ")
            .map(|s| s.trim().parse().unwrap())
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
            counter: 0
        })
    }
}

impl Monkey {
    fn take_turn(&mut self, deflator: usize) -> Vec<Throw> {
        let mut output = Vec::new();

        while !self.items.is_empty() {
            self.counter += 1;
            let mut worry = self.items.pop_front().unwrap();
        
            worry = match self.infix {
                Infix::Add(x, y) => {
                    x.get(worry) + y.get(worry)
                },
                Infix::Mult(x, y) => {
                    x.get(worry) * y.get(worry)
                }
            } / deflator;

            let dest = if (worry % self.test) == 0 {
                self.if_true
            } else {
                self.if_false
            };

            output.push(Throw { val: worry, dest });
        }
        output
    }

    fn push(&mut self, new_val: usize) {
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
        assert_eq!(solution(INPUT, 20, 3), 10605);
       // assert_eq!(solution(INPUT, 10000, 1), 2713310158);
    }

    #[test]
    fn bigint() {
        let mut b1 = BigInt::new(10);
        let b2 = BigInt::new(8);
        assert_eq!(b1.components, vec![vec![2, 5]]);
        assert_eq!(b2.components, vec![vec![2, 2, 2]]);
        b1.add(b2);
        assert_eq!(b1.components, vec![vec![2, 5], vec![2, 2, 2]]);
        let mut b3 = BigInt::new(8);
        b1.mult(&mut b3);
        assert_eq!(b1.components,
                   vec![vec![2, 5, 2, 2, 2], vec![2, 2, 2, 2, 2, 2]]);

        assert!(b1.is_div(12));
        assert!(!b1.is_div(13));
    }
}
