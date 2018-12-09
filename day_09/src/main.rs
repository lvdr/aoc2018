use std::collections::VecDeque;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(high_score(9, 25), 32);
        assert_eq!(high_score(10, 1618), 8317);
        assert_eq!(high_score(13, 7999), 146373);
        assert_eq!(high_score(17, 1104), 2764);
        assert_eq!(high_score(21, 6111), 54718);
        assert_eq!(high_score(30, 5807), 37305);
    }
}

fn rotate_ccw(ring: &mut VecDeque<u32>, positions: u32) {
    for _ in 0..positions {
        let val = ring.pop_back().unwrap();
        ring.push_front(val);
    }
}

fn rotate_cw(ring: &mut VecDeque<u32>, positions: u32) {
    for _ in 0..positions {
        let val = ring.pop_front().unwrap();
        ring.push_back(val);
    }
}

fn play_marble(ring: &mut VecDeque<u32>, value: u32) -> u32 {
    if value % 23 == 0 {
        rotate_ccw(ring, 7);
        let score = value + ring.pop_back().unwrap();
        rotate_cw(ring, 1);
        score
    } else {
        rotate_cw(ring, 1);
        ring.push_back(value);
        0
    }
}

fn high_score(num_players: u32, last_marble: u32) -> u32 {
    let mut scores = vec![0; num_players as usize];
    let mut ring = VecDeque::new();
    ring.push_back(0);

    let mut current_player = 0;
    for i in 1..last_marble+1 {

        scores[current_player] += play_marble(&mut ring, i);
        current_player = (current_player+1) % num_players as usize;
    }
    *scores.iter().max().unwrap()
}

fn main() {
    println!("In the reasonable case, high score is {}", high_score(424, 71482));
    println!("In the unreasonable case, high score is {}", high_score(424, 71482*100));
}
