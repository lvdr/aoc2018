use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
                    .#.#...|#.\n\
                    .....#|##|\n\
                    .|..|...#.\n\
                    ..|#.....#\n\
                    #.#|||#|#|\n\
                    ...#.||...\n\
                    .|....|...\n\
                    ||...#|.#|\n\
                    |.||||..|.\n\
                    ...#.|..|.";

    #[test]
    fn test_both_halves() {
        let input = String::from(TEST_INPUT);
        let mut map = parse_input(input);
        print_state(&map);
        for _ in 0..10 {
            simulate(&mut map, 1, false);
            print_state(&map);
        }
        assert_eq!(count(&map), (37, 31));
    }

    fn print_state(map: &Vec<Vec<Cell>>) {

        for line in map {
            for cell in line {
                let cell = match cell {
                    Cell::Tree => '|',
                    Cell::Lumberyard => '#',
                    Cell::Open => '.',
                };
                print!("{}", cell);
            }
            print!("\n");
        }
        print!("\n");
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Cell {
    Open,
    Tree,
    Lumberyard,
}

fn parse_input(input: String) -> Vec<Vec<Cell>> {
    let mut map = Vec::new(); 
    for line in input.lines() {
        map.push(Vec::new());

        for byte in line.bytes() {
            let next_cell = match byte as char {
                '.' => Cell::Open,
                '|' => Cell::Tree,
                '#' => Cell::Lumberyard,
                _ => panic!("Unknown symbol: {}", byte as char),
            };
            map.last_mut().unwrap()
               .push(next_cell);
        }
    }
    map
}

fn simulate_line(map: &Vec<Vec<Cell>>, line: usize) -> Vec<Cell> {
    let width = map[0].len();
    let height = map.len();
    let mut sums = Vec::new();

    let min = if line == 0 { 0 } else { line - 1 };
    let max = if line == height-1 { line } else { line+1 };
    for x in 0..width {
        let mut n_yards = 0;
        let mut n_trees = 0;
        for y in min..max+1 {
            let cell = map[y][x];
            if      cell == Cell::Tree       { n_trees += 1; }
            else if cell == Cell::Lumberyard { n_yards += 1; }
        }
        sums.push((n_trees, n_yards));
    }

    let mut result = Vec::new();
    let mut trees = sums[0].0;
    let mut yards = sums[0].1;
    for x in 0..width {
        if x >= 2 {
            trees -= sums[x-2].0;
            yards -= sums[x-2].1;
        }
        if x != width-1 {
            trees += sums[x+1].0;
            yards += sums[x+1].1;
        }

        let next_cell = match map[line][x] {
            Cell::Open       => if trees >= 3 {Cell::Tree}
                                else {Cell::Open},
            Cell::Tree       => if yards >= 3 {Cell::Lumberyard}
                                else {Cell::Tree},
            Cell::Lumberyard => if yards >= 2 && trees >= 1 {Cell::Lumberyard}
                                else {Cell::Open},
        };
        result.push(next_cell);
    }
    result
}

fn simulate(map: &mut Vec<Vec<Cell>>, steps: u64, interpolate: bool) {
    let mut states = HashMap::new();
    let mut remaining = 0;
    for s in 0..steps {
        let mut buffer = simulate_line(map, 0);
        for i in 1..map.len() {
            let new_line = simulate_line(map, i);
            map[i-1] = buffer;
            buffer = new_line;
        }
        let end = map.len()-1;
        map[end] = buffer;

        if interpolate {
            let mut hasher = DefaultHasher::new();
            map.hash(&mut hasher);
            let state = hasher.finish();
            if states.contains_key(&state) {
                let period = s - states[&state];
                remaining = (steps - s - 1) % period;
                break;
            } else {
                states.insert(state, s);
            }
        }
    }

    if interpolate && remaining > 0 {
        simulate(map, remaining, false);
    }
}

fn count(map: &Vec<Vec<Cell>>) -> (u32, u32) {
    let mut trees = 0;
    let mut yards = 0;
    for line in map {
        for cell in line {
            match cell {
                Cell::Tree => trees += 1,
                Cell::Lumberyard => yards += 1,
                _ => (),
            }
        }
    }
    (trees, yards)
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let mut map = parse_input(input);
    simulate(&mut map, 10, false);
    let (trees, yards) = count(&map);
    println!("10: Trees: {}, yards: {}, value: {}",
             trees, yards, trees*yards);

    simulate(&mut map, 1000000000u64-10, true);
    let (trees, yards) = count(&map);
    println!("1e9: Trees: {}, yards: {}, value: {}",
             trees, yards, trees*yards);

}
