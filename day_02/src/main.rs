use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

fn hash(ids: &[String]) -> i32 {
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

fn seek_dupes(i: usize, ids: &[String]) {
    let mut rem_set = HashSet::new();
    for id in ids {
        let dupe = id.clone();
        let mut removed = format!("{}{}", &dupe[..i], &dupe[i+1..]);
        if rem_set.contains(&removed) {
            println!("Duplicate found: {}", removed);
        } else {
            rem_set.insert(removed);
        }
    }
}

fn main() {
    let f = File::open("input").expect("Failed to open input.");
    let f = BufReader::new(f);
    let mut ids = Vec::new();
    for line in f.lines() {
        ids.push(line.unwrap());
    }

    println!("Hash: {}", hash(ids.as_slice()));

    for i in 0..26 {
        seek_dupes(i, ids.as_slice());
    }
}
