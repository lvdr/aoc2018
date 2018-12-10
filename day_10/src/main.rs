use std::io::prelude::*;
use std::fs::File;
use std::cmp;

struct Point {
	x: isize,
	y: isize,
	vx: isize,
	vy: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
	    let input = String::from("position=<     9,      1> velocity=< 0,  2>\n\
	                              position=<     7,      0> velocity=<-1,  0>\n\
	                              position=<     3,     -2> velocity=<-1,  1>\n\
	                              position=<     6,     10> velocity=<-2, -1>\n\
	                              position=<     2,     -4> velocity=< 2,  2>\n\
	                              position=<    -6,     10> velocity=< 2, -2>\n\
	                              position=<     1,      8> velocity=< 1, -1>\n\
	                              position=<     1,      7> velocity=< 1,  0>\n\
	                              position=<    -3,     11> velocity=< 1, -2>\n\
	                              position=<     7,      6> velocity=<-1, -1>\n\
	                              position=<    -2,      3> velocity=< 1,  0>\n\
	                              position=<    -4,      3> velocity=< 2,  0>\n\
	                              position=<    10,     -3> velocity=<-1,  1>\n\
	                              position=<     5,     11> velocity=< 1, -2>\n\
	                              position=<     4,      7> velocity=< 0, -1>\n\
	                              position=<     8,     -2> velocity=< 0,  1>\n\
	                              position=<    15,      0> velocity=<-2,  0>\n\
	                              position=<     1,      6> velocity=< 1,  0>\n\
	                              position=<     8,      9> velocity=< 0, -1>\n\
	                              position=<     3,      3> velocity=<-1,  1>\n\
	                              position=<     0,      5> velocity=< 0, -1>\n\
	                              position=<    -2,      2> velocity=< 2,  0>\n\
	                              position=<     5,     -2> velocity=< 1,  2>\n\
	                              position=<     1,      4> velocity=< 2,  1>\n\
	                              position=<    -2,      7> velocity=< 2, -2>\n\
	                              position=<     3,      6> velocity=<-1, -1>\n\
	                              position=<     5,      0> velocity=< 1,  0>\n\
	                              position=<    -6,      0> velocity=< 2,  0>\n\
	                              position=<     5,      9> velocity=< 1, -2>\n\
	                              position=<    14,      7> velocity=<-2,  0>\n\
	                              position=<    -3,      6> velocity=< 2, -1>");
	    let mut points = parse_input(input);
	    assert_eq!(find_min_area(&mut points), 3);
	    print_points(&points);
	}

	#[test]
	fn test_simulate_step() {
	    let input = String::from("position=<     9,      1> velocity=< 0,  2>\n\
	                              position=<     7,      0> velocity=<-1,  0>\n\
	                              position=<     3,     -2> velocity=<-1,  1>\n\
	                              position=<     6,     10> velocity=<-2, -1>\n\
	                              position=<     2,     -4> velocity=< 2,  2>\n\
	                              position=<    -6,     10> velocity=< 2, -2>\n\
	                              position=<     1,      8> velocity=< 1, -1>\n\
	                              position=<     1,      7> velocity=< 1,  0>\n\
	                              position=<    -3,     11> velocity=< 1, -2>\n\
	                              position=<     7,      6> velocity=<-1, -1>\n\
	                              position=<    -2,      3> velocity=< 1,  0>\n\
	                              position=<    -4,      3> velocity=< 2,  0>\n\
	                              position=<    10,     -3> velocity=<-1,  1>\n\
	                              position=<     5,     11> velocity=< 1, -2>\n\
	                              position=<     4,      7> velocity=< 0, -1>\n\
	                              position=<     8,     -2> velocity=< 0,  1>\n\
	                              position=<    15,      0> velocity=<-2,  0>\n\
	                              position=<     1,      6> velocity=< 1,  0>\n\
	                              position=<     8,      9> velocity=< 0, -1>\n\
	                              position=<     3,      3> velocity=<-1,  1>\n\
	                              position=<     0,      5> velocity=< 0, -1>\n\
	                              position=<    -2,      2> velocity=< 2,  0>\n\
	                              position=<     5,     -2> velocity=< 1,  2>\n\
	                              position=<     1,      4> velocity=< 2,  1>\n\
	                              position=<    -2,      7> velocity=< 2, -2>\n\
	                              position=<     3,      6> velocity=<-1, -1>\n\
	                              position=<     5,      0> velocity=< 1,  0>\n\
	                              position=<    -6,      0> velocity=< 2,  0>\n\
	                              position=<     5,      9> velocity=< 1, -2>\n\
	                              position=<    14,      7> velocity=<-2,  0>\n\
	                              position=<    -3,      6> velocity=< 2, -1>");
	    let mut points = parse_input(input);
	    let old_entropy = entropy(&points);
	    for steps in vec![1024, 100, 300, 255, 10] {
			simulate(&mut points, steps);
			simulate(&mut points, -steps);
			assert_eq!(old_entropy, entropy(&points));
	    }
	}
}

fn parse_input(input: String) -> Vec<Point> {
	let mut points = Vec::new();
	for line in input.lines() {
		let mut pos = line[10..24].split(",")
		                          .map(|v| v.trim().parse::<isize>().unwrap());
		let mut vel = line[36..42].split(",")
		                          .map(|v| v.trim().parse::<isize>().unwrap());
		let point = Point {x: pos.next().unwrap(), y: pos.next().unwrap(),
		                   vx: vel.next().unwrap(), vy: vel.next().unwrap()};
		points.push(point);
	}
	points
}

fn bounding_box(points: &Vec<Point>) -> (isize, isize, isize, isize) {
	let mut min_x = points[0].x;
	let mut max_x = points[0].x;
	let mut min_y = points[0].y;
	let mut max_y = points[0].y;
	for point in points {
		min_x = cmp::min(min_x, point.x);
		max_x = cmp::max(max_x, point.x);
		min_y = cmp::min(min_y, point.y);
		max_y = cmp::max(max_y, point.y);
	}
	(min_x, max_x, min_y, max_y)
}

fn bb_area(points: &Vec<Point>) -> f64 {
	let (min_x, max_x, min_y, max_y) = bounding_box(points);
	(max_x - min_x) as f64 * (max_y - min_y) as f64
}

fn simulate(points: &mut Vec<Point>, steps: isize) {
	for point in points {
		point.x += point.vx*steps;
		point.y += point.vy*steps;
	}
}

fn find_min_area(points: &mut Vec<Point>) -> isize {
	let mut time = 0;
	let mut step = 1024*4;
	let mut max_reached = false;
	loop {
		time += step;
		simulate(points, step-1);
		let area_back = bb_area(points);
		simulate(points, 2);
		let area_fwd = bb_area(points);
		simulate(points, -1);
		let area_at = bb_area(points);
		match (area_at < area_back, area_at < area_fwd) {
			(true, true) => break, // local minima
			(true, false) => if max_reached { step = -step/2; }, // minima towards the future
			(false, true) => { max_reached = true;  step = -step/2; }, // minima towards the past
			(false, false) => panic!("Reached a local maxima"),
		}
	}
	time
}

fn print_points(points: &Vec<Point>) {
	let (min_x, max_x, min_y, max_y) = bounding_box(points);
	let sky_height = max_y - min_y + 1;
	let sky_width = max_x - min_x + 1;
	let mut sky = Vec::new();
	for _ in 0..sky_height {
		sky.append(&mut vec!['.'; sky_width as usize]);
		sky.push('\n');
	}

	for point in points {
		let x = point.x - min_x;
		let y = point.y - min_y;
		let point = x + (sky_width+1)*y;
		sky[point as usize] = '#';
	}

	let sky: String = sky.into_iter().collect();
	println!("{}", sky);
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let mut points = parse_input(input);
    println!("Stop time: {}", find_min_area(&mut points));
    print_points(&points);
}
