use std::io::prelude::*;
use std::fs::File;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Bot {
    pos: (i32, i32, i32),
    range: u64,
}

fn parse_input(input: String) -> Vec<Bot> {
    let mut bots = Vec::new();
    for line in input.lines() {
        let mut split = line[5..].split(">, r=");
        let mut coords = split.next().unwrap()
                              .split(",")
                              .map(|c| c.parse::<i32>().unwrap() );
        let mut radius = split.next().unwrap()
                              .parse::<u64>().unwrap();
        bots.push(Bot { pos: (coords.next().unwrap(),
                              coords.next().unwrap(),
                              coords.next().unwrap()),
                        range: radius });
    }

    bots
}

fn distance(a: (i32, i32, i32), b: (i32, i32, i32)) -> u64 {
    (a.0 - b.0).abs() as u64 + (a.1 - b.1).abs() as u64 + (a.2 - b.2).abs() as u64
}

fn find_largest_range_bots(bots: &Vec<Bot>) -> usize {
    let max_bot = bots.iter().max_by_key(|b| b.range).unwrap();
    bots.iter().filter(|b| distance(b.pos, max_bot.pos) <= max_bot.range)
               .count()
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Box {
 x_min: i32,
 x_max: i32,
 y_min: i32,
 y_max: i32,
 z_min: i32,
 z_max: i32,
}

fn midpoint(bb: Box) -> (i32, i32, i32) {
    (((bb.x_max as i64 + bb.x_min as i64)/2) as i32,
     ((bb.y_max as i64 + bb.y_min as i64)/2) as i32,
     ((bb.z_max as i64 + bb.z_min as i64)/2) as i32)
}

fn max_in_range(bots: &Vec<Bot>, bb: Box) -> (usize, usize) {
    // Returns how many points, at most, can be in range
    // of any point in a given range according to a heuristic
    let point = midpoint(bb);
    // Add 1 to round up
    let max_distance = (bb.x_max as i64 - bb.x_min as i64 + 1)/2 +
                       (bb.x_max as i64 - bb.x_min as i64 + 1)/2 +
                       (bb.x_max as i64 - bb.x_min as i64 + 1)/2;
    let max_distance = max_distance as u64;

    let high = bots.iter()
                   .filter(|b| distance(b.pos, point) <= b.range + max_distance)
                   .count();
    let low = bots.iter()
                  .filter(|b| distance(b.pos, point) <= b.range)
                  .count();
    (low, high)
}

fn subvolume(b: &Box, section: u32) -> Box {
    let mut subvol = b.clone();
    if section & 1 != 0 {
        subvol.x_max = (subvol.x_max + subvol.x_min)/2;
    } else {
        subvol.x_min = (subvol.x_max + subvol.x_min)/2;
    }
    if section & 2 != 0 {
        subvol.y_max = (subvol.y_max + subvol.y_min)/2;
    } else {
        subvol.y_min = (subvol.y_max + subvol.y_min)/2;
    }
    if section & 4 != 0 {
        subvol.z_max = (subvol.z_max + subvol.z_min)/2;
    } else {
        subvol.z_min = (subvol.z_max + subvol.z_min)/2;
    }
    subvol
}

fn find_safest_spot(bots: &Vec<Bot>) -> (i32, i32, i32) {
    let start_vol   = Box {x_min: std::i32::MIN,
                           x_max: std::i32::MAX,
                           y_min: std::i32::MIN,
                           y_max: std::i32::MAX,
                           z_min: std::i32::MIN,
                           z_max: std::i32::MAX};

    let mut ranges = BinaryHeap::new();
    ranges.push((0, 0, start_vol));

    let mut best_point = (0, 0, 0);
    let mut best_distance = 0;
    let mut best_score = 0;
    let mut max_heur = 0;
    while !ranges.is_empty() {
        let (high, _low, next) = ranges.pop().unwrap();
        if high < best_score {
            break;
        }
        for i in 0..8 {
            let subvol = subvolume(&next, i);
            let (low, high) = max_in_range(bots, subvol);
            max_heur = std::cmp::max(low, max_heur);
            if is_point(subvol) {
                let point = (subvol.x_min, subvol.y_min, subvol.z_min);
                let distance = distance(point, (0,0,0));
                if best_score < low ||
                   (best_score == low &&
                   distance < best_distance) {
                    println!("Got to point {},{},{} ({})", point.0, point.1, point.2, distance);
                    best_score = low;
                    best_point = point;
                    best_distance = distance;
                }
            } else if high >= max_heur {
                ranges.push((high, low, subvol));
            }
        }
    }
    best_point
}

fn is_point(b: Box) -> bool {
    b.x_max == b.x_min &&
    b.y_max == b.y_min &&
    b.z_max == b.z_min
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let bots = parse_input(input);
    println!("{} bots found in range", find_largest_range_bots(&bots));
    let spot = find_safest_spot(&bots);
    println!("Safest spot at {},{},{}", spot.0, spot.1, spot.2);
    println!("Manhattan distance from origin: {}",
             spot.0.abs() + spot.1.abs() + spot.2.abs());
}
