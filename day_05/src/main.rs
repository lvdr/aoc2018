use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::cmp;

fn is_alternate_case(a: u8, b: u8) -> bool {
    // ASCII upper- and lowercase letters are 32 code points apart
    return (a as i8 - b as i8).abs() == 32;
}

fn main() {
    let f = File::open("input").expect("Failed to open input.");
    let f = BufReader::new(f);

    let mut polymer = Vec::new();
    for byte in f.bytes() {
        let byte = byte.unwrap();

        // crude check if byte is a letter (primarily to avoid newline)
        if byte > 'z' as u8 || byte < 'A' as u8 {
            continue;
        }

        if !polymer.is_empty() && is_alternate_case(polymer[polymer.len()-1], byte) {
            polymer.pop();
        } else {
            polymer.push(byte);
        }
    }

    println!("First reduction length: {}", polymer.len());

    let mut best_polymer_length = polymer.len();
    for i in 'A' as u8..'Z' as u8 {
        let mut improved_polymer = Vec::new();
        for byte in &polymer {
            let byte = *byte;
            if byte == i || is_alternate_case(byte, i) {
                continue;
            }

            if !improved_polymer.is_empty()
               && is_alternate_case(improved_polymer[improved_polymer.len()-1], byte) {
                improved_polymer.pop();
            } else {
                improved_polymer.push(byte);
            }
        }
        best_polymer_length = cmp::min(best_polymer_length, improved_polymer.len());
    }
    println!("Improved polymer length: {}", best_polymer_length);
}
