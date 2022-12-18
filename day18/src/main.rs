use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let part1 = solution1(&input);
    let part2 = solution2(&input);
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

impl Point3D {
    fn dist(&self, other: &Point3D) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn adjacent(&self, other: &Point3D, diag: bool) -> bool {
        if !diag {
            return self.dist(other) == 1;
        }

        let distances = [
            (self.x - other.x).abs(),
            (self.y - other.y).abs(),
            (self.z - other.z).abs(),
        ];

        (distances.iter().sum::<isize>() <= 3) & (distances.iter().max() == Some(&1))
    }
}

impl FromStr for Point3D {
    type Err = ();
    fn from_str(input: &str) -> Result<Point3D, Self::Err> {
        let mut split = input.split(',');

        Ok(Point3D {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        })
    }
}

fn solution1(input: &str) -> usize {
    let points: Vec<_> = input
        .lines()
        .map(|s| Point3D::from_str(s).unwrap())
        .collect();
    let points2 = points.clone();
    let mut surface_area = 0usize;

    for point in points.into_iter() {
        let adjacent = points2.iter().filter(|p| point.adjacent(p, false)).count();
        surface_area += 6 - adjacent;
    }

    surface_area
}

fn solution2(input: &str) -> usize {
    let points: Vec<_> = input
        .lines()
        .map(|s| Point3D::from_str(s).unwrap())
        .collect();

    let occupied_space: HashSet<Point3D> = points.clone().into_iter().collect();
    let clusters = make_clusters(&points, true);

    dbg!(clusters.iter().map(|c| c.len()).collect::<Vec<usize>>());
    clusters
        .iter()
        .map(|c| exterior_sides(c, &occupied_space, false))
        .sum()
}

fn make_clusters(points: &[Point3D], diag: bool) -> Vec<Vec<Point3D>> {
    let mut clusters: Vec<Vec<Point3D>> = Vec::new();
    clusters.push(Vec::new());
    let mut cluster_idx = 0;

    'outer: for point in points.iter() {
        for cluster in 0..=cluster_idx {
            let close = clusters[cluster]
                .iter()
                .filter(|p| point.adjacent(p, diag))
                .count();
            if close > 0 {
                clusters[cluster].push(point.clone());
                continue 'outer;
            }
        }

        if clusters[cluster_idx].is_empty() {
            clusters[cluster_idx].push(point.clone());
        } else {
            clusters.push(vec![point.clone()]);
            cluster_idx += 1;
        }
    }

    // recombine clusters until stable
    while let Some(pair) = to_recombine(&clusters, diag) {
        recombine(&mut clusters, pair);
    }

    clusters
}

fn to_recombine(clusters: &[Vec<Point3D>], diag: bool) -> Option<(usize, usize)> {
    for (idx1, cluster1) in clusters.iter().enumerate() {
        for (idx2, cluster2) in clusters.iter().enumerate() {
            if idx1 >= idx2 {
                continue;
            }

            let touching = cluster1
                .iter()
                .any(|c1| cluster2.iter().any(|c2| c1.adjacent(c2, diag)));

            if touching {
                return Some((idx1, idx2));
            }
        }
    }

    None
}

fn recombine(clusters: &mut Vec<Vec<Point3D>>, pair: (usize, usize)) {
    let mut move_ = clusters[pair.0].clone();
    clusters[pair.1].append(&mut move_);
    clusters.remove(pair.0);
}

fn exterior_sides(points: &[Point3D], occupied: &HashSet<Point3D>, diag: bool) -> usize {
    if points.len() == 1 {
        return 6;
    }

    // can't have interior points for clusters smaller than 6
    if points.len() < 6 {
        let mut surface_area = 0usize;
        for point in points.iter() {
            let adjacent = points.iter().filter(|p| point.adjacent(p, diag)).count();
            surface_area += 6 - adjacent;
        }

        return surface_area;
    }

    // get cluster bounds with buffer of size 1 all around
    let x_iter = points.iter().map(|p| p.x);
    let x_bound = (x_iter.clone().min().unwrap() - 1)..=(x_iter.max().unwrap() + 1);
    let y_iter = points.iter().map(|p| p.y);
    let y_bound = (y_iter.clone().min().unwrap() - 1)..=(y_iter.max().unwrap() + 1);
    let z_iter = points.iter().map(|p| p.z);
    let z_bound = (z_iter.clone().min().unwrap() - 1)..=(z_iter.max().unwrap() + 1);

    // generate all unoccupied points within the buffered boundary
    let mut points_cluster = Vec::new();

    for x in x_bound.clone() {
        for y in y_bound.clone() {
            for z in z_bound.clone() {
                let candidate_point = Point3D { x, y, z };
                if !occupied.contains(&candidate_point) {
                    points_cluster.push(candidate_point);
                }
            }
        }
    }

    let mut space_cluster = make_clusters(&points_cluster, false);
    dbg!(&space_cluster
        .iter()
        .map(|c| c.len())
        .collect::<Vec<usize>>());
    let corner = Point3D {
        x: x_bound.last().unwrap(),
        y: y_bound.last().unwrap(),
        z: z_bound.last().unwrap(),
    };
    // keep only the outer space
    space_cluster.retain(|c| c.contains(&corner));

    //let mut filled_points: Vec<_> = points.iter().cloned().collect();

    //for cluster in space_cluster.iter_mut() {
    //    filled_points.append(cluster);
    //}

    //let mut surface_area = 0;
    //for point in filled_points.iter() {
    //    let adjacent = filled_points.iter().filter(|p| point.dist(p) == 1).count();
    //    surface_area += 6 - adjacent;
    //}

    let mut surface_area = 0;

    for point in points.iter() {
        surface_area += space_cluster[0]
            .iter()
            .filter(|s| point.adjacent(s, false))
            .count();
    }

    surface_area
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT2: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    const INPUT: &str = "1,0,0
-1,0,0
0,1,0
0,-1,0
0,0,1
0,0,-1
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    #[test]
    fn example() {
        assert_eq!(solution1(INPUT), 66);
        assert_eq!(solution1(INPUT2), 64);
    }

    #[test]
    fn example2() {
        assert_eq!(solution2(INPUT), 60);
        assert_eq!(solution2(INPUT2), 58);
    }
}
