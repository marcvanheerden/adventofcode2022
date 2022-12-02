use std::fs;

fn main() {
    
    let input = fs::read_to_string("input").unwrap();
    println!("Part1: {}", solution(&input));
    println!("Part2: {}", solution2(&input));
}

fn solution(plays: &str) -> u32 {

    let mut plays = plays.replace("A", "X");
    plays = plays.replace("B", "Y");
    plays = plays.replace("C", "Z");
    
    plays.lines()
        .map(|s| {
            let mut signs = s.split(' ');
            let opp = signs.next().unwrap();
            let you = signs.next().unwrap();

            let sign_score: u32 = match you {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => panic!()
            };

            let outcome_score: u32 = match (opp, you, opp == you) {
                (_, _, true) => 3,
                ("X", "Y", false) => 6,
                ("Y", "Z", false) => 6,
                ("Z", "X", false) => 6,
                (_, _, _) => 0
            };

            sign_score + outcome_score
        })
        .sum()
}

fn solution2(plays: &str) -> u32 {

    plays.lines()
        .map(|s| {
            let mut signs = s.split(' ');
            let opp = signs.next().unwrap();
            let result = signs.next().unwrap();

            let outcome_score: u32 = match result {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!()
            };

            let sign_score: u32 = match (opp, result) {
                ("A", "X") => 3,
                ("B", "X") => 1,
                ("C", "X") => 2,
                ("A", "Y") => 1,
                ("B", "Y") => 2,
                ("C", "Y") => 3,
                ("A", "Z") => 2,
                ("B", "Z") => 3,
                ("C", "Z") => 1,
                (_, _) => panic!()
            };

            sign_score + outcome_score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1_example() {
        assert_eq!(solution(INPUT), 15);
    }
    #[test]
    fn part2_example() {
        assert_eq!(solution2(INPUT), 12);
    }
}
