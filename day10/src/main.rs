use std::fs;
use std::str::FromStr;
use fxhash::FxHashSet;
use rayon::prelude::*;
mod big_input;

fn main() {
    //let biginput = big_input::create();
    //fs::write("day10_input32000", biginput).unwrap();

    let input: String = fs::read_to_string("day10_input32000").unwrap();
    let commands: Vec<_> = input.par_lines().map(|l| Cmd::from_str(l).unwrap()).collect();
    let mut tracer = Tracer::new(40);

    tracer.run(&commands, &[20, 60, 100, 140, 180, 220]);
    println!("Part1: {}\nPart2:", tracer.checksum);
    tracer.display_frame();
}

#[derive(Debug)]
enum Cmd {
    Noop,
    Addx(i32),
}

impl FromStr for Cmd {
    type Err = ();
    fn from_str(input: &str) -> Result<Cmd, Self::Err> {
        if input == "noop" {
            return Ok(Cmd::Noop);
        }

        let splits = input.split_once(' ').unwrap();

        Ok(Cmd::Addx(splits.1.parse::<i32>().unwrap()))
    }
}

struct Tracer {
    reg_x: i32,
    cycle: usize,
    screen: Vec<bool>,
    width: usize,
    checksum: usize,
}

impl Tracer {
    fn new(width: usize) -> Tracer {
        Tracer {
            reg_x: 1,
            cycle: 0,
            width,
            screen: Vec::new(),
            checksum: 0,
        }
    }

    fn process_instr(&mut self, command: &Cmd, checks: &FxHashSet<usize>) {
        // check if sprite and position overlap
        let sprite = (self.reg_x - 1)..=(self.reg_x + 1);
        let position = (self.cycle % self.width) as i32;
        self.screen.push(sprite.contains(&position));
        self.cycle += 1;

        if checks.contains(&self.cycle) {
            self.checksum += self.cycle * (self.reg_x as usize);
        }

        if let Cmd::Addx(shift) = command {
            self.process_instr(&Cmd::Noop, checks);
            self.reg_x += shift;
        }
    }

    fn run(&mut self, commands: &[Cmd], checks: &[usize]) {

        let mut checks_ = FxHashSet::default();

        for check in checks {
            checks_.insert(*check);
        }

        for command in commands {
            self.process_instr(command, &checks_);
        }
    }

    fn display_frame(&self) {
        let mut start = 0;
        let mut end = self.width;

        while end <= self.screen.len() {
            println!(
                "{}",
                &self.screen[start..end]
                    .iter()
                    .map(|b| if *b { '#' } else { '.' })
                    .collect::<String>()
            );
            start += self.width;
            end += self.width;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
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

    #[test]
    fn example() {
        let commands: Vec<_> = INPUT.lines().map(|l| Cmd::from_str(l).unwrap()).collect();
        let mut tracer = Tracer::new(40);
        tracer.run(&commands, &vec![20, 60, 100, 140, 180, 220]);
        assert_eq!(tracer.checksum, 13140);
    }

    #[test]
    fn test_encode() {
        let commands: Vec<_> = INPUT.lines().map(|l| Cmd::from_str(l).unwrap()).collect();
        let mut tracer = Tracer::new(40);
        tracer.run(&commands, &Vec::new());
        let mimic_input = big_input::encode(&tracer.screen, 40);
        let commands2: Vec<_> = mimic_input
            .lines()
            .map(|l| Cmd::from_str(l).unwrap())
            .collect();
        let mut tracer2 = Tracer::new(40);
        tracer2.run(&commands2, &Vec::new());
        assert!(tracer
            .screen
            .iter()
            .zip(tracer2.screen.iter())
            .all(|(x, y)| x == y));
    }
}
