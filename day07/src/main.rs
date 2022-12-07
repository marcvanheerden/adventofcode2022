use rand::Rng;
use rayon::prelude::*;
use std::fs;
use std::str::FromStr;

fn main() {
    let input: String = fs::read_to_string("day07_input12000").unwrap();
    let (dir_sizes, usage) = par_get_sizes(&input);
    let part1 = solution1(&dir_sizes, 100000);
    let part2 = solution2(&dir_sizes, usage, 700000000, 300000000);
    println!("Part1: {:?} \nPart2: {:?}", part1, part2);

    //let mut commands = "$ cd /\n$ ls\n".to_owned();
    //big_input(&mut commands, 0);
    //fs::write("day07_input10000", &commands).unwrap();
    //println!("{} {}", solution(&commands), solution2(&commands));
    //println!("{}", commands);
}

enum CMD {
    DownDir,
    UpDir,
    Ls,
    File(u32),
    Dir,
}

impl FromStr for CMD {
    type Err = ();

    fn from_str(input: &str) -> Result<CMD, Self::Err> {
        let split: Vec<_> = input.split_whitespace().collect();

        match split[0] {
            "$" => match split.len() {
                3 => {
                    if split[2] == ".." {
                        Ok(CMD::UpDir)
                    } else {
                        Ok(CMD::DownDir)
                    }
                }
                _ => Ok(CMD::Ls),
            },
            "dir" => Ok(CMD::Dir),
            _ => Ok(CMD::File(split[0].parse::<u32>().unwrap())),
        }
    }
}

fn ran(start: u32, end: u32) -> u32 {
    rand::thread_rng().gen_range(start..=end)
}

fn big_input(comm: &mut String, depth: usize) {
    let dirs = if depth > 5 { 0 } else { ran(1, 20) };
    let dir_ls: String = (0..dirs)
        .map(|d| format!("dir {}\n", (d + 65) as u8 as char))
        .collect();
    let files: String = (0..ran(1, 3))
        .map(|_| format!("{} file{}\n", ran(1, 320), ran(65, 90) as u8 as char))
        .collect();

    comm.push_str(&dir_ls);
    comm.push_str(&files);

    for dir in 0..dirs {
        comm.push_str(&format!("$ cd {}\n$ ls\n", (dir + 65) as u8 as char));
        big_input(comm, depth + 1);
    }
    comm.push_str("$ cd ..\n");
}

fn par_get_sizes(commands: &str) -> (Vec<u32>, u32) {
    let comm: Vec<CMD> = commands
        .lines()
        .map(|s| CMD::from_str(s).unwrap())
        .filter(|c| match c {
            CMD::Ls | CMD::Dir => false,
            _ => true,
        })
        .collect();

    let indices: Vec<_> = (0..comm.len()).collect();

    let (dir_sizes, usage): (Vec<u32>, Vec<u32>) = indices
        .par_iter()
        .map(|idx| {
            let mut dir_size = 0u32;
            let mut usage = 0u32;

            match comm[*idx] {
                CMD::DownDir => {
                    let mut size = 0u32;
                    let mut depth = 0i8;
                    for command in comm.iter().skip(idx + 1) {
                        match command {
                            CMD::DownDir => {
                                depth += 1;
                            }
                            CMD::UpDir => {
                                depth -= 1;
                            }
                            CMD::File(size_) => {
                                size += size_;
                            }
                            _ => (),
                        }

                        if depth < 0 {
                            break;
                        }
                    }
                    dir_size = size;
                }
                CMD::File(size) => {
                    usage = size;
                }
                _ => (),
            }
            (dir_size, usage)
        })
        .unzip();

    (dir_sizes, usage.iter().sum())
}

fn solution1(sizes: &[u32], cutoff: u32) -> u32 {
    sizes.par_iter().filter(|i| i < &&cutoff).sum()
}

fn solution2(sizes: &[u32], space_used: u32, total_space: u32, space_needed: u32) -> u32 {
    let space_shortfall = total_space - space_needed;
    *sizes
        .par_iter()
        .filter(|i| i > &&(space_used - space_shortfall))
        .min()
        .unwrap()
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

        let (dir_sizes, usage) = par_get_sizes(input1);
        let part1 = solution1(&dir_sizes, 100000);
        let part2 = solution2(&dir_sizes, usage, 70000000, 30000000);
        assert_eq!(part1, 95437);
        assert_eq!(part2, 24933642);
    }
}
