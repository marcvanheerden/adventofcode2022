use std::cmp::max;
use std::fs;

fn main() {
    //big_input();
    let input: String = fs::read_to_string("day08_input1024").unwrap();
    let part1 = solution1(&input);
    let part2 = solution2(&input);
    println!("Part1: {:?} \nPart2: {:?}", part1, part2);
}

fn big_input() {
    let input: String = fs::read_to_string("input").unwrap();
    let mut dwide: String = input
        .lines()
        .map(|v| {
            let mut a = v.to_owned();
            for _ in 1..32 {
                a.push_str(v);
            }
            a.push('\n');
            a
        })
        .collect();

    let dwide2 = dwide.clone();

    for _ in 1..32 {
        dwide.push_str(&dwide2);
    }
    fs::write("day08_input1024", dwide).unwrap();
}

fn solution1(map: &str) -> u32 {
    let mut vmap: Vec<Vec<u8>> = Vec::new();

    for (idx, line) in map.lines().enumerate() {
        vmap.push(Vec::new());
        for tree in line.chars() {
            vmap[idx].push(tree as u8 - 48);
        }
    }

    let height = vmap.len();
    let width = vmap[0].len();

    let mut vis_map = vec![vec![false; width]; height];

    // l to r
    for y in 0..height {
        let mut max = 0u8;
        for x in 0..width {
            if (x == 0) | (y == 0) {
                max = vmap[y][x];
                vis_map[y][x] = true;
            } else if vmap[y][x] > max {
                vis_map[y][x] = true;
                max = vmap[y][x];
            }
        }
    }

    // r to l
    for y in 0..height {
        let mut max = 0u8;
        for x in 0..width {
            if (x == 0) | (y == 0) | (vmap[y][width - x - 1] > max) {
                vis_map[y][width - x - 1] = true;
                max = vmap[y][width - x - 1];
            }
        }
    }

    // t to b
    for x in 0..width {
        let mut max = 0u8;
        for y in 0..height {
            if (x == 0) | (y == 0) | (vmap[y][x] > max) {
                vis_map[y][x] = true;
                max = vmap[y][x];
            }
        }
    }

    // b to t
    for x in 0..width {
        let mut max = 0u8;
        for y in 0..height {
            if (x == 0) | (y == 0) | (vmap[height - y - 1][x] > max) {
                vis_map[height - y - 1][x] = true;
                max = vmap[height - y - 1][x];
            }
        }
    }

    let mut output = 0u32;
    for row in vis_map {
        for val in row {
            if val {
                output += 1
            }
        }
    }

    output
}

fn solution2(map: &str) -> usize {
    let mut vmap: Vec<Vec<u8>> = Vec::new();

    for (idx, line) in map.lines().enumerate() {
        vmap.push(Vec::new());
        for tree in line.chars() {
            vmap[idx].push(tree as u8 - 48);
        }
    }

    let height = vmap.len();
    let width = vmap[0].len();

    let mut bigmax = 0usize;
    for y in 1..height {
        for x in 1..width {
            let (mut up, mut down, mut left, mut right) = (0, 0, 0, 0);

            for yi in 1..=y {
                up = yi;
                if vmap[y - yi][x] >= vmap[y][x] {
                    break;
                }
            }

            for yi in (y + 1)..height {
                down = yi - y;
                if vmap[yi][x] >= vmap[y][x] {
                    break;
                }
            }

            for xi in 1..=x {
                left = xi;
                if vmap[y][x - xi] >= vmap[y][x] {
                    break;
                }
            }

            for xi in (x + 1)..width {
                right = xi - x;
                if vmap[y][xi] >= vmap[y][x] {
                    break;
                }
            }

            bigmax = max(up * down * left * right, bigmax);
        }
    }

    bigmax
}

fn viz(map: &[Vec<bool>]) {
    let printout: Vec<String> = map
        .iter()
        .map(|v| {
            v.iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        })
        .collect();

    println!();
    for line in printout.iter() {
        println!("{}", line);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "30373
25512
65332
33549
35390";

        let part1 = solution1(&input);
        assert_eq!(part1, 21);
        let part2 = solution2(&input);
        assert_eq!(part2, 8);
    }
}
