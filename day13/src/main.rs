use rayon::prelude::*;
use std::cmp::min;
use std::cmp::Ordering;
use std::fs;
use std::sync::Arc;
use std::str::FromStr;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let part1 = solution1(&input);
    let part2 = solution2(&input);
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

#[derive(Debug, Eq)]
struct NumList {
    // can have sublists or a value or neither, not both
    sublists: Arc<Vec<NumList>>,
    val: Option<usize>,
}

impl FromStr for NumList {
    type Err = ();

    fn from_str(input: &str) -> Result<NumList, Self::Err> {
        // empty list
        if input.is_empty() {
            return Ok(NumList {
                sublists: Arc::new(Vec::with_capacity(1)),
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
                sublists: Arc::new(sublists),
                val: None,
            });
        }

        // number literal
        let sublists = Arc::new(Vec::with_capacity(1));
        let val: Option<usize> = Some(input.parse::<usize>().unwrap());
        Ok(NumList { sublists, val })
    }
}

impl Ord for NumList {
    fn cmp(&self, other: &NumList) -> Ordering {
        if (!self.sublists.is_empty() & self.val.is_some())
            | (!other.sublists.is_empty() & other.val.is_some())
        {
            panic!("Broken NumList format")
        }

        // both are literals
        if self.val.is_some() & other.val.is_some() {
            return self.val.unwrap().cmp(&other.val.unwrap());
        }

        // both are lists
        if self.val.is_none() & other.val.is_none() {
            for idx in 0..min(other.sublists.len(), self.sublists.len()) {
                match self.sublists[idx].cmp(&other.sublists[idx]) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => (),
                }
            }

            return self.sublists.len().cmp(&other.sublists.len());
        }

        // left is literal, right is list (possibly empty)
        if self.val.is_some() & other.val.is_none() {
            let self_clone = NumList {
                sublists: Arc::new(Vec::with_capacity(1)),
                val: self.val,
            };
            let clone_wrap = NumList {
                sublists: Arc::new(vec![self_clone]),
                val: None,
            };

            return clone_wrap.cmp(other);
        }

        // left is list (possibly empty), right is literal
        if self.val.is_none() & other.val.is_some() {
            let other_clone = NumList {
                sublists: Arc::new(Vec::with_capacity(1)),
                val: other.val,
            };
            let clone_wrap = NumList {
                sublists: Arc::new(vec![other_clone]),
                val: None,
            };

            return self.cmp(&clone_wrap);
        }
        unreachable!();
    }
}

impl PartialOrd for NumList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NumList {
    fn eq(&self, other: &Self) -> bool {
        (self.val) == (other.val)
    }
}

fn solution1(input: &str) -> usize {
    let sections: Vec<_> = input.split("\n\n").collect();

    sections
        .par_iter()
        .enumerate()
        .map(|(idx, s)| {
            let (left, right) = s.split_once('\n').unwrap();
            let left = NumList::from_str(left).unwrap();
            let right = NumList::from_str(right).unwrap();

            match &left.cmp(&right) {
                Ordering::Less | Ordering::Equal => idx + 1,
                Ordering::Greater => 0,
            }
        })
        .sum()
}

fn solution2(input: &str) -> usize {
    let mut input = input.to_owned();
    input.push_str("\n[[2]]");
    input.push_str("\n[[6]]");

    let mut packets: Vec<NumList> = input
        .par_lines()
        .filter(|l| !l.is_empty())
        .map(|l| NumList::from_str(l).unwrap())
        .collect();

    packets.sort_unstable();

    // find dividers
    packets
        .into_iter()
        .enumerate()
        .map(|(idx, nl)| {
            if (nl.sublists.len() == 1) & (nl.sublists[0].sublists.len() == 1) {
                if [Some(2), Some(6)].contains(&nl.sublists[0].sublists[0].val) {
                    return idx + 1;
                }
            }
            1
        })
        .product()
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
        assert_eq!(a.cmp(&b), Ordering::Greater); // wrong order

        let a = NumList::from_str("[[1],[2,3,4]]").unwrap();
        let b = NumList::from_str("[[1],4]").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less); // right order

        let a = NumList::from_str("[9]").unwrap();
        let b = NumList::from_str("[[8,7,6]]").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Greater); // wrong order

        let a = NumList::from_str("[[4,4],4,4]").unwrap();
        let b = NumList::from_str("[[4,4],4,4,4]").unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less); // right order

        assert_eq!(solution1(INPUT), 13);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 140);
    }
}
