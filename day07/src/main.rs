use std::fs;
//use std::rc::Rc;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let part1 = solution(&input);
    let part2 = solution2(&input);
    println!("Part1: {:?} \nPart2: {:?}", part1, part2);
}

fn solution(commands: &str) -> u32 {
    let comm: Vec<Vec<&str>> = commands
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect();

    let mut sizes = Vec::new();

    for idx1 in 0..comm.len() {
        if comm[idx1][1] == "cd" {
            if comm[idx1][2] != ".." {
                let mut size = 0u32;
                let mut depth = 0i32;
                for idx2 in (idx1 + 1)..comm.len() {
                    if (comm[idx2][1] == "cd") {
                        if (comm[idx2][2] == "..") {
                            depth -= 1;
                        } else {
                            depth += 1;
                        }
                    } else if !["$", "dir"].contains(&comm[idx2][0]) {
                        size += comm[idx2][0].parse::<u32>().unwrap();
                    }

                    if depth < 0 {
                        break;
                    }
                }
                sizes.push(size);
            }
        }
    }
    sizes.iter().filter(|i| i < &&100000).sum()
}

fn solution2(commands: &str) -> u32 {
    let comm: Vec<Vec<&str>> = commands
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect();

    let mut sizes = Vec::new();

    for idx1 in 0..comm.len() {
        if (comm[idx1][1] == "cd") {
            if (comm[idx1][2] != "..") {
                let mut size = 0u32;
                let mut depth = 0i32;
                for idx2 in (idx1 + 1)..comm.len() {
                    if (comm[idx2][1] == "cd") {
                        if (comm[idx2][2] == "..") {
                            depth -= 1;
                        } else {
                            depth += 1;
                        }
                    } else if !["$", "dir"].contains(&comm[idx2][0]) {
                        size += comm[idx2][0].parse::<u32>().unwrap();
                    }

                    if depth < 0 {
                        break;
                    }
                }
                sizes.push(size);
            }
        }
    }
    let space_needed = 70000000 - 30000000;
    let space_used: u32 = comm.iter().map(|v| { 
        if !["$", "dir"].contains(&v[0]) {
            v[0].parse::<u32>().unwrap()
        } else {
            0u32
        }
    }).sum();
    *sizes.iter().filter(|i| i > &&(space_used - space_needed)).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input1 = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        assert_eq!(solution(input1), 95437);
        assert_eq!(solution2(input1), 24933642);
    }
}
