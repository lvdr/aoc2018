#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_half() {
        assert_eq!(run_simulation(5, false), 0124515891u64);
        assert_eq!(run_simulation(9, false), 5158916779u64);
        assert_eq!(run_simulation(18, false), 9251071085u64);
        assert_eq!(run_simulation(2018, false), 5941429882u64);
    }

    #[test]
    fn test_second_half() {
        assert_eq!(run_simulation(515891, true), 9);
        assert_eq!(run_simulation(012451, true), 5);
        assert_eq!(run_simulation(925107, true), 18);
        assert_eq!(run_simulation(594142, true), 2018);
    }
}

fn check_recipes(recipes: &Vec<usize>, input: usize) -> u64 {
    let end = recipes.len() - 1;
    if end < 7 {
        return 0;
    }
    if recipes[end] == input % 10 {
        let mut sum = 0;
        let mut coeff = 1;
        for i in 0..6 {
            sum += recipes[end - i] * coeff;
            coeff *= 10;
        }
        if sum == input {
            return end as u64 - 5;
        }
    }

    if recipes[end - 1] == input % 10 {
        let mut sum = 0;
        let mut coeff = 1;
        for i in 0..6 {
            sum += recipes[end - 1 - i] * coeff;
            coeff *= 10;
        }
        if sum == input {
            return end as u64 - 6;
        }

    }

    0
}

fn run_simulation(input: usize, find_occurrence: bool) -> u64 {
    let mut recipes = vec![3, 7];
    let mut elves = vec![0, 1];
    loop {
        let mut sum = 0;
        for elf in &elves {
            sum += recipes[*elf];
        }

        let mut new_recipes = Vec::new();

        if sum == 0 {
            new_recipes.push(0);
        }
        while sum > 0 {
            new_recipes.push(sum % 10);
            sum /= 10;
        }

        while !new_recipes.is_empty() {
            recipes.push(new_recipes.pop().unwrap());
        }

        for elf in &mut elves {
            *elf = (*elf + recipes[*elf] + 1) % recipes.len()
        }

        if   !find_occurrence && recipes.len() >= input + 10
           || find_occurrence && check_recipes(&recipes, input) != 0 {
            break;
        }
    }

    if !find_occurrence {
        // Output last 10 recipes
        let mut ans: u64 = 0;
        for i in 0..10 {
            ans = ans*10 + recipes[input + i] as u64;
        }
        ans
    } else {
        check_recipes(&recipes, input)
    }
}

fn main() {
    println!("Score: {}", run_simulation(360781, false));
    println!("At: {}", run_simulation(360781, true));
}
