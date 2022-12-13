use std::cmp::min;
use std::fs;
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let part1 = solution(&input);
    println!("Part1: {:?}\nPart2: {:?}", part1, part1);
}

#[derive(Debug, Eq, PartialEq)]
enum Order {
    Less,
    More,
    Equal
}

#[derive(Debug)]
struct NumList {
    // can have sublists or a value or neither, not both
    // literals handled as list of len 1
    sublists: Rc<Vec<NumList>>,
    val: Option<usize>,
}

impl FromStr for NumList {
    type Err = ();

    fn from_str(input: &str) -> Result<NumList, Self::Err> {
        // empty list
        if input.is_empty() {
            return Ok(NumList {
                sublists: Rc::new(Vec::with_capacity(1)),
                val: None,
            });
        }

        // start of a list
        if input.starts_with('[') {
            let mut depth = 0u8;
            let mut sublist_str: Vec<String> = vec!["".to_owned()];
            let mut sublist_idx = 0usize;

            // build strings of sublists
            for chr in input.chars().skip(1) {
                match chr {
                    '[' => depth += 1,
                    ']' => {
                        if depth == 0 {
                            break;
                        } else {
                            depth -= 1
                        }
                    }
                    _ => (),
                }

                if (depth == 0) & (chr == ',') {
                    sublist_str.push("".to_owned());
                    sublist_idx = sublist_str.len() - 1;
                } else {
                    sublist_str[sublist_idx].push(chr);
                }
            }

            // recursively parse the strings of sublists
            let sublists: Vec<NumList> = sublist_str
                .into_iter()
                .map(|s| NumList::from_str(&s).unwrap())
                .collect();

            return Ok(NumList {
                sublists: Rc::new(sublists),
                val: None,
            });
        }

        // number literal
        let sublists = Rc::new(Vec::with_capacity(1));
        let val: Option<usize> = Some(input.parse::<usize>().unwrap());
        Ok(NumList { sublists, val })
    }
}

impl NumList {
    fn comp(&self, other: &NumList) -> Order {
        //dbg!(self);
        //dbg!(other);
        // both list and literal -> fail
        if (!self.sublists.is_empty() & self.val.is_some())
            | (!other.sublists.is_empty() & other.val.is_some())
        {
            panic!("Broken NumList format")
        }

        // both are literals
        if self.val.is_some() & other.val.is_some() {
            return if self.val.unwrap() > other.val.unwrap() {
                Order::More
            } else if self.val.unwrap() == other.val.unwrap() {
                Order::Equal
            } else {
                Order::Less
            }
        }

        // both are lists
        if !self.val.is_some() & !other.val.is_some() {
            //dbg!("lists");
            for idx in 0..min(other.sublists.len(), self.sublists.len()) {
                let x = self.sublists[idx].comp(&other.sublists[idx]);
                match x {
                    Order::More | Order::Less => return x,
                    Order::Equal => ()
                }
            }

            if self.sublists.len() < other.sublists.len() {
                return Order::Less
            } else if self.sublists.len() > other.sublists.len() {
                return Order::More
            } else {
                return Order::Equal
            }
        }

        // left is literal, right is list (possibly empty)
        if self.val.is_some() & !other.val.is_some() {
            let self_clone = NumList {
                sublists: Rc::new(Vec::with_capacity(1)),
                val: self.val,
            };
            let clone_wrap = NumList {
                sublists: Rc::new(vec![self_clone]),
                val: None,
            };

            return clone_wrap.comp(other);
        }

        // left is list (possibly empty), right is literal
        if !self.val.is_some() & other.val.is_some() {
            let other_clone = NumList {
                sublists: Rc::new(Vec::with_capacity(1)),
                val: other.val,
            };
            let clone_wrap = NumList {
                sublists: Rc::new(vec![other_clone]),
                val: None,
            };

            return self.comp(&clone_wrap);
        }
        unreachable!();
    }
}

fn solution(input: &str) -> usize {
    let packets: Vec<_> = input.split("\n\n").collect();

    packets
        .into_iter()
        .enumerate()
        .map(|(idx, s)| {
            dbg!("ssssssssssssxxxxxxxxxxxxxx");
            let (left, right) = s.split_once('\n').unwrap();

            let left = NumList::from_str(left).unwrap();
            let right = NumList::from_str(right).unwrap();

            let x = &left.comp(&right);
            //dbg!(x);
            
            if [Order::Less, Order::Equal].contains(x) {
                idx + 1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn example() {
        let a = NumList::from_str("[1,3,2]").unwrap();
        let b = NumList::from_str("[1,2,3]").unwrap();
        assert_eq!(a.comp(&b), Order::More); // wrong order

        let a = NumList::from_str("[[1],[2,3,4]]").unwrap();
        let b = NumList::from_str("[[1],4]").unwrap();
        assert_eq!(a.comp(&b), Order::Less); // right order

        let a = NumList::from_str("[9]").unwrap();
        let b = NumList::from_str("[[8,7,6]]").unwrap();
        assert_eq!(a.comp(&b), Order::More); // wrong order

        let a = NumList::from_str("[[4,4],4,4]").unwrap();
        let b = NumList::from_str("[[4,4],4,4,4]").unwrap();
        assert_eq!(a.comp(&b), Order::Less); // right order

        assert_eq!(solution(INPUT), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 140);
    }
}
