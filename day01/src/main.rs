use std::fs;

fn main() {

    let input = fs::read_to_string("input").unwrap();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(cals: &str) -> u32 {
    
    let mut max = 0;
    let mut run = 0;

    for line in cals.lines() {
        if line.len() < 1 {
            if run > max {
                max = run;
            }
            run = 0;
            continue
        }

        run += line.parse::<u32>().unwrap();

    }

    max
}

fn part2(cals: &str) -> u32 {
    
    let mut top: Vec<u32> = Vec::new();
    let mut run = 0;

    for line in cals.lines() {
        dbg!(run);
        if line.len() < 1 {
            dbg!(&top);
            if top.len() < 3 {
                top.push(run);
                run = 0;
                continue
            }
            for val in top.iter_mut() {
                if val < &mut run {
                    *val = run;
                    break
                }
            }
            run = 0;
            continue
        }

        run += line.parse::<u32>().unwrap();

    }
    for val in top.iter_mut() {
                if val < &mut run {
                    *val = run;
                    break
                }
            }
    dbg!(&top);

    top.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(part1(input), 24000);
    }

    #[test]
    fn part2_example() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        assert_eq!(part2(input), 45000);
    }
}
