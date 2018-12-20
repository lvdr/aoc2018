use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashSet,VecDeque};

fn pathfind(map_pos: (i32, i32),
            dir_pos: usize, directions: &Vec<u8>,
            map: &mut HashSet<(i32, i32)>,
            starts: &mut HashSet<(i32, i32, usize)>) {
    if !starts.insert((map_pos.0, map_pos.1, dir_pos)) {
        return;
    }

    let mut map_pos = map_pos;
    for i in dir_pos..directions.len() {
        match directions[i] as char {
            'E' => {
                map.insert((map_pos.0+1, map_pos.1));
                map.insert((map_pos.0+2, map_pos.1));
                map_pos = (map_pos.0+2, map_pos.1);
            },
            'S' => {
                map.insert((map_pos.0, map_pos.1+1));
                map.insert((map_pos.0, map_pos.1+2));
                map_pos = (map_pos.0, map_pos.1+2);
            },
            'N' => {
                map.insert((map_pos.0, map_pos.1-1));
                map.insert((map_pos.0, map_pos.1-2));
                map_pos = (map_pos.0, map_pos.1-2);
            },
            'W' => {
                map.insert((map_pos.0-1, map_pos.1));
                map.insert((map_pos.0-2, map_pos.1));
                map_pos = (map_pos.0-2, map_pos.1);
            },
            '^' => continue,
            '(' => {
                let mut parens = 0;
                for k in i+1..directions.len() {
                    match directions[k] as char {
                        '(' => parens += 1,
                        ')' => {
                            if parens == 0 {
                                break;
                            }
                            parens -= 1
                        },
                        '|' => {
                            if parens == 0 {
                                pathfind(map_pos, k+1, directions, map, starts);
                            }
                        },
                        '$' => panic!("Unmatched parens!"),
                        _ => continue,
                    }
                }
            },
            ')' => continue,
            '|' => {
                let mut parens = 1;
                for k in i+1..directions.len() {
                    match directions[k] as char {
                        '(' => parens += 1,
                        ')' => parens -= 1,
                        '$' => panic!("Unmatched parens!"),
                        _ => continue,
                    }
                    if parens == 0 {
                        pathfind(map_pos, k+1, directions, map, starts);
                        return;
                    }
                }
            },
            '$' => return,
            _ => panic!("Unknown symbol: {}", directions[i] as char),
        }
    }
}

fn adjacent(pos: (i32, i32), direction: u32) -> (i32, i32) {
    match direction { // In reading order
        0 => (pos.0, pos.1 - 1),
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0 - 1, pos.1),
        3 => (pos.0 + 1, pos.1),
        _ => panic!("Invalid difference!"),
    }
}

fn find_furthest(map: &HashSet<(i32, i32)>) -> (u32, u32) {
    let mut frontier = VecDeque::new();
    let mut explored = HashSet::new();

    frontier.push_back(((0,0), 0));

    let mut max_distance = 0;
    let mut far_rooms = 0;
    while !frontier.is_empty() {
        let (pos, distance) = frontier.pop_front().unwrap();

        max_distance = std::cmp::max(distance, max_distance);
        if distance >= 1000 {
            far_rooms += 1;
        }

        let distance = distance + 1;
        for i in 0..4 {
            let door = adjacent(pos, i);
            let room = adjacent(door, i);

            if !map.contains(&door) || explored.contains(&room) {
                continue;
            }
            explored.insert(room);
            frontier.push_back( (room, distance) );
        }
    }

    (max_distance, far_rooms)
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");
    let directions = input.into_bytes();

    let mut map = HashSet::new();
    let mut starts = HashSet::new();
    pathfind((0,0), 0, &directions, &mut map, &mut starts);
    let (distance, rooms) = find_furthest(&map);
    println!("Furthest room is {} doors away", distance);
    println!("{} rooms at least 1000 doors away", rooms);

}
