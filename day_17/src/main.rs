use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::cmp;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "x=495, y=2..7\n\
                              y=7, x=495..501\n\
                              x=501, y=3..7\n\
                              x=498, y=2..4\n\
                              x=506, y=1..2\n\
                              x=498, y=10..13\n\
                              x=504, y=10..13\n\
                              y=13, x=498..504";

    #[test]
    fn test_flood() {
        let input = String::from(TEST_INPUT);
        let (mut tiles, y_range) = parse_input(input);
        add_spring(&mut tiles, (500, 0), y_range.1);
        assert_eq!(count_water(&tiles, y_range), 57);
    }


    const TEST_INPUT_2: &str = "y=7, x=499..501\n\
                                y=10, x=495..505\n\
                                x=495, y=6..9\n\
                                x=505, y=6..9\n\
                                x=506, y=1..2\n";

    #[test]
    fn test_divergence() {
        let input = String::from(TEST_INPUT_2);
        let (mut tiles, y_range) = parse_input(input);
        add_spring(&mut tiles, (500, 0), y_range.1);
        print_state(&tiles, y_range);
        assert_eq!(count_water(&tiles, y_range), 60);
    }


    const TEST_INPUT_3: &str = "y=7, x=500..500\n\
                                y=7, x=498..498\n\
                                y=8, x=498..500\n\
                                y=10, x=495..505\n\
                                x=495, y=6..9\n\
                                x=505, y=6..9\n\
                                x=506, y=1..2\n";

    #[test]
    fn test_hit_edge() {
        let input = String::from(TEST_INPUT_3);
        let (mut tiles, y_range) = parse_input(input);
        add_spring(&mut tiles, (500, 0), y_range.1);
        print_state(&tiles, y_range);
        assert_eq!(count_water(&tiles, y_range), 58);
    }
}

type Pos = (u32, u32);

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum Tile {
    Clay,
    RunningWater,
    StillWater,
}

fn parse_input(input: String) -> (HashMap<Pos, Tile>, (u32, u32)) {
    let mut tiles = HashMap::new();
    let mut max_y = 0;
    let mut min_y = std::u32::MAX;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut halves = line.split(", ");
        let flip = line.find("y=") == Some(0);

        let left = halves.next().unwrap();
        let right = halves.next().unwrap();


        let left = left[2..].parse::<u32>().unwrap();
        let mut right = right[2..].split("..")
                                  .map(|r| r.parse::<u32>().unwrap());
        let r_low = right.next().unwrap();
        let r_high = right.next().unwrap() + 1;
        for right in r_low..r_high {
            if flip {
                tiles.insert((right, left), Tile::Clay);
                max_y = cmp::max(max_y, left);
                min_y = cmp::min(min_y, left);
            } else {
                tiles.insert((left, right), Tile::Clay);
                max_y = cmp::max(max_y, right);
                min_y = cmp::min(min_y, right);
            }
        }
    }
    (tiles, (min_y, max_y))
}

fn count_water(tiles: &HashMap<Pos, Tile>, y_range: (u32, u32)) -> u32 {
    let mut count = 0;
    for (pos, tile) in tiles {
        if pos.1 >= y_range.0 && pos.1 <= y_range.1 &&
           *tile != Tile::Clay {
            count += 1;
        }
    }
    count
}

fn fill_basin(tiles: &mut HashMap<Pos, Tile>, pos: (u32, u32), y_max: u32) {
    let mut flood_point = pos;
    let mut top = false;
    while !top {
        let x_max;
        let mut inspect = flood_point;
        loop {
            let down = (inspect.0, inspect.1 + 1);
            let right = (inspect.0 + 1, inspect.1);
            if !tiles.contains_key(&down) {
                add_spring(tiles, inspect, y_max);
                if !tiles.contains_key(&right) {
                    top = true;
                    x_max = inspect.0;
                    break;
                }
            }
            if tiles.contains_key(&right) {
                if tiles[&right] == Tile::Clay {
                    x_max = inspect.0;
                    break;
                }
                if tiles[&down] == Tile::Clay {
                    top = true;
                    x_max = inspect.0;
                    break;
                }
            }
            inspect.0 += 1;
        }

        let x_min;
        let mut inspect = flood_point;
        loop {
            let down = (inspect.0, inspect.1 + 1);
            let left = (inspect.0 - 1, inspect.1);
            if !tiles.contains_key(&down) {
                add_spring(tiles, inspect, y_max);
                if !tiles.contains_key(&left) {
                    top = true;
                    x_min = inspect.0;
                    break;
                }
            }

            if tiles.contains_key(&left) {
                if tiles[&left] == Tile::Clay {
                    x_min = inspect.0;
                    break;
                }
                if tiles[&down] == Tile::Clay {
                    top = true;
                    x_min = inspect.0;
                    break;
                }
            }
            inspect.0 -= 1;
        }

        for x in x_min..x_max+1 {
            let pos = (x, flood_point.1);
            let tile = if top { Tile::RunningWater } 
                       else { Tile:: StillWater };
            tiles.insert(pos, tile);
        }

        flood_point.1 -= 1;
    }
}

fn add_spring(tiles: &mut HashMap<Pos, Tile>, pos: (u32, u32), y_max: u32) {
    let mut pos = pos;
    loop {
        tiles.insert(pos, Tile::RunningWater);
        let next_step = (pos.0, pos.1+1);
        if tiles.contains_key(&next_step) {
            if tiles[&next_step] != Tile::RunningWater {
                fill_basin(tiles, pos, y_max);

            }
            return;
        }

        pos = next_step;
        if pos.1 > y_max {
            return;
        }
    }
}

fn print_state(tiles: &HashMap<Pos, Tile>, range: (u32, u32)) {
    for y in range.0..range.1 + 1 {
        for x in 450..551 {
            if tiles.contains_key(&(x,y)) {
                match tiles[&(x,y)] {
                    Tile::Clay => print!("#"),
                    Tile::RunningWater => print!("|"),
                    Tile::StillWater => print!("~"),
                }
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let (mut tiles, y_range) = parse_input(input);

    //print_state(&tiles, y_range);
    add_spring(&mut tiles, (500, 0), y_range.1);
    let water = count_water(&tiles, y_range);
    print_state(&tiles, y_range);
    println!("{} water tiles", water);
}
