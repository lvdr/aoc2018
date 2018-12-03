use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main()  -> io::Result<()> {

    let mut freq = 0;
    let mut second_time = 0;
    let mut freqs = HashSet::new();
    freqs.insert(0);
    let mut got_final_freq = 0;
    while second_time == 0 {
	    let f = File::open("input.txt")?;
	    let f = BufReader::new(f);
	    for line in f.lines() {
	    	let unwrapped = line.unwrap();
	    	let change: i32 = unwrapped.trim()[1..].parse().unwrap();
	    	if unwrapped[0..1].to_string() == "+" {
	    		freq += change;
	    	} else {
	    		freq -= change;
	    	}
	    	if second_time == 0 && freqs.contains(&freq) {
	    		println!("Second time seeing {}", freq);
	    		second_time = 1;
	    	} else {
	    		freqs.insert(freq);
	    	}
	    }
	    if got_final_freq == 0 {
			println!("Final frequency: {}", freq);
			got_final_freq = 1;
		}
	}

    Ok(())
}
