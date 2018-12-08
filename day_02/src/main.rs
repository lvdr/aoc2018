use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let input = String::from("abcdef\n\
                                  bababc\n\
                                  abbcde\n\
                                  abcccd\n\
                                  aabcdd\n\
                                  abcdee\n\
                                  ababab");
        let ids = parse_input(&input);
        assert_eq!(hash(&ids), 12);
    }

    #[test]
    fn test_second_half() {
        let input = String::from("abcde\n\
                                  fghij\n\
                                  klmno\n\
                                  pqrst\n\
                                  fguij\n\
                                  axcye\n\
                                  wvxyz");
        let ids = parse_input(&input);
        assert_eq!(seek_dupes(&ids), String::from("fgij"));
    }
}

fn hash(ids: &Vec<String>) -> i32 {
    let mut twos = 0;
    let mut threes = 0;
    for id in ids.into_iter() {
        let mut counts = HashMap::new();
        for character in id.trim().chars() {
            *counts.entry(character).or_insert(0) += 1;
        }

        if counts.values().any(|&x| x == 2) {
            twos += 1;
        }
        if counts.values().any(|&x| x == 3) {
            threes += 1;
        }
    }
    twos*threes
}

fn seek_dupes(ids: &Vec<String>) -> String {
    let id_len = ids[0].len();
    for i in 0..id_len {
        let mut rem_set = HashSet::new();
        for id in ids {
            let dupe = id.clone();
            let mut removed = format!("{}{}", &dupe[..i], &dupe[i+1..]);
            if rem_set.contains(&removed) {
                return String::from(removed);
            } else {
                rem_set.insert(removed);
            }
        }
    }
    String::from("Failed to find duplicate!")
}

fn parse_input(input: &String) -> Vec<String> {
    input.trim().lines()
         .map(|l| String::from(l))
         .collect()
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let ids = parse_input(&input);

    println!("Hash: {}", hash(&ids));
    println!("Duplicate: {}", seek_dupes(&ids));
}
