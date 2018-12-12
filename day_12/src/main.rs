use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

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
            evolve(&mut pots, &rules);
        }

        assert_eq!(sum_plants(&pots), 325);
    }
}

#[derive(Debug, Clone)]
struct Pots {
    positive: Vec<bool>,
    negative: Vec<bool>,
}

fn get(pots: &Pots, pos: isize) -> bool {
    if pos >= 0 {
        let idx = pos as usize;
        if idx >= pots.positive.len() {
            false
        } else {
            pots.positive[pos as usize]
        }
    } else {
        pots.negative[(-pos-1) as usize]
    }
}

fn set(pots: &mut Pots, pos: isize, value: bool) {
    if pos >= 0 {
        pots.positive[pos as usize] = value;
    } else {
        pots.negative[(-pos-1) as usize] = value;
    }
}

fn evolve(pots: &mut Pots, rules: &HashSet<u32>) {
    let min = -(pots.negative.len() as isize);
    let max = pots.positive.len() as isize;

    let mut leftmost = 0;
    for i in min..max {
        let neighbors = leftmost*2 + get(pots, i+2) as u32;
        set(pots, i, rules.contains(&neighbors));
        leftmost = neighbors & 0b1111;
    }

    if pots.negative[(-min-1) as usize] {
        pots.negative.append(&mut vec![false, false]);
    } else if pots.negative[(-min-2) as usize] {
        pots.negative.push(false);
    }

    if pots.positive[(max-1) as usize] {
        pots.positive.append(&mut vec![false, false]);
    } else if pots.positive[(max-2) as usize] {
        pots.positive.push(false);
    }
}

fn sum_plants(pots: &Pots) -> i64 {
    let min = -(pots.negative.len() as isize);
    let max = pots.positive.len() as isize;

    let mut sum: i64 = 0;
    for i in min..max {
        if get(pots, i) {
            sum += i;
        }
    }
    sum as i64
}

fn parse_input(input: String) -> (Pots, HashSet<u32>) {
    let mut lines = input.lines();

    let mut initial_state: Vec<bool> = lines.next().unwrap()
                                            .split(' ').nth(2)
                                            .unwrap().trim()
                                            .bytes().map(|b| b == '#' as u8)
                                            .collect();
    initial_state.append(&mut vec![false, false]);

    // extend vectors so that lookup is easier
    let pots = Pots{ positive: initial_state,
                     negative: vec![false, false] };
    lines.next();

    let mut rules = HashSet::new();
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
        rules.insert(byte_val);
    }
    (pots, rules)
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let (mut pots, rules) = parse_input(input);

    for _ in 0..20 {
        evolve(&mut pots, &rules);
    }

    println!("Value after 20 steps: {}", sum_plants(&pots));


    for _ in 20..50000000000 as u64 {
        evolve(&mut pots, &rules);
    }

    println!("Value after 50000000000 steps: {}", sum_plants(&pots));
}
