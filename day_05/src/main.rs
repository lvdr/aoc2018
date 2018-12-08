use std::io::prelude::*;
use std::fs::File;
use std::cmp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_reduction() {
        let input = "dabAcCaCBAcCcaDA".bytes().collect();
        assert_eq!(reduce_polymer(&input, None), 10);
    }

    #[test]
    fn test_best_improvement() {
        let input = "dabAcCaCBAcCcaDA".bytes().collect();
        assert_eq!(find_best_improvement(&input), 4);
    }
}

fn is_alternate_case(a: u8, b: u8) -> bool {
    // ASCII upper- and lowercase letters are 32 code points apart
    return a == b ^ 32;
}

fn reduce_polymer(polymer: &Vec<u8>, ignored_letter: Option<u8>) -> usize {
    let mut reduced_polymer = Vec::new();
    for byte in polymer {
        let byte = *byte;

        if byte < 'A' as u8 || byte > 'z' as u8 {
            continue;
        }

        if ignored_letter != None {
            let ignored_letter = ignored_letter.unwrap();
            if    byte == ignored_letter
               || is_alternate_case(byte, ignored_letter) {
                continue;
            }
        }

        if !reduced_polymer.is_empty()
           && is_alternate_case(reduced_polymer[reduced_polymer.len()-1], byte) {
            reduced_polymer.pop();
        } else {
            reduced_polymer.push(byte);
        }
    }
    reduced_polymer.len()
}

fn find_best_improvement(polymer: &Vec<u8>) -> usize {
    let mut best_polymer_length = polymer.len();
    for i in 'A' as u8..'Z' as u8 {
        best_polymer_length = cmp::min(best_polymer_length,
                                       reduce_polymer(&polymer, Some(i)));
    }
    best_polymer_length
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");
    
    let polymer = input.bytes().collect();
    println!("First reduction length: {}", reduce_polymer(&polymer, None));
    println!("Improved polymer length: {}", find_best_improvement(&polymer));
}
