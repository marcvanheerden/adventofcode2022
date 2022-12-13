use fxhash::FxHashMap;
use std::collections::VecDeque;
use std::fs;
use std::thread;

fn main() {
    //let rands = fs::read_to_string("day06_input10000_pre").unwrap();
    //let input = make_big_input(&rands);
    //fs::write("day06_input10000", &input).unwrap();
    //let input = fs::read_to_string("day06_input10000").unwrap();
    let input = "abcdefghijklmnopqrsatuav".to_owned();
    let mut part1: Option<usize> = None;
    let mut part2: Option<usize> = None;

    thread::scope(|s| {
        let thread1 = s.spawn(|| solution(&input, 4));
        part2 = solution(&input, 14);
        part1 = thread1.join().unwrap();
    });
    println!("Part1: {:?} \nPart2: {:?}", part1, part2);
}

fn make_big_input(rand_chars: &str) -> String {
    let mut copy: Vec<char> = rand_chars.clone().chars().collect();
    let mut last_change = 0usize;

    let mut switch = true;
    for idx in 0..copy.len() {
        if ((4096usize * 4999)..(4096usize * 5001)).contains(&idx) {
            continue;
        }

        switch = !switch;
        let period = if switch { 3usize } else { 2 };

        if idx == (last_change + 2) {
            copy[idx] = copy[idx - period];
            last_change = idx;
        }
    }

    copy.into_iter().collect()
}

fn solution(signal: &str, window_size: usize) -> Option<usize> {
    let mut window = VecDeque::new();
    let mut tracker: FxHashMap<char, u32> = FxHashMap::default();

    for (idx, sig) in signal.chars().enumerate() {
        if window.len() >= window_size {
            let record = tracker.get_mut(&window.pop_back().unwrap()).unwrap();
            *record -= 1;
        }

        window.push_front(sig);
        let record = tracker.entry(sig).or_insert(0);
        *record += 1;

        if tracker.values().filter(|i| i > &&0).count() >= window_size {
            return Some(idx + 1);
        }
    }
    None
}

fn solution2(signal: &str, window_size: usize) -> Option<usize> {
    let mut window = VecDeque::new();

    for (idx, sig) in signal.chars().enumerate() {
        window.push_front(sig);
        if window.len() >= window_size {
            if window.len() > window_size {
                window.pop_back().unwrap();
            }
            for idx1 in 0..(window_size - 1) {
                for idx2 in (idx1 + 1)..window_size {
                    if window[idx1] == window[idx2] {
                        return Some(idx + 2);
                    }
                }
            }
        }
        
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input1 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solution2(input1, 4), Some(5));
    }
}
