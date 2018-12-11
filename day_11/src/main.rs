use std::cmp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
	    let mut grid = create_grid(18);
	    integrate(&mut grid);
	    assert_eq!(find_max_at_size(&grid, 3), (29, 33, 45));
	    assert_eq!(find_max(&grid), (90, 269, 16));


	    let mut grid = create_grid(42);
	    integrate(&mut grid);
	    assert_eq!(find_max_at_size(&grid, 3), (30, 21, 61));
	    assert_eq!(find_max(&grid), (232, 251, 12));
	}

	#[test]
	fn test_power_algo() {
	    assert_eq!(cell_power(122, 79, 57), -5);
	    assert_eq!(cell_power(217, 196, 39), 0);
	    assert_eq!(cell_power(101, 153, 71), 4);
	}

	#[test]
	fn test_integrate() {
	    let mut grid = vec![vec![1; 300]; 300];
	    integrate(&mut grid);

		for y in 0..300 {
			for x in 0..300 {
				assert_eq!(grid[y][x], ((y+1)*(x+1)) as i32);
			}
		}

	}
}

fn cell_power(x: i32, y: i32, serial: i32) -> i32 {
	let rack_id = x + 10;
	let mut power = rack_id * y + serial;
	power = power * rack_id;
	power = (power % 1000) / 100;
	power - 5
}

fn create_grid(serial: i32) -> Vec<Vec<i32>> {
	let mut grid = vec![vec![0; 300]; 300];
	for y in 0..300 {
		for x in 0..300 {
			grid[y][x] = cell_power(x as i32 + 1, y as i32 + 1, serial);
		}
	}
	grid
}

fn integrate(grid: &mut Vec<Vec<i32>>) {
	// Builds summed-area table out of grid
	for y in 0..300 {
		for x in 0..300 {
			grid[y][x] += match (y == 0, x == 0) {
				(true, true) => 0,
				(true, false) => grid[y][x-1],
				(false, true) => grid[y-1][x],
				(false, false) => grid[y-1][x] + grid[y][x-1] - grid[y-1][x-1],
			}
		}
	}
}

fn sum_of_area(grid: & Vec<Vec<i32>>, x: usize, y: usize, size: usize) -> i32 {
	let size = size-1;
	match (x == 0, y == 0) {
		(true, true) => grid[y+size][x+size],
		(true, false) => grid[y+size][x+size] - grid[y-1][x+size],
		(false, true) => grid[y+size][x+size] - grid[y+size][x-1],
		(false, false) => grid[y+size][x+size] - grid[y+size][x-1]
		                  - grid[y-1][x+size] + grid[y-1][x-1],
	}
}

fn find_max_at_size(grid: &Vec<Vec<i32>>, size: usize) -> (i32, i32, i32) {
	let mut max = (sum_of_area(grid, 0, 0, size), 0, 0);
	for y in 0..300-size+1 {
		for x in 0..300-size+1 {
			max = cmp::max(max, (sum_of_area(grid, x, y, size), x, y));
		}
	}
	(max.0, max.1 as i32 + 1, max.2 as i32 + 1)
}

fn find_max(grid: &Vec<Vec<i32>>) -> (i32, i32, i32) {
	let mut max = (grid[0][0], 0, 0, 1);
	for size in 1..301 {
		let vxy = find_max_at_size(grid, size);
		max = cmp::max(max, (vxy.0, vxy.1, vxy.2, size));
	}
	(max.1, max.2, max.3 as i32)
}

fn main() {
    let mut grid = create_grid(4151);
    integrate(&mut grid);
    println!("post integrate");
    let max = find_max_at_size(&grid, 3);
    println!("3x3 coordinates: {},{}", max.1, max.2);
    let max = find_max(&grid);
    println!("Unrestricted size: size {} at {},{}", max.2, max.0, max.1);
}
