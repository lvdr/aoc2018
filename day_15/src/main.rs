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

    const TEST_INPUT_3: &str = 
                "#######\n\
                 #E..EG#\n\
                 #.#G.E#\n\
                 #E.##E#\n\
                 #G..#.#\n\
                 #..E#.#\n\
                 #######";

    const TEST_INPUT_4: &str = 
                "#######\n\
                 #E.G#.#\n\
                 #.#G..#\n\
                 #G.#.G#\n\
                 #G..#.#\n\
                 #...E.#\n\
                 #######";

    const TEST_INPUT_5: &str = 
                "#######\n\
                 #.E...#\n\
                 #.#..G#\n\
                 #.###.#\n\
                 #E#G#G#\n\
                 #...#G#\n\
                 #######";

    const TEST_INPUT_6: &str = 
                "#########\n\
                 #G......#\n\
                 #.E.#...#\n\
                 #..##..G#\n\
                 #...##..#\n\
                 #...#...#\n\
                 #.G...G.#\n\
                 #.....G.#\n\
                 #########";

    #[test]
    fn test_first_half() {
        //test_resolution(TEST_INPUT_1, (47, 590));
        //test_resolution(TEST_INPUT_2, (37, 982));
        //test_resolution(TEST_INPUT_3, (46, 859));
        test_resolution(TEST_INPUT_4, (35, 793));
        //test_resolution(TEST_INPUT_5, (54, 536));
        //test_resolution(TEST_INPUT_6, (20, 937));
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
                units: &HashMap<GridPos, bool>) -> GridPos {
        // Dijkstra's since I'm feeling lazy
        let mut frontier = VecDeque::new();
        let mut explored = HashSet::new();

        for i in 0..4 {
            let step = self.adjacent(i);
            if walls[step.y][step.x]
               || units.contains_key(&step) {
                continue;
            }

            if step == other {
                return step;
            }

            frontier.push_back((Distance{val: 0}, step, step));
        }

        if frontier.len() == 1 {
            return frontier.pop_front().unwrap().2;
        }

        while !frontier.is_empty() {
            let (distance, next_step, first_step) = frontier.pop_front().unwrap();
            let distance = distance.val + 1;
            for i in 0..4 {
                let adj = next_step.adjacent(i);
                if adj == other {
                    return first_step;
                }
                if walls[adj.y][adj.x] ||
                   units.contains_key(&adj) ||
                   explored.contains(&adj) {
                    continue;
                }
                explored.insert(adj);
                frontier.push_back( (Distance {val: distance}, adj, first_step) );
            }
        }

        return self;
    }

    fn nearest_enemy(self, is_elf: bool, walls: &Vec<Vec<bool>>,
                     units: &HashMap<GridPos, bool>) -> GridPos {
        // Dijkstra's since I'm feeling lazy
        let mut frontier = VecDeque::new();
        let mut explored = HashSet::new();

        frontier.push_back((Distance{val: 0}, self));

        let mut finish_distance = std::usize::MAX;
        let mut target = self;
        while !frontier.is_empty() {
            let (distance, next_step) = frontier.pop_front().unwrap();

            if distance.val > finish_distance {
                break;
            }

            let distance = distance.val + 1;
            for i in 0..4 {
                let adj = next_step.adjacent(i);

                if units.contains_key(&adj) {
                    if units[&adj] != is_elf &&
                       (distance < finish_distance || adj < target) {
                        target = adj;
                        finish_distance = distance;
                    }
                    continue;
                }

                if walls[adj.y][adj.x] ||
                   explored.contains(&adj) {
                    continue;
                }
                explored.insert(adj);
                frontier.push_back( (Distance {val: distance}, adj) );
            }
        }

        return target;
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
    let mut actor_pos = HashMap::new();

    for (_, actor) in actors.iter() {
        if actor.is_elf != is_elf && start.is_adjacent(actor.pos) {
            return start;
        }
        actor_pos.insert(actor.pos, actor.is_elf);
    }

    let closest = start.nearest_enemy(is_elf, &walls, &actor_pos);

    if closest == start {
        // No accessible targets found
        return start;
    }

    start.pathfind(closest, walls, &actor_pos)
}

fn run(walls: &Vec<Vec<bool>>, actors: &mut HashMap<usize, Actor>)
    -> (u32, i32) {
    let mut turn = 0;

    let mut skip_movement = false;
    let mut last_actors = HashSet::new();
    loop {

        let actor;
        {
            let next_actor = actors.iter()
                                   .filter(|(_, a)| a.turn == turn)
                                   .map(|(id, a)| (a.pos, id))
                                   .min();

            if next_actor == None {
                turn += 1;

                let mut actor_pos = HashSet::new();
                for (_, actor) in actors.iter() {
                    actor_pos.insert( (actor.pos, actor.is_elf) );
                }
                skip_movement = last_actors == actor_pos;
                last_actors = actor_pos;
                continue;
            }

            actor = *next_actor.unwrap().1;
        }

        if should_end_combat(actors) {
            break;
        }

        let new_pos =
            if !skip_movement { next_step(actor, actors, walls) }
            else              { actors[&actor].pos };
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
