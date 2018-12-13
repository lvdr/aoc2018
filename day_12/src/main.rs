use std::io::prelude::*;
use std::fs::File;
use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "initial state: #..#.#..##......###...###\n\
                              \n\
                              ...## => #\n\
                              ..#.. => #\n\
                              .#... => #\n\
                              .#.#. => #\n\
                              .#.## => #\n\
                              .##.. => #\n\
                              .#### => #\n\
                              #.#.# => #\n\
                              #.### => #\n\
                              ##.#. => #\n\
                              ##.## => #\n\
                              ###.. => #\n\
                              ###.# => #\n\
                              ####. => #";

    #[test]
    fn test_both_halves() {
        let input = String::from(TEST_INPUT);
        let (mut pots, rules) = parse_input(input);

        for _ in 0..20 {
            pots.evolve(&rules);
        }

        assert_eq!(pots.sum_plants(), 325);
    }
}

#[derive(Debug, Clone)]
struct Pots {
    living: VecDeque<bool>,
    offset: isize,
}

impl Pots {

    fn evolve(&mut self, rules: &[bool; 32]) {

        let mut leftmost = 0;        
        for i in 0..self.living.len()-2 {
            leftmost = leftmost*2 + self.living[i+2] as usize;
            self.living[i] = rules[leftmost];
            leftmost &= 0b1111;
        }

        if !self.living[0] && !self.living[1] && !self.living[2] {
            while !self.living[2] {
                self.living.pop_front();
                self.offset += 1;
            }
        } else {
            while self.living[0] || self.living[1] {
                self.living.push_front(false);
                self.offset -= 1;
            }
        }

        for i in 0..4 {
            if self.living[self.living.len()-1-i] {
                for _ in 0..4-i {
                    self.living.push_back(false);
                }
                break;
            }
        }

        while !self.living[self.living.len()-5] {
            self.living.pop_back();
        }
    }

    fn sum_plants(&self) -> i64 {
        let mut sum: i64 = 0;
        for i in 0..self.living.len() {
            if self.living[i] {
                sum += (i as isize + self.offset) as i64;
            }
        }
        sum
    }
}

fn parse_input(input: String) -> (Pots, [bool; 32]) {
    let mut lines = input.lines();

    let initial_state: VecDeque<bool> = lines.next().unwrap()
                                             .split(' ').nth(2)
                                             .unwrap().trim()
                                             .bytes().map(|b| b == '#' as u8)
                                             .collect();

    let mut pots = Pots{ living: initial_state, offset: -2 };

    for _ in 0..4 {
        pots.living.push_back(false);
    }

    pots.living.push_front(false);
    pots.living.push_front(false);


    lines.next();

    let mut rules = [false; 32];
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut halves = line.split(" => ");
        let cause = halves.next().unwrap();
        let result = halves.next().unwrap();
        if result == "." {
            continue;
        }

        let mut byte_val = 0;
        for byte in cause.bytes() {
            byte_val *= 2;
            if byte == '#' as u8 {
                byte_val |= 0b1;
            }
        }
        rules[byte_val] = true;
    }
    (pots, rules)
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let (mut pots, rules) = parse_input(input);

    for _ in 0..20 {
        pots.evolve(&rules);
    }

    println!("Value after 20 steps: {}", pots.sum_plants());

    for _ in 20u64..50_000_000_000u64 {
        pots.evolve(&rules);
    }

    println!("Value after 50_000_000_000 steps: {}", pots.sum_plants());
}
