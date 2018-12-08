use std::io::prelude::*;
use std::fs::File;
use std::slice::Iter;

fn metadata_sum(tokens: &mut Iter<u32>) -> u32 {
	let mut sum = 0;
	let child_nodes = *tokens.next().unwrap();
	let metadata_entries = *tokens.next().unwrap();

	for _ in 0..child_nodes {
		sum += metadata_sum(tokens);
	}

	for _ in 0..metadata_entries {
		sum += tokens.next().unwrap();
	}
	sum
}

fn node_value(tokens: &mut Iter<u32>) -> u32 {
	let mut sum = 0;
	let child_nodes = *tokens.next().unwrap();
	let metadata_entries = *tokens.next().unwrap();

	let mut child_values = Vec::new();
	for _ in 0..child_nodes {
		child_values.push(node_value(tokens));
	}

	if child_nodes == 0 {
		for _ in 0..metadata_entries {
			sum += tokens.next().unwrap();
		}
	} else {
		for _ in 0..metadata_entries {
			let entry = *tokens.next().unwrap() as usize;
			if entry > child_values.len() {
				continue;
			}

			sum += child_values[entry-1];
		}
	}
	sum
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let tokens: Vec<u32> = input.trim().split(" ")
                                .map(|x| x.parse::<u32>().unwrap())
                                .collect();

    let sum = metadata_sum(&mut tokens.iter());
    println!("Sum of metadata: {}", sum);

    let value = node_value(&mut tokens.iter());
    println!("Value of first node: {}", value);
}
