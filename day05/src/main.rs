use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let answer = solution(&input);
    println!("Part1: {} \nPart2: {}", answer.0, answer.1);
}

fn solution(instr: &str) -> (String, String) {
    let mut foo = instr.split("\n\n");

    let init: Vec<&str> = foo.next().unwrap().lines().collect();
    let moves = foo.next().unwrap();

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for (idx, chr) in init[init.len() - 1].chars().enumerate() {
        if chr == ' ' {
            continue
        }

        let mut stack = Vec::new();

        let mut stacks_ = init.iter().rev();
        stacks_.next();

        for line in stacks_ {
            if line.len() <= (idx + 1) {
                continue
            }

            let chars: Vec<char> = line.chars().collect();
            if chars[idx] != ' ' {
                stack.push(chars[idx]);
            }
        }
        stacks.push(stack);

    }

    let mut stacks2 = stacks.clone();

    for move_ in moves.lines() {
        let bar: Vec<_> = move_.split(' ').collect();
        let quant: u32 = bar[1].parse().unwrap();
        let orig: usize = bar[3].parse().unwrap();
        let dest: usize = bar[5].parse().unwrap();

        let mut holder: Vec<char> = Vec::new();

        for _ in 1..=quant {
            let trans = stacks[orig - 1].pop().unwrap();
            stacks[dest - 1].push(trans);

            holder.push(stacks2[orig - 1].pop().unwrap());
        }

        for chr in holder.iter().rev() {
            stacks2[dest - 1].push(*chr);
        }

    }

    dbg!(&stacks2);
    let mut output1 = "".to_string();
    let mut output2 = "".to_string();
    for stack in stacks.iter() {
        output1.push(stack[stack.len() - 1])
    }
    for stack in stacks2.iter() {
        output2.push(stack[stack.len() - 1])
    }
    (output1, output2)
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
