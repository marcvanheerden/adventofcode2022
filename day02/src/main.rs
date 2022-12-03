use std::fs;
use rayon::prelude::*;

fn main() {
    let input = fs::read_to_string("input10000").unwrap();
    let answer = solution(&input);
    println!("Part1: {} \nPart2: {}", answer.0, answer.1);
}

fn solution(plays: &str) -> (u32, u32) {
    let mut plays = plays.replace('A', "X");
    plays = plays.replace('B', "Y");
    plays = plays.replace('C', "Z");

    plays
        .par_lines()
        .map(|s| {
            let mut signs = s.split(' ');
            let opp = signs.next().unwrap();
            let you = signs.next().unwrap();

            let sign_score_pt1: u32 = match you {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!(),
            };

            let outcome_score_pt1: u32 = match (opp, you, opp == you) {
                (_, _, true) => 3,
                ("X", "Y", false) => 6,
                ("Y", "Z", false) => 6,
                ("Z", "X", false) => 6,
                (_, _, _) => 0,
            };

            let outcome_score_pt2: u32 = match you {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!(),
            };

            let sign_score_pt2: u32 = match (opp, you) {
                ("X", "X") => 3,
                ("Y", "X") => 1,
                ("Z", "X") => 2,
                ("X", "Y") => 1,
                ("Y", "Y") => 2,
                ("Z", "Y") => 3,
                ("X", "Z") => 2,
                ("Y", "Z") => 3,
                ("Z", "Z") => 1,
                (_, _) => panic!(),
            };

            (
                sign_score_pt1 + outcome_score_pt1,
                sign_score_pt2 + outcome_score_pt2,
            )
        })
        .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn example() {
        assert_eq!(solution(INPUT), (15, 12));
    }
}
