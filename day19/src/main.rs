use rayon::prelude::*;
use std::collections::VecDeque;
use std::fs;
use std::ops::{Add, Sub};
use std::str::FromStr;

const KEEPTOP: usize = 100;

fn main() {
    let input: String = fs::read_to_string("input").unwrap();
    let (part1, part2) = solution(&input);
    println!("Part1: {:?}\nPart2: {:?}", part1, part2);
}

#[derive(Debug, Copy, Clone)]
enum NextBot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct Materials {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Add for Materials {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl Sub for Materials {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl Materials {
    fn can_afford(&self, other: &Self) -> bool {
        (self.ore >= other.ore) & (self.clay >= other.clay) & (self.obsidian >= other.obsidian)
    }
}

impl FromStr for Materials {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let costs_split = input.split_once("costs ").unwrap();

        let mut ore = 0usize;
        let mut clay = 0usize;
        let mut obsidian = 0usize;

        let resource_split: Vec<_> = costs_split.1.split(" and ").collect();

        for resource in resource_split.into_iter() {
            let (amount, type_) = resource.split_once(' ').unwrap();
            match type_ {
                "ore" => ore += amount.parse::<usize>().unwrap(),
                "clay" => clay += amount.parse::<usize>().unwrap(),
                "obsidian" => obsidian += amount.parse::<usize>().unwrap(),
                _ => panic!("Unknown input"),
            }
        }

        Ok(Materials {
            ore,
            clay,
            obsidian,
            geode: 0,
        })
    }
}

#[derive(Debug, Clone)]
struct BluePrint {
    no: usize,
    ore_bot_cost: Materials,
    clay_bot_cost: Materials,
    obsidian_bot_cost: Materials,
    geode_bot_cost: Materials,
}

#[derive(Debug, Clone)]
struct Fleet {
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
}

impl Fleet {
    fn harvest(&self) -> Materials {
        Materials {
            ore: self.ore_bots,
            clay: self.clay_bots,
            obsidian: self.obsidian_bots,
            geode: self.geode_bots,
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    fleet: Fleet,
    blueprint: BluePrint,
    materials: Materials,
    time_left: usize,
    next: NextBot,
}

impl FromStr for BluePrint {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let colon_split = input.split_once(':').unwrap();
        let no = colon_split.0.split_once(' ').unwrap().1.parse().unwrap();

        let bot_split: Vec<_> = colon_split.1.split('.').collect();

        let ore_bot_cost = Materials::from_str(bot_split[0])?;
        let clay_bot_cost = Materials::from_str(bot_split[1])?;
        let obsidian_bot_cost = Materials::from_str(bot_split[2])?;
        let geode_bot_cost = Materials::from_str(bot_split[3])?;

        Ok(BluePrint {
            no,
            ore_bot_cost,
            clay_bot_cost,
            obsidian_bot_cost,
            geode_bot_cost,
        })
    }
}

fn optimal_path(
    fleet: Fleet,
    blueprint: BluePrint,
    materials: Materials,
    time_left: usize,
) -> usize {
    let mut tasks: VecDeque<Task> = VecDeque::new();

    tasks.push_back(Task {
        fleet: fleet.clone(),
        blueprint: blueprint.clone(),
        materials,
        time_left,
        next: NextBot::Clay,
    });

    tasks.push_back(Task {
        fleet,
        blueprint,
        materials,
        time_left,
        next: NextBot::Ore,
    });

    let mut finished_tasks = Vec::new();

    while !tasks.is_empty() {
        // score the tasks in the queue, keep the best ones
        if tasks.len() >= KEEPTOP {
            let mut task_vec: Vec<_> = tasks.into_iter().collect();
            task_vec.sort_by_key(|t| {
                t.fleet.geode_bots * t.time_left * 1000
                    + t.fleet.obsidian_bots * t.time_left * 100
                    + t.fleet.clay_bots * t.time_left * 10
                    + t.fleet.ore_bots * t.time_left
                    + t.materials.geode * 1000
                    + t.materials.obsidian * 100
                    + t.materials.clay * 10
                    + t.materials.ore
            });
            tasks = task_vec.into_iter().rev().take(KEEPTOP).collect();
        }

        let mut new_tasks = VecDeque::new();
        while !tasks.is_empty() {
            let mut t = tasks.pop_front().unwrap();

            t.time_left -= 1;

            if t.time_left == 0 {
                let mut t1 = t.clone();
                t1.materials = t1.materials + t1.fleet.harvest();
                finished_tasks.push(t);
                continue;
            }

            let mut t1 = t.clone();
            match t.next {
                NextBot::Ore => {
                    if t1.materials.can_afford(&t1.blueprint.ore_bot_cost) {
                        t1.materials = t1.materials + t1.fleet.harvest();
                        t1.fleet.ore_bots += 1;
                        t1.materials = t1.materials - t1.blueprint.ore_bot_cost;
                        new_tasks.push_back(t1.clone());
                        t1.next = NextBot::Clay;
                        new_tasks.push_back(t1.clone());
                        if t1.fleet.clay_bots > 0 {
                            t1.next = NextBot::Obsidian;
                            new_tasks.push_back(t1.clone());
                        }
                        if t1.fleet.obsidian_bots > 0 {
                            t1.next = NextBot::Geode;
                            new_tasks.push_back(t1);
                        }
                        continue;
                    }
                }
                NextBot::Clay => {
                    if t1.materials.can_afford(&t1.blueprint.clay_bot_cost) {
                        t1.materials = t1.materials + t1.fleet.harvest();
                        t1.fleet.clay_bots += 1;
                        t1.materials = t1.materials - t1.blueprint.clay_bot_cost;
                        new_tasks.push_back(t1.clone());
                        t1.next = NextBot::Ore;
                        new_tasks.push_back(t1.clone());
                        t1.next = NextBot::Obsidian;
                        new_tasks.push_back(t1.clone());
                        if t1.fleet.obsidian_bots > 0 {
                            t1.next = NextBot::Geode;
                            new_tasks.push_back(t1);
                        }
                        continue;
                    }
                }
                NextBot::Obsidian => {
                    if t1.materials.can_afford(&t1.blueprint.obsidian_bot_cost) {
                        t1.materials = t1.materials + t1.fleet.harvest();
                        t1.fleet.obsidian_bots += 1;
                        t1.materials = t1.materials - t1.blueprint.obsidian_bot_cost;
                        new_tasks.push_back(t1.clone());
                        t1.next = NextBot::Ore;
                        new_tasks.push_back(t1.clone());
                        t1.next = NextBot::Clay;
                        new_tasks.push_back(t1.clone());
                        t1.next = NextBot::Geode;
                        new_tasks.push_back(t1);
                        continue;
                    }
                }
                NextBot::Geode => {
                    if t1.materials.can_afford(&t1.blueprint.geode_bot_cost) {
                        t1.materials = t1.materials + t1.fleet.harvest();
                        t1.fleet.geode_bots += 1;
                        t1.materials = t1.materials - t1.blueprint.geode_bot_cost;
                        new_tasks.push_back(t1.clone());
                        t1.next = NextBot::Ore;
                        new_tasks.push_back(t1.clone());
                        t1.next = NextBot::Clay;
                        new_tasks.push_back(t1.clone());
                        t1.next = NextBot::Obsidian;
                        new_tasks.push_back(t1);
                        continue;
                    }
                }
            }
            // no new bot path
            t.materials = t.materials + t.fleet.harvest();
            new_tasks.push_back(t.clone());
        }
        tasks.append(&mut new_tasks);
    }

    finished_tasks
        .into_iter()
        .map(|t| t.materials.geode)
        .max()
        .unwrap()
}

fn solution(input: &str) -> (usize, usize) {
    let blueprints: Vec<_> = input
        .lines()
        .map(|l| BluePrint::from_str(l).unwrap())
        .collect();

    let start_fleet = Fleet {
        ore_bots: 1,
        clay_bots: 0,
        obsidian_bots: 0,
        geode_bots: 0,
    };
    let start_materials = Materials {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };

    let part1 = blueprints
        .par_iter()
        .map(|b| optimal_path(start_fleet.clone(), b.clone(), start_materials, 24 + 1) * b.no)
        .sum();

    let part2: usize = blueprints
        .par_iter()
        .take(3)
        .map(|b| optimal_path(start_fleet.clone(), b.clone(), start_materials, 32 + 1))
        .product();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn example() {
        let (answer1, _) = solution(INPUT);
        assert_eq!(answer1, 33);
    }
}
