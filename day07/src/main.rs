use rand::Rng;
use rayon::prelude::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("day07_input12000").unwrap();
    let (dir_sizes, usage) = get_sizes(&input);
    let part1 = solution1(&dir_sizes, 100000);
    let part2 = solution2(&dir_sizes, usage, 700000000, 300000000);
    println!("Part1: {:?} \nPart2: {:?}", part1, part2);

    //let mut commands = "$ cd /\n$ ls\n".to_owned();
    //big_input(&mut commands, 0);
    //fs::write("day07_input10000", &commands).unwrap();
    //println!("{} {}", solution(&commands), solution2(&commands));
    //println!("{}", commands);
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

fn solution1(sizes: &[u32], cutoff: u32) -> u32 {
    sizes.iter().filter(|i| i < &&cutoff).sum()
}

fn get_sizes(commands: &str) -> (Vec<u32>, u32) {
    let comm: Vec<Vec<&str>> = commands
        .lines()
        .map(|s| s.split_whitespace().collect())
        .collect();

    let mut dir_sizes = Vec::new();
    let mut usage = 0u32;

    for idx1 in 0..comm.len() {
        if comm[idx1][1] == "cd" {
            if comm[idx1][2] != ".." {
                let mut size = 0u32;
                let mut depth = 0i32;
                for command in comm.iter().skip(idx1 + 1) {
                    if command[1] == "cd" {
                        if command[2] == ".." {
                            depth -= 1;
                        } else {
                            depth += 1;
                        }
                    } else if !["$", "dir"].contains(&command[0]) {
                        size += command[0].parse::<u32>().unwrap();
                    }

                    if depth < 0 {
                        break;
                    }
                }
                dir_sizes.push(size);
            }
        } else if let Ok(filesize) = comm[idx1][0].parse::<u32>() {
            usage += filesize;
        }
    }

    (dir_sizes, usage)
}

fn solution2(sizes: &[u32], space_used: u32, total_space: u32, space_needed: u32) -> u32 {
    let space_shortfall = total_space - space_needed;
    *sizes
        .iter()
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

        let (dir_sizes, usage) = get_sizes(input1);
        let part1 = solution1(&dir_sizes, 100000);
        let part2 = solution2(&dir_sizes, usage, 70000000, 30000000);
        assert_eq!(part1, 95437);
        assert_eq!(part2, 24933642);
    }
}
