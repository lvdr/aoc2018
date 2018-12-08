use std::io::prelude::*;
use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both_halves() {
        let input = String::from("#1 @ 1,3: 4x4\n\
                                  #2 @ 3,1: 4x4\n\
                                  #3 @ 5,5: 2x2");
        const ARR_SIZE: usize = 10;

        let mut dat = vec![ vec![0; ARR_SIZE]; ARR_SIZE];
        let claims = parse_input(input);
        for claim in &claims {
            mark_claim(*claim, &mut dat);
        }

        assert_eq!(integrate(&mut dat), 4);
        assert_eq!(check_claims(&dat, &claims).unwrap(), 3);
    }
}

#[derive(Copy, Clone)]
struct Claim {
    id: u32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn mark_claim(c: Claim,
              dat: &mut Vec<Vec<i32>>) {
    // Mark corners of each claim, so that when the array is
    // integrated each claim contributes a 1 in its area, additively
    dat[c.x][c.y] += 1;
    dat[c.x+c.width][c.y] += -1;
    dat[c.x][c.y+c.height] += -1;
    dat[c.x+c.width][c.y+c.height] += 1;
}

fn integrate(dat: &mut Vec<Vec<i32>>) -> i32 {
    for x in 1..dat.len() {
        for y in 0..dat[x].len() {
            dat[x][y] += dat[x-1][y];
        }
    }
    let mut total_overlaps = 0;
    for y in 0..dat[0].len() {
        for x in 0..dat.len() {
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

fn check_claims(dat: &Vec<Vec<i32>>, claims: &Vec<Claim>) -> Option<u32> {
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
            return Some(claim.id);
        }
    }
    None
}

fn parse_input(input: String) -> Vec<Claim> {
    let mut claims = Vec::new();
    let lines : Vec<&str> = input.trim()
                                .split("\n")
                                .map(|x| x.trim())
                                .collect();

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
    }
    claims
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    const ARR_SIZE: usize = 1001;

    let mut dat = vec![ vec![0; ARR_SIZE]; ARR_SIZE];
    let claims = parse_input(input);
    for claim in &claims {
        mark_claim(*claim, &mut dat);
    }

    println!("Overlaps: {}", integrate(&mut dat));

    println!("Found non-overlaping claim {}",
             check_claims(&dat, &claims).unwrap());
}
