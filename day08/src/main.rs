use rayon::prelude::*;
use std::cmp::max;
use std::fs;
use std::str::FromStr;
use std::thread;

// 14 ms parse, 140ms part1, 200ms part2
fn main() {
    //big_input();
    let input: String = fs::read_to_string("day08_input1024").unwrap();
    let treemap = TreeMap::from_str(&input).unwrap();
    let mut part1 = 0u32;
    let mut part2 = 0usize;
    thread::scope(|s| {
        let thread1 = s.spawn(|| solution1(&treemap));
        part2 = solution2(&treemap);
        part1 = thread1.join().unwrap();
    });
    println!("Part1: {:?} \nPart2: {:?}", part1, part2);
}

// orientation
#[derive(Debug)]
enum Orient {
    Orig,
    Cw90,
    Cw180,
    Cw270,
}

struct TreeMap {
    trees: Vec<Vec<u8>>,
    height: isize,
    width: isize,
}

impl FromStr for TreeMap {
    type Err = ();
    fn from_str(input: &str) -> Result<TreeMap, Self::Err> {
        let trees: Vec<Vec<u8>> = input
            .par_lines()
            .map(|l| l.chars().map(|c| c as u8 - 48).collect::<Vec<u8>>())
            .collect();

        let height = trees.len() as isize;
        let width = trees[0].len() as isize;
        Ok(TreeMap {
            trees,
            height,
            width,
        })
    }
}

impl TreeMap {
    fn get(&self, y: isize, x: isize, orient: &Orient) -> Option<u8> {
        let (y, x) = self.translate_coords(y, x, orient);
        if (x < 0) | (y < 0) | (x >= self.width) | (y >= self.height) {
            return None;
        }

        Some(self.trees[y as usize][x as usize])
    }

    fn get_unchecked(&self, y: isize, x: isize, orient: &Orient) -> u8 {
        let (y, x) = self.translate_coords(y, x, orient);
        self.trees[y as usize][x as usize]
    }

    fn translate_coords(&self, y: isize, x: isize, orient: &Orient) -> (isize, isize) {
        match orient {
            Orient::Orig => (y, x),
            Orient::Cw90 => (x, self.height - 1 - y),
            Orient::Cw180 => (self.height - 1 - y, self.width - 1 - x),
            Orient::Cw270 => (self.width - 1 - x, y),
        }
    }
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

fn solution1(treemap: &TreeMap) -> u32 {
    let mut vis_map = vec![vec![false; treemap.width as usize]; treemap.height as usize];

    let orientations = [Orient::Orig, Orient::Cw90, Orient::Cw180, Orient::Cw270];
    for orient in orientations {
        for y in 0..treemap.height {
            let mut max = 0u8;
            for x in 0..treemap.width {
                if (x == 0) | (y == 0) | (treemap.get_unchecked(y, x, &orient) > max) {
                    max = treemap.get_unchecked(y, x, &orient);
                    let (yi, xi) = treemap.translate_coords(y, x, &orient);
                    vis_map[yi as usize][xi as usize] = true;
                }
            }
        }
    }

    vis_map
        .iter()
        .map(|v| v.iter().filter(|x| **x).count())
        .sum::<usize>() as u32
}

fn solution2(treemap: &TreeMap) -> usize {
    let orientations = [Orient::Orig, Orient::Cw90, Orient::Cw180, Orient::Cw270];

    let columns: Vec<_> = (1..(treemap.height - 1)).collect();

    columns
        .par_iter()
        .map(|y| {
            let mut max_out = 0usize;
            for x in 1..(treemap.width - 1) {
                let mut output = 1usize;
                let current_height = treemap.get_unchecked(*y, x, &Orient::Orig);

                for orient in &orientations {
                    let mut steps = 0usize;
                    for idx in 1..max(treemap.width, treemap.height) {
                        let (yo, xo) = match orient {
                            Orient::Orig => (*y, x + idx),
                            Orient::Cw90 => (*y + idx, x),
                            Orient::Cw180 => (*y, x - idx),
                            Orient::Cw270 => (*y - idx, x),
                        };
                        if let Some(val) = treemap.get(yo, xo, &Orient::Orig) {
                            steps += 1;
                            if val >= current_height {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    output *= steps;
                }
                max_out = max(output, max_out);
            }
            max_out
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn treemaps() {
        let input = "12\n34";
        let treemap = TreeMap::from_str(input).unwrap();

        assert_eq!(treemap.translate_coords(0, 0, &Orient::Orig), (0, 0));
        assert_eq!(treemap.translate_coords(0, 1, &Orient::Cw90), (1, 1));
        assert_eq!(treemap.translate_coords(1, 0, &Orient::Cw180), (0, 1));
        assert_eq!(treemap.translate_coords(1, 1, &Orient::Cw270), (0, 1));
    }

    #[test]
    fn example() {
        let input = "30373
25512
65332
33549
35390";

        let treemap = TreeMap::from_str(&input).unwrap();
        let part1 = solution1(&treemap);
        assert_eq!(part1, 21);
        let part2 = solution2(&treemap);
        assert_eq!(part2, 8);
    }
}
