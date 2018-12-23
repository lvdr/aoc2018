use std::collections::{BinaryHeap, HashSet};
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        let erosion = build_erosion_level((10, 10), 510, false);
        print_cave(&erosion);
        assert_eq!(calc_danger(&erosion), 114);
    }

    #[test]
    fn test_second_half() {
        let erosion = build_erosion_level((10, 10), 510, true);
        print_cave(&erosion);
        assert_eq!(find_path((10, 10), &erosion), 45);
    }

    fn print_cave(erosion: &Vec<Vec<usize>>) {
        for line in erosion {
            for tile in line {
                let tile = match tile % 3 {
                     0 => '.',
                     1 => '=',
                     2 => '|',
                     _ => panic!("How did you even get here?"),
                 };
                 print!("{}", tile);
            }
            print!("\n");
        }

    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Equipment {
    Torch, Gear, Neither,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    distance: usize,
    equipment: Equipment,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.distance.cmp(&self.distance)
            .then_with(|| (self.equipment as u32).cmp(&(other.equipment as u32)))
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_erosion_level(range: (usize, usize), depth: usize, extend: bool)
    -> Vec<Vec<usize>> {

    let mut erosion = Vec::new();
    let y_max = if extend { range.1*10 } else { range.1+1 };
    let x_max = if extend { range.0*10 } else { range.1+1 };
    for y in 0..y_max {
        erosion.push(Vec::new());
        for x in 0..x_max {
            let idx =
                if x == range.0 && y == range.1 { 0 }
                else if x == 0 { y*48271 }
                else if y == 0 { x*16807 }
                else { erosion[y-1][x] * erosion[y][x-1] };
            erosion[y].push((idx + depth) % 20183);
        }
    }

    erosion
}

fn calc_danger(erosion: &Vec<Vec<usize>>) -> usize {
    let mut danger = 0;
    for line in erosion {
        for tile in line {
            danger += tile % 3;
        }
    }

    danger
}

fn neighbours(state: State) -> Vec<State> {
    let mut nbs = Vec::new();

    // Direct movement
    if state.position.0 > 0 {
        nbs.push( State { position: (state.position.0-1,
                                     state.position.1),
                          equipment: state.equipment,
                          distance: state.distance + 1});
    }

    if state.position.1 > 0 {
        nbs.push( State { position: (state.position.0,
                                     state.position.1-1),
                          equipment: state.equipment,
                          distance: state.distance + 1});
    }


    nbs.push( State { position: (state.position.0+1,
                                 state.position.1),
                      equipment: state.equipment,
                      distance: state.distance + 1});


    nbs.push( State { position: (state.position.0,
                                 state.position.1+1),
                      equipment: state.equipment,
                      distance: state.distance + 1});

    // Change equipment:
    for new_quip in [Equipment::Torch,
                     Equipment::Gear,
                     Equipment::Neither].iter() {
        if state.equipment != *new_quip {
            nbs.push( State { position: state.position,
                              equipment: *new_quip,
                              distance: state.distance + 7});
        }
    }
    nbs
}

fn is_valid_move(state: State, erosion: &Vec<Vec<usize>>) -> bool {
    let x = state.position.0;
    let y = state.position.1;
    if erosion.len() <= y || erosion[0].len() <= x {
        return false;
    }
    match erosion[y][x] % 3 {
        0 => state.equipment != Equipment::Neither, // rocky
        1 => state.equipment != Equipment::Torch, // wet
        2 => state.equipment != Equipment::Gear, // narrow
        _ => panic!("What is even happening?"),
    }
}

fn find_path(target: (usize, usize), erosion: &Vec<Vec<usize>>) -> usize {
    // Dijkstra's, again
    let mut frontier = BinaryHeap::new();
    let mut explored = HashSet::new();
    let mut front_set = HashSet::new();
    let initial = State { distance: 0,
                          equipment: Equipment::Torch,
                          position: (0, 0) };
    frontier.push(initial);
    front_set.insert(initial);

    while !frontier.is_empty() {
        let next = frontier.pop().unwrap();

        if next.position == target && next.equipment == Equipment::Torch {
            return next.distance;
        }

        explored.insert((next.equipment, next.position));
        for nb in neighbours(next) {
            if !is_valid_move(nb, erosion) {
                continue;
            }
            if explored.contains(&(nb.equipment, nb.position)) {
                continue;
            }

            if front_set.insert(nb) {
                frontier.push(nb);
            }
        }
    }

    panic!("No path found to target (?!)");
}

fn main() {
    let erosion = build_erosion_level((14,785), 4080, false);
    println!("Dangerating: {}", calc_danger(&erosion));

    let erosion = build_erosion_level((14,785), 4080, true);
    let time = find_path((14, 785), &erosion);
    println!("{} minutes to reach target", time);

}
