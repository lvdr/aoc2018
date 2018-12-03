use std::io::prelude::*;
use std::fs::File;

const ARR_SIZE: usize = 1001;

#[derive(Copy, Clone)]
struct Claim {
    id: u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn mark_claim(c: Claim,
              dat: &mut [[i32; ARR_SIZE]; ARR_SIZE]) {
    // Mark corners of each claim, so that when the array is
    // integrated each claim contributes a 1 in its area, additively
    dat[c.x][c.y] += 1;
    dat[c.x+c.width][c.y] += -1;
    dat[c.x][c.y+c.height] += -1;
    dat[c.x+c.width][c.y+c.height] += 1;
}

fn integrate(dat: &mut [[i32; ARR_SIZE]; ARR_SIZE]) -> i32 {
    for x in 1..ARR_SIZE {
        for y in 0..ARR_SIZE {
            dat[x][y] += dat[x-1][y];
        }
    }
    let mut total_overlaps = 0;
    for y in 0..ARR_SIZE {
        for x in 0..ARR_SIZE {
            if y != 0 {
                dat[x][y] += dat[x][y-1];
            }
            if dat[x][y] > 1 {
                total_overlaps += 1;
            }
        }
    }
    total_overlaps
}

fn check_claims(dat: &[[i32; ARR_SIZE]; ARR_SIZE], claims: &Vec<Claim>) {
    for claim in claims {
        let mut failed = 0;
        for x in 0..claim.width {
            for y in 0..claim.height {
                if dat[claim.x + x][claim.y+y] != 1 {
                    failed = 1;
                }
            }
        }
        if failed == 0 {
            println!("Claim {} doesn't overlap with any other!", claim.id);
        }
    }
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let mut dat = [[0; ARR_SIZE]; ARR_SIZE];

    let lines : Vec<&str> = input.trim()
                                .split("\n")
                                .map(|x| x.trim())
                                .collect();

    let mut claims : Vec<Claim> = Vec::new();

    for line in lines {
        let mut vals = line.split("@");
       
        let id = vals.next().unwrap()
                     .split("#").nth(1)
                     .unwrap().trim()
                     .parse::<u32>().unwrap();
        let mut detail = vals.next().unwrap().split(":");
        let corners: Vec<usize> = detail.next().unwrap()
                          .split(",")
                          .map(|x| x.trim().parse::<usize>().unwrap())
                          .collect();
        let size: Vec<usize> = detail.next().unwrap()
                              .split("x")
                              .map(|x| x.trim().parse::<usize>().unwrap())
                              .collect();
        let claim: Claim = Claim {id: id, x: corners[0], y: corners[1],
                                  width: size[0], height: size[1]};
        claims.push(claim);
        mark_claim(claim, &mut dat);
    }

    println!("Overlaps: {}", integrate(&mut dat));

    check_claims(&dat, &claims);
}
