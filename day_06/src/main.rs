use std::io::prelude::*;
use std::fs::File;
use std::cmp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both_halves() {
        let input = String::from("1, 1\n\
                                  1, 6\n\
                                  8, 3\n\
                                  3, 4\n\
                                  5, 5\n\
                                  8, 9");
        let (coords, max_x, max_y) = parse_input(input);
        assert_eq!(largest_voronoi_area(&coords, max_x , max_y), 17);
        assert_eq!(central_area(&coords, max_x, max_y, 32), 16);
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
struct Coord {
    id: usize,
    x: usize,
    y: usize,
}

fn parse_line(line: &str, id: usize) -> Coord {
    let mut xy = line.split(", ");
    let x = xy.next().unwrap()
              .parse::<usize>().unwrap();
    let y = xy.next().unwrap()
              .parse::<usize>().unwrap();
    Coord{ id: id, x: x, y: y }
}

fn distance(a: Coord, b: Coord) -> usize {
    return ((a.x as isize - b.x as isize).abs()
         + (a.y as isize - b.y as isize).abs()) as usize;
}

fn parse_input(input: String) -> (Vec<Coord>, usize, usize) {
    let lines : Vec<&str> = input.trim()
                                 .split("\n")
                                 .map(|x| x.trim())
                                 .collect();

    let mut max_y : usize = 0;
    let mut max_x : usize = 0;
    let mut coords = Vec::new();
    for line in lines {
        let coord = parse_line(line, coords.len()+1);
        max_x = cmp::max(max_x, coord.x+1);
        max_y = cmp::max(max_y, coord.y+1);
        coords.push(coord);
    }
    (coords, max_x, max_y)
}

fn largest_voronoi_area(coords: &Vec<Coord>, max_x: usize,
                        max_y: usize) -> usize {
    let mut grid = vec![vec![0 as usize; max_y]; max_x];
    let mut id_counts = vec![0; coords.len()+1];
    let mut on_edge = vec![0; coords.len()+1];
    for x in 0..max_x {
        for y in 0..max_y {
            let mut xy = Coord { x: x, y: y, id: 0};

            let mut min_distance = max_y+max_x;
            let mut min_id = 0;
            for coord in coords {
                let distance = distance(*coord, xy);
                if min_distance > distance {
                    min_distance = distance;
                    min_id = coord.id;
                }
            }
            grid[x][y] = min_id;

            if x == 0 || y == 0 || x == max_x-1 || y == max_y-1 {
                on_edge[min_id] = 1;
                id_counts[min_id] = 0;
            }
            if on_edge[min_id] != 1 {
                id_counts[min_id] += 1;
            }
        }
    }

    let mut max_count = 0; 
    for id in 0..coords.len()+1 {
        max_count = cmp::max(max_count, id_counts[id]);
    }
    max_count
}

fn central_area(coords: &Vec<Coord>, max_x: usize, max_y: usize,
                max_distance: usize) -> usize {

    let mut central_area = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            let mut total_distance = 0;
            let xy = Coord { x: x, y: y, id: 0};
            for coord in coords {
                total_distance += distance(*coord, xy);
            }
            if total_distance < max_distance {
                let vertical_edge = x == 0 || x == max_x-1;
                let horizontal_edge = y == 0 || y == max_y-1;
                let excess = max_distance - total_distance;
                central_area += match (vertical_edge, horizontal_edge) {
                    // corner, forms a triangle
                    // e.g. xxxx
                    //       xxx
                    //        xx
                    //         x
                    // area = sum(4..1) = 4*(4+1)/2
                    (true, true) => excess*(excess-1)/2,
                    // edge
                    (true, false) | (false, true) => excess,
                    // neither
                    (false, false) => 1,
                }
            }
        }
    }
    central_area
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let (coords, max_x, max_y) = parse_input(input);
    println!("Largest voronoi area: {}",
             largest_voronoi_area(&coords, max_x , max_y));


    println!("central_area: {}", central_area(&coords, max_x, max_y, 10_000));
}
