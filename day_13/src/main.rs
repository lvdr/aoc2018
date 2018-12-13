use std::io::prelude::*;
use std::fs::File;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "/->-\\        \n\
                              |   |  /----\\\n\
                              | /-+--+-\\  |\n\
                              | | |  | v  |\n\
                              \\-+-/  \\-+--/\n\
                              - \\------/   \n\
                              ";

    #[test]
    fn test_both_halves() {
        let input = String::from(TEST_INPUT);
        let (network, mut carts, mut cart_pos) = parse_input(input);
        let first_collision = simulate(&network, &mut carts, &mut cart_pos);
        assert_eq!(first_collision, (7, 3));
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Eq, Copy, Clone)]
struct Cart {
    coordinate: (usize, usize),
    direction: Direction,
    tick: u32,
    next_turn: Direction,
    id: u32,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        // Reverse order; to make min heap
        other.tick.cmp(&self.tick)
             .then(other.coordinate.cmp(&self.coordinate))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.tick == other.tick &&
            self.coordinate == other.coordinate &&
            self.direction == other.direction
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum TurnDir {
    TwoEight, // '/'
    FourTen,  // '\'
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Rail {
    Empty,
    Horizontal,
    Vertical,
    Intersection,
    Turn(TurnDir),
}

fn parse_input(input: String)
        -> (Vec<Vec<Rail>>, BinaryHeap<Cart>, HashSet<(usize, usize)>) {

    let mut carts: Vec<Cart> = Vec::new();
    let mut network = Vec::new(); 

    let mut y = 0; 
    for line in input.lines() {
        network.push(Vec::new());

        let mut x = 0;
        let mut num_carts = 0;
        for byte in line.bytes() {
            let next_bit = match byte as char {
                ' ' => Rail::Empty,
                '-' => Rail::Horizontal,
                '|' => Rail::Vertical,
                '+' => Rail::Intersection,
                '/' => Rail::Turn(TurnDir::TwoEight),
                '\\' => Rail::Turn(TurnDir::FourTen),

                '>' => {carts.push(Cart { coordinate: (x,y),
                                          direction: Direction::Right,
                                          next_turn: Direction::Left,
                                          tick: 0, id: num_carts});
                        num_carts += 1;
                        Rail::Horizontal}
                '<' => {carts.push(Cart { coordinate: (x,y),
                                          direction: Direction::Left,
                                          next_turn: Direction::Left,
                                          tick: 0, id: num_carts});
                        num_carts += 1;
                        Rail::Horizontal}
                '^' => {carts.push(Cart { coordinate: (x,y),
                                          direction: Direction::Up,
                                          next_turn: Direction::Left,
                                          tick: 0, id: num_carts});
                        num_carts += 1;
                        Rail::Vertical}
                'v' => {carts.push(Cart { coordinate: (x,y),
                                          direction: Direction::Down,
                                          next_turn: Direction::Left,
                                          tick: 0, id: num_carts});
                        num_carts += 1;
                        Rail::Vertical}
                _ => panic!("Unknown symbol: {}", byte as char),
            };
            network.last_mut().unwrap()
                   .push(next_bit);
            x += 1;
        }
        y += 1;
    }

    let mut cart_heap = BinaryHeap::new();
    let mut cart_pos = HashSet::new();
    for cart in carts {
        cart_pos.insert(cart.coordinate);
        cart_heap.push(cart);
    }

    (network, cart_heap, cart_pos)
}

fn turn(old_dir: Direction, turn: &TurnDir) -> Direction {
    match (turn, old_dir) {
        (TurnDir::TwoEight, Direction::Right) => Direction::Up,
        (TurnDir::TwoEight, Direction::Up) => Direction::Right,
        (TurnDir::TwoEight, Direction::Left) => Direction::Down,
        (TurnDir::TwoEight, Direction::Down) => Direction::Left,
        (TurnDir::FourTen, Direction::Left) => Direction::Up,
        (TurnDir::FourTen, Direction::Up) => Direction::Left,
        (TurnDir::FourTen, Direction::Right) => Direction::Down,
        (TurnDir::FourTen, Direction::Down) => Direction::Right,
    }
}

fn turn_left(old_dir: Direction) -> Direction {
    match old_dir {
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
        Direction::Up => Direction::Left,
    }
}


fn turn_right(old_dir: Direction) -> Direction {
    match old_dir {
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
    }
}

fn intersect(cart: Cart) -> Direction {
    let old_dir = cart.direction;
    let next_turn = cart.next_turn;
    match next_turn {
        Direction::Left => turn_left(old_dir),
        Direction::Up => old_dir,
        Direction::Right => turn_right(old_dir),
        _ => panic!("Next turn can't be backwards (Down)"),
    }
}

fn advance(next_turn: Direction) -> Direction {
    match  next_turn {
        Direction::Left => Direction::Up,
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Left,
        _ => panic!("Next turn can't be backwards (Down)"),
    }
}

fn simulate(network: &Vec<Vec<Rail>>,
            carts: &mut BinaryHeap<Cart>,
            cart_pos: &mut HashSet<(usize, usize)>) -> (usize, usize) {
    loop {
        let next_cart = carts.pop().unwrap();

        if cart_pos.contains(&next_cart.coordinate) {
            cart_pos.remove(&next_cart.coordinate);
        } else {
            continue;
        }

        let new_pos = match next_cart.direction {
            Direction::Right => (next_cart.coordinate.0+1,
                                 next_cart.coordinate.1),
            Direction::Left  => (next_cart.coordinate.0-1,
                                 next_cart.coordinate.1),
            Direction::Up    => (next_cart.coordinate.0,
                                 next_cart.coordinate.1-1),
            Direction::Down  => (next_cart.coordinate.0,
                                 next_cart.coordinate.1+1),
        };

        if cart_pos.contains(&new_pos) {
            cart_pos.remove(&new_pos);
            return new_pos;
        }

        let mut next_next_turn = next_cart.next_turn;
        let new_dir = match network[new_pos.1][new_pos.0] {
            Rail::Empty => panic!("Cart off track!"),
            Rail::Horizontal | Rail::Vertical => next_cart.direction,
            Rail::Intersection => {next_next_turn = advance(next_next_turn);
                             intersect(next_cart)},
            Rail::Turn(ref dir) => turn(next_cart.direction, dir),            
        };

        cart_pos.insert(new_pos);
        carts.push(Cart { coordinate: new_pos, direction: new_dir,
                          tick: next_cart.tick + 1, next_turn: next_next_turn,
                          id: next_cart.id});
    }
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let (network, mut carts, mut cart_pos) = parse_input(input);
    let first_collision = simulate(&network, &mut carts, &mut cart_pos);
    println!("First collision at {},{}", first_collision.0, first_collision.1);

    while cart_pos.len() > 1 {
        simulate(&network, &mut carts, &mut cart_pos);
    }

    let last_cart = *cart_pos.iter().nth(0).unwrap();
    println!("Last cart at {},{}", last_cart.0,
                                   last_cart.1);
}
