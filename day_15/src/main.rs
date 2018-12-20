use std::io::prelude::*;
use std::fs::File;
use std::sync::atomic::{Ordering, AtomicUsize};
use std::collections::{HashMap, HashSet, VecDeque};


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = 
                "#######\n\
                 #.G...#\n\
                 #...EG#\n\
                 #.#.#G#\n\
                 #..G#E#\n\
                 #.....#\n\
                 #######";

    const TEST_INPUT_2: &str = 
                "#######\n\
                 #G..#E#\n\
                 #E#E.E#\n\
                 #G.##.#\n\
                 #...#E#\n\
                 #...E.#\n\
                 #######";


    #[test]
    fn test_first_half() {
        //test_resolution(TEST_INPUT_1, (47, 590));
        test_resolution(TEST_INPUT_2, (37, 982));
    }

    fn test_resolution(input: &str, expected: (u32, i32)) {
        let input = String::from(input);
        let (walls, mut actors) = parse_input(input);
        assert_eq!(run(&walls, &mut actors), expected);
    }

    #[test]
    fn test_pos_comparison() {
        assert!(GridPos {y: 0, x: 0} < GridPos {y: 0, x: 1});
        assert!(GridPos {y: 0, x: 0} < GridPos {y: 1, x: 0});
        assert!(GridPos {y: 0, x: 1} < GridPos {y: 1, x: 0});
    }

}

#[derive(Eq, PartialEq, PartialOrd, Copy, Clone)]
struct Distance {
    val: usize,
}

impl Ord for Distance {
    fn cmp(&self, other: &Distance) -> std::cmp::Ordering {
        // Reverse order to make min heap
        other.val.cmp(&self.val)
    }

}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
struct GridPos {
    y: usize,
    x: usize,
}

impl GridPos {
    fn distance(self, other: GridPos) -> usize {
        let x_diff = (self.x as isize - other.x as isize).abs() as usize;
        let y_diff = (self.y as isize - other.y as isize).abs() as usize;
        x_diff + y_diff
    }

    fn is_adjacent(self, other: GridPos) -> bool {
        self.distance(other) == 1
    }

    fn adjacent(self, direction: u32) -> GridPos {
            match direction { // In reading order
                0 => GridPos {x: self.x, y: self.y - 1},
                1 => GridPos {x: self.x - 1, y: self.y},
                2 => GridPos {x: self.x + 1, y: self.y},
                3 => GridPos {x: self.x, y: self.y + 1},
                _ => panic!("Invalid difference!"),
            }
            
    }

    fn pathfind(self, other: GridPos, walls: &Vec<Vec<bool>>,
                units: &HashSet<GridPos>) -> usize {
        // Dijkstra's since I'm feeling lazy
        let mut frontier = VecDeque::new();
        let mut explored = HashSet::new();

        frontier.push_back((Distance{val: 0}, self));

        while !frontier.is_empty() {
            let (distance, next_step) = frontier.pop_front().unwrap();
            explored.insert(next_step);
            let distance = distance.val + 1;
            for i in 0..4 {
                let adj = next_step.adjacent(i);
                if adj == other {
                    return distance;
                }
                if walls[adj.y][adj.x] ||
                   units.contains(&adj) ||
                   explored.contains(&adj) {
                    continue;
                }
                frontier.push_back( (Distance {val: distance}, adj) );
            }
        }

        return 0;
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Actor {
    uid: usize,
    is_elf: bool,
    hp: i32,
    pos: GridPos,
    turn: u32,
}

impl Actor {
    fn new(is_elf: bool, x: usize, y: usize) -> Actor {
        static NUM_ACTORS: AtomicUsize = AtomicUsize::new(0);
        Actor {uid: NUM_ACTORS.fetch_add(1, Ordering::Relaxed),
               is_elf: is_elf,
               hp: 200,
               pos: GridPos{x: x, y: y},
               turn: 0}
    }
}

fn should_end_combat(actors: &HashMap<usize, Actor>) -> bool {
    let mut found_elf = false;
    let mut found_goblin = false;
    for (_, actor) in actors {
        found_elf |= actor.is_elf;
        found_goblin |= !actor.is_elf;
        if found_goblin && found_elf {
            break;
        }
    }
    !(found_goblin && found_elf)
}

fn next_step(actor: usize, actors: &mut HashMap<usize, Actor>,
             walls: &Vec<Vec<bool>>) -> GridPos {;
    let start: GridPos = actors[&actor].pos;
    let is_elf = actors[&actor].is_elf;
    let mut actor_pos = HashSet::new();

    for (_, actor) in actors.iter() {
        if actor.is_elf != is_elf && start.is_adjacent(actor.pos) {
            return start;
        }
        actor_pos.insert(actor.pos);
    }

    let mut closest = start;
    let mut distance = 0;
    for (_, actor) in actors {
        if actor.is_elf == is_elf {
            continue;
        }
        let pos = actor.pos;

        for i in 0..4 {
            let target = pos.adjacent(i);

            if start == target {
                return start;
            }

            if walls[target.y][target.x] ||
               actor_pos.contains(&target) {
                continue;                
            }

            let path = start.pathfind(target, walls, &actor_pos);

            if path == 0 {
                continue;
            }

            if    distance == 0 || path < distance
               || (path == distance && target < closest) {
                closest = target;
                distance = path;
            }
        }
    }

    if distance == 0 {
        // No accessible targets found
        return start;
    }

    let mut next_step = start;
    let mut distance = 0;
    for i in 0..4 {
        let start = start.adjacent(i);
        if    walls[start.y][start.x]
           || actor_pos.contains(&start) {
            continue;
        }
        if start == closest {
            return start;
        }
        let path = start.pathfind(closest, walls, &actor_pos);

        if path == 0 {
            continue;
        }

        if     distance == 0 || distance > path
            || (path == distance && start < next_step) {
            next_step = start;
            distance = path;
        }
    }

    next_step
}

fn run(walls: &Vec<Vec<bool>>, actors: &mut HashMap<usize, Actor>)
    -> (u32, i32) {
    let mut turn = 0;

    loop {
        let actor;
        {
            let next_actor = actors.iter()
                                   .filter(|(_, a)| a.turn == turn)
                                   .map(|(id, a)| (a.pos, id))
                                   .min();

            if next_actor == None {
                println!("Turn {} done", turn);
                turn += 1;
                continue;
            }

            actor = *next_actor.unwrap().1;
        }

        if should_end_combat(actors) {
            break;
        }

        let new_pos = next_step(actor, actors, walls);
        actors.entry(actor).and_modify(|a| a.pos = new_pos);
        actors.entry(actor).and_modify(|a| a.turn += 1);
        let elf_attacker = actors[&actor].is_elf;

        let target = {
            let target_actor = actors.iter()
                .filter(|(_, a)| new_pos.is_adjacent(a.pos)
                                && (a.is_elf != elf_attacker))
                                   .map(|(id, a)| (a.hp, a.pos, id))
                                   .min();
            match target_actor {
                None => None,
                Some((_, _, id)) => Some(*id),
            }
        };

        if target == None {
            continue;
        }

        let target = target.unwrap();

        actors.entry(target).and_modify(|a| a.hp -= 3);
        if actors[&target].hp <= 0 {
            actors.remove(&target);
        }

    }

    let total_hp = actors.iter().map(|(_, a)| a.hp).sum();
    (turn, total_hp)
}

fn parse_input(input: String) -> (Vec<Vec<bool>>, HashMap<usize, Actor>) {
    let mut walls = Vec::new();
    let mut actors = HashMap::new();

    let mut y = 0; 
    for line in input.lines() {
        walls.push(Vec::new());

        let mut x = 0;
        for byte in line.bytes() {
            let next_tile = match byte as char {
                '#' => true,
                '.' => false,
                'E' | 'G' => {
                    let actor = Actor::new(byte as char == 'E', x, y);
                    actors.insert(actor.uid, actor);
                    false
                },
                _ => panic!("Unknown symbol: {}", byte as char),
            };
            walls.last_mut().unwrap()
                   .push(next_tile);
            x += 1;
        }
        y += 1;
    }
    (walls, actors)
}


fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let (walls, mut actors) = parse_input(input);
    let (turn, total_hp) = run(&walls, &mut actors);
    println!("Ended at turn {}, with a total HP of {}", turn, total_hp);
    println!("Product: {}", turn as i32 *total_hp);
}
