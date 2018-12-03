use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::vec::Vec;

fn main()  -> io::Result<()> {

	let mut changes = Vec::new();

    let f = File::open("input.txt")?;
    let f = BufReader::new(f);
    for line in f.lines() {
    	changes.push(line.unwrap().trim().parse::<i32>().unwrap());
    }
    println!("Final frequency: {}", changes.to_vec().iter().sum::<i32>());

    let mut second_time = 0;
    let mut freqs = HashSet::new();
    freqs.insert(0);
    let mut freq = 0;
    while second_time == 0 {
    	for change in &changes {
    		freq += change;
	    	if freqs.contains(&freq) {
	    		println!("Second time seeing {}", freq);
	    		second_time = 1;
	    		break;
	    	} else {
	    		freqs.insert(freq);
	    	}
	    }
	}

    Ok(())
}
