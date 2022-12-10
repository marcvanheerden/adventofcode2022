use std::fs;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    println!("{}", solution(&input, vec![20, 60, 100, 140, 180, 220]));
    solution2(&input);
}

fn solution(input: &str, checks: Vec<usize>) -> i32 {

    let mut cycle = 0usize;
    let mut reg = 1i32;
    let mut points = 0i32;

    for inst in input.lines() {

        if inst == "noop" {
            cycle += 1;
            if checks.contains(&cycle) {
                points += reg * (cycle as i32);
            }
            continue
        }

        let split = inst.split_once(' ').unwrap();
        
        for _ in 0..2 {
            cycle += 1;
            if checks.contains(&cycle) {
                points += reg * (cycle as i32);
            }
        }
        reg += split.1.parse::<i32>().unwrap();
    }

    points
}

fn solution2(input: &str) {

    let mut cycle = 0usize;
    let mut reg = 1i32;
    let mut screen = vec!['.'; 240];

    for inst in input.lines() {
        if inst == "noop" {
            let sprite = (reg - 1)..=(reg + 1);
            let horiz = cycle - (cycle / 40) * 40;
            for pix in sprite {
                if horiz == (pix as usize) {
                    screen[cycle] = '#';
                    break
                }
            }
            cycle += 1;

            continue
        }

        let split = inst.split_once(' ').unwrap();
        
        for _ in 0..2 {
            let sprite = (reg - 1)..=(reg + 1);
            let horiz = cycle - (cycle / 40) * 40;
            for pix in sprite {
                if horiz == (pix as usize) {
                    screen[cycle] = '#';
                    break
                }
            }
            cycle += 1;
        }
        reg += split.1.parse::<i32>().unwrap();

    }

    dbg!(&screen);
    println!("{}", &screen[0..40].iter().collect::<String>());
    println!("{}", &screen[40..80].iter().collect::<String>());
    println!("{}", &screen[80..120].iter().collect::<String>());
    println!("{}", &screen[120..160].iter().collect::<String>());
    println!("{}", &screen[160..200].iter().collect::<String>());
    println!("{}", &screen[200..240].iter().collect::<String>());

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        let part1 = solution(input, vec![20, 60, 100, 140, 180, 220]);
        assert_eq!(part1, 13140);
        solution2(input);
        assert!(false);
    }


}
