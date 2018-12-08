use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::vec::Vec;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        test_final_freq("+1\n-2\n+3\n+1\n", 3);
        test_final_freq("+1\n+1\n+1\n", 3);
        test_final_freq("+1\n+1\n-2", 0);
        test_final_freq("-1\n-2\n-3", -6);
    }

    fn test_final_freq(input: &str, result: i32) {
        let input = String::from(input);
        let changes = parse_input(&input);
        assert_eq!(final_frequency(&changes), result);
    }

    #[test]
    fn test_second_half() {
        // Second half
        test_repeat_freq("+1\n-1\n", 0);
        test_repeat_freq("+3\n+3\n+4\n-2\n-4\n", 10);
        test_repeat_freq("-6\n+3\n+8\n+5\n-6", 5);
        test_repeat_freq("+7\n+7\n-2\n-7\n-4", 14);
    }

    fn test_repeat_freq(input: &str, result: i32) {
        let input = String::from(input);
        let changes = parse_input(&input);
        assert_eq!(repeat_frequency(&changes), result);
    }
}

fn parse_input(input : &String) -> Vec<i32> {
    input.trim().lines()
         .map(|x| x.parse::<i32>().unwrap())
         .collect()
}

fn final_frequency(changes : &Vec<i32>) -> i32 {
    changes.iter().sum::<i32>()
}

fn repeat_frequency(changes: &Vec<i32>) -> i32 {
    let mut freqs = HashSet::new();
    let mut freq = 0;
    for change in changes.iter().cycle() {
        if freqs.contains(&freq) {
            break;
        }
        freqs.insert(freq);
        freq += change;
    }
    freq
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

	let changes = parse_input(&input);
    let final_frequency = final_frequency(&changes);
    println!("Final frequency: {}", final_frequency);

    let repeat_frequency = repeat_frequency(&changes);
    println!("First repeated frequency: {}", repeat_frequency);
}
