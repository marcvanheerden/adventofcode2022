use rayon::prelude::*;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;
use std::thread;

fn main() {
    //make_big_input("input", "day05_input10000", 10000);
    let input = fs::read_to_string("day05_input10000").unwrap();
    let answer = solution(&input);
    println!("Part1: {} \nPart2: {}", answer.0, answer.1);
}

fn make_big_input(infile: &str, outfile: &str, scale_up: usize) {
    let input = fs::read_to_string(infile).unwrap();

    let mut splits = input.split("\n\n");
    let mut init = splits.next().unwrap().to_string();
    let moves = splits.next().unwrap();
    let mut extra_moves = "".to_string();
    let reverse_moves = moves
        .lines()
        .rev()
        .map(|s| {
            let splits = s.split_whitespace().collect::<Vec<_>>();
            let mut switch = "".to_owned();
            switch.push_str(splits[0]);
            switch.push(' ');
            switch.push_str(splits[1]);
            switch.push(' ');
            switch.push_str(splits[2]);
            switch.push(' ');
            switch.push_str(splits[5]);
            switch.push(' ');
            switch.push_str(splits[4]);
            switch.push(' ');
            switch.push_str(splits[3]);
            switch.push('\n');
            switch
        })
        .collect::<String>();

    for step in 1..=scale_up {
        if (step % 2) == 1 {
            extra_moves.push_str(&reverse_moves);
        } else {
            extra_moves.push_str(moves);
        }
    }

    init.push('\n');
    init.push('\n');
    init.push_str(moves);
    init.push_str(&extra_moves);

    fs::write(outfile, init).unwrap();
}

#[derive(Debug)]
struct Instruction {
    quant: u32,
    orig: usize,
    dest: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits: Vec<_> = s.split_whitespace().collect();
        let quant: u32 = splits[1].parse()?;
        let orig: usize = splits[3].parse::<usize>()?;
        let dest: usize = splits[5].parse::<usize>()?;

        // switch to 0-indexing
        Ok(Instruction {
            quant,
            orig: orig - 1,
            dest: dest - 1,
        })
    }
}

#[derive(Clone, Debug)]
struct CrateYard {
    stacks: Vec<Vec<char>>,
}

impl FromStr for CrateYard {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // assumes that labels are single digits

        let mut init = s.lines().rev();
        let positions: Vec<usize> = init
            .next()
            .expect("Empty input")
            .chars()
            .enumerate()
            .filter(|(_idx, val)| *val != ' ')
            .map(|(idx, _val)| idx)
            .collect();

        let mut stacks = vec![Vec::new(); positions.len()];

        for line in init {
            let chars: Vec<char> = line.chars().collect();
            for (idx, pos) in positions.iter().enumerate() {
                if line.len() <= (pos + 1) {
                    continue;
                }

                if chars[*pos] != ' ' {
                    stacks[idx].push(chars[*pos]);
                }
            }
        }
        Ok(CrateYard { stacks })
    }
}

impl CrateYard {
    fn move_crates_pt1(&mut self, inst: &Instruction) {
        for _ in 0..inst.quant {
            let moved = self.stacks[inst.orig].pop().unwrap();
            self.stacks[inst.dest].push(moved);
        }
    }

    fn move_crates_pt2(&mut self, inst: &Instruction) {
        let mut holder: Vec<char> = Vec::with_capacity(inst.quant as usize);
        for _ in 0..inst.quant {
            holder.push(self.stacks[inst.orig].pop().unwrap());
        }

        for chr in holder.into_iter().rev() {
            self.stacks[inst.dest].push(chr);
        }
    }

    fn top_crates(&self) -> String {
        self.stacks.iter().map(|v| v.last().unwrap()).collect()
    }
}

fn solution(instr: &str) -> (String, String) {
    let mut splits = instr.split("\n\n");
    let init = splits.next().unwrap();
    let moves = splits.next().unwrap();

    let mut crate_yard1 = CrateYard::from_str(init).unwrap();
    let mut crate_yard2 = crate_yard1.clone();

    let instructions: Vec<_> = moves
        .par_lines()
        .map(|s| Instruction::from_str(s).unwrap())
        .collect();

    let mut top_crates1 = "".to_string();
    let mut top_crates2 = "".to_string();

    thread::scope(|s| {
        let thread1 = s.spawn(|| {
            for instruction in instructions.iter() {
                crate_yard1.move_crates_pt1(instruction);
            }
            crate_yard1.top_crates()
        });
        let thread2 = s.spawn(|| {
            for instruction in instructions.iter() {
                crate_yard2.move_crates_pt2(instruction);
            }
            crate_yard2.top_crates()
        });
        top_crates1 = thread1.join().unwrap();
        top_crates2 = thread2.join().unwrap();
    });

    (top_crates1, top_crates2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn example() {
        assert_eq!(&solution(INPUT), &("CMZ".to_string(), "MCD".to_string()));
    }
}
