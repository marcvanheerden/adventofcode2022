use std::fs;
use std::collections::HashMap;
use std::cmp::{min, max};
use rayon::prelude::*;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let part1 = solution1(&input, 2000_000);
    let part2 = solution2(&input, 4000000, 4000000);
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

fn get_points(input: &str) -> (HashMap<(isize, isize), (isize, isize, isize)>, (isize, isize, isize, isize)) {
    let mut points = HashMap::new();
    let mut minx = isize::MAX;
    let mut miny = isize::MAX;
    let mut maxx = isize::MIN;
    let mut maxy = isize::MIN;

    for line in input.lines() {
        let splitws: Vec<_> = line.split_whitespace().collect();
        
        let mut sxstr: Vec<&str> = splitws[2].split('=').collect();
        let mut systr: Vec<&str> = splitws[3].split('=').collect();
        let mut bxstr: Vec<&str> = splitws[8].split('=').collect();
        let bystr: Vec<&str> = splitws[9].split('=').collect();

        sxstr = sxstr[1].split(',').collect();
        systr = systr[1].split(':').collect();
        bxstr = bxstr[1].split(',').collect();
        
        let sx = sxstr[0].parse().unwrap();
        let sy = systr[0].parse().unwrap();
        let bx = bxstr[0].parse().unwrap();
        let by = bystr[1].parse().unwrap();
        
        let dist = manh_dist(sx, sy, bx, by);

        minx = min(min(minx, sx - dist), bx);
        miny = min(min(miny, sy - dist), by);
        maxx = max(max(maxx, sx + dist), bx);
        maxy = max(max(maxy, sy + dist), by);

        points.insert((sx, sy), (bx, by, dist));
    }
    
    (points, (minx, maxx, miny, maxy))
}

fn manh_dist(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn solution1(input: &str, row: isize) -> isize {
    let (points, (minx, maxx, _, _)) = get_points(input);
    let mut counter = 0isize;

    for x in minx..=maxx {
        for (key, val) in &points {
            if [(val.0, val.1), *key].contains(&(x, row)) {
                continue
            }
            if manh_dist(key.0, key.1, x, row) <= val.2 {
                counter += 1;
                break
            }
        }
    }

    counter
}

fn solution2(input: &str, maxx: isize, maxy: isize) -> Option<isize> {
    let (points, _) = get_points(input);
    let columns: Vec<_> = (0..=maxx).collect();

    columns.par_iter()
        .find_map_any(|x| {
            let mut y = 0;
            'this: while y <= maxy {
                let mut poi = false;
                for (key, val) in &points {
                    if [(val.0, val.1), *key].contains(&(*x, y)) {
                        poi = true;
                        continue
                    }
                    if manh_dist(key.0, key.1, *x, y) <= val.2 {
                        for exp in 2..40 {
                            let cand_y = y - 1 + 2isize.pow(exp);
                            if manh_dist(key.0, key.1, *x, cand_y) > val.2 {
                                y = y - 1 + 2isize.pow(exp - 1);
                                continue 'this
                            }
                        }
                        continue 'this
                    }
                }
                if !poi {
                    return Some(x * 4000000 + y)
                }
                y += 1;
            }
            None
        })

}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn example() {
        assert_eq!(solution1(INPUT, 10), 26);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT, 20, 20), Some(56000011));
    }
}
