use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashSet, HashMap};

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_0: &str = 
                "0,0,0,0\n\
                 3,0,0,0\n\
                 0,3,0,0\n\
                 0,0,3,0\n\
                 0,0,0,3\n\
                 0,0,0,6\n\
                 9,0,0,0\n\
                12,0,0,0";


    const TEST_INPUT_1: &str = 
                "-1,2,2,0\n\
                 0,0,2,-2\n\
                 0,0,0,-2\n\
                 -1,2,0,0\n\
                 -2,-2,-2,2\n\
                 3,0,2,-1\n\
                 -1,3,2,2\n\
                 -1,0,-1,0\n\
                 0,2,1,-2\n\
                 3,0,0,0";


    #[test]
    fn test_first_half() {
        test_constellation_count(TEST_INPUT_0, 2);
        test_constellation_count(TEST_INPUT_1, 4);
    }

    fn test_constellation_count(input: &str, expected: usize) {
        let stars = parse_input(String::from(input));
        let constellations = build_constellations(&stars);
        assert_eq!(constellations.len(), expected);
    }
}

#[derive(Debug, Clone)]
struct Star {
    x: i32,
    y: i32,
    z: i32,
    k: i32,
}

fn distance(a: &Star, b: &Star) -> i32 {
    (a.x-b.x).abs() + (a.y-b.y).abs() +
    (a.z-b.z).abs() + (a.k-b.k).abs()
}

fn parse_input(input: String) -> Vec<Star> {
    let mut stars = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut coords = line.split(",")
                             .map(|c| c.parse::<i32>().unwrap());
        stars.push( Star {x: coords.next().unwrap(),
                          y: coords.next().unwrap(),
                          z: coords.next().unwrap(),
                          k: coords.next().unwrap()})
    }
    stars
}

fn is_same_constellation(a: &Vec<Star>, b: &Vec<Star>) -> bool {
    for sa in a {
        for sb in b {
            if distance(sa, sb) <= 3 {
                return true;
            }
        }
    }
    false
}

fn build_constellations(stars: &Vec<Star>) -> Vec<Vec<Star>> {
    let mut constellations: Vec<Vec<Star>> = stars.iter().map(|s| vec![s.clone()])
                                                 .collect();
    let mut num_c = 0;
    while num_c != constellations.len() {
        num_c = constellations.len();
        for i in 0..constellations.len() {
            for k in 0..i {
                if is_same_constellation(&constellations[i],
                                         &constellations[k]) {
                    let mut appendable = constellations[k].clone();
                    constellations[i].append(&mut appendable);
                    constellations[k].clear();
                }
            }
        }
        constellations.retain(|c| c.len() > 0);
    }
    constellations
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let stars = parse_input(input);
    let constellations = build_constellations(&stars);
    println!("{} constellations", constellations.len());
}
