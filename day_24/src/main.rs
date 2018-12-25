use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashSet, HashMap};

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = 
                "Immune System:
                 17 units each with 5390 hit points (weak to radiation, bludgeoning) with \
                 an attack that does 4507 fire damage at initiative 2\n\
                 989 units each with 1274 hit points (immune to fire; weak to bludgeoning, \
                 slashing) with an attack that does 25 slashing damage at initiative 3\n\
                 \n\
                 Infection:\n\
                 801 units each with 4706 hit points (weak to radiation) with an attack \
                 that does 116 bludgeoning damage at initiative 1\n\
                 4485 units each with 2961 hit points (immune to radiation; weak to fire, \
                 cold) with an attack that does 12 slashing damage at initiative 4";


    #[test]
    fn test_first_half() {
        let input = String::from(TEST_INPUT);
        let mut groups = parse_input(input);
        run_battle(&mut groups);
        assert_eq!(total_units(&groups), 5216);
    }

    #[test]
    fn test_second_half() {
        let input = String::from(TEST_INPUT);
        let

         groups = parse_input(input);
        assert_eq!(find_minimum_boost(&groups), 51);
    }
}

#[derive(Debug, Clone)]
struct Group {
    is_infection: bool,
    units: u64,
    hit_points: u64,
    damage: u64,
    attack_type: String,
    initiative: u64,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
}

impl Group {
    fn from_line(line: &str, is_infection: bool) -> Group {
        let mut split = line.split(" units each with ");
        let units = split.next().unwrap()
                         .parse::<u64>().unwrap();
        let mut split = split.next().unwrap()
                             .split(" hit points");
        let hit_points = split.next().unwrap()
                              .parse::<u64>().unwrap();
        let mut split = split.next().unwrap()
                             .split("attack that does ")
                             .nth(1).unwrap()
                             .split(" ");
        let damage = split.next().unwrap()
                          .parse::<u64>().expect("2");
        let attack_type = String::from(split.next().unwrap());
        let initiative = split.nth(3).unwrap()
                              .parse::<u64>().unwrap();

        let mut weaknesses = HashSet::new();
        let weak_start = line.find("weak to");
        if weak_start != None {
            let weak_start = weak_start.unwrap();
            let mut weak_end = line.find(")").unwrap();
            let semic = line.find(";");
            if semic != None && semic.unwrap() > weak_start {
                weak_end = semic.unwrap();
            }
            let types = &line[weak_start+8..weak_end];
            for immunity in types.split(", ") {
                weaknesses.insert(String::from(immunity));
            }
        }

        let mut immunities = HashSet::new();
        let immune_start = line.find("immune to");
        if immune_start != None {
            let immune_start = immune_start.unwrap();
            let mut immune_end = line.find(")").unwrap();
            let semic = line.find(";");
            if semic != None && semic.unwrap() > immune_start {
                immune_end = semic.unwrap();
            }
            let types = &line[immune_start+10..immune_end];
            for immunity in types.split(", ") {
                immunities.insert(String::from(immunity));
            }
        }

        Group { is_infection: is_infection,
                units: units,
                hit_points: hit_points,
                damage: damage,
                attack_type: attack_type,
                initiative: initiative,
                weaknesses: weaknesses,
                immunities: immunities}
    }

    fn damage_taken(&self, attacker: &Group) -> u64 {
        if self.weaknesses.contains(&attacker.attack_type) {
            attacker.power()*2
        } else if self.immunities.contains(&attacker.attack_type) {
            0
        } else {
            attacker.power()
        }
    }

    fn power(&self) -> u64 {
        self.damage * self.units
    }

    fn find_target(&self, groups: &HashMap<u64, Group>, targeted: &HashSet<u64>) -> Option<u64> {
        groups.iter()
              .filter(|(k, v)| (v.is_infection != self.is_infection) &&
                               !targeted.contains(k) &&
                               v.damage_taken(self) > 0)
              .max_by_key(|(_k, v)| (v.damage_taken(self), v.power(), v.initiative))
              .map(|(k, _v)| *k)
    }
}

fn parse_input(input: String) -> HashMap<u64, Group> {
    let mut parsed_is = false;
    let mut armies = HashMap::new();

    let mut uid = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line == "Immune System:" {
            continue;
        }

        if line == "Infection:" {
            parsed_is = true;
            continue;
        }

        armies.insert(uid, Group::from_line(line, parsed_is));
        uid += 1;
    }

    armies
}

fn finished_battle(groups: &HashMap<u64, Group>) -> bool {
    let mut inf_found = false;
    let mut is_found = false;
    for (_uid, group) in groups {
        inf_found |=  group.is_infection;
        is_found  |= !group.is_infection;
    }
    !inf_found || !is_found
}

fn run_battle(groups: &mut HashMap<u64, Group>) {
    loop {
        let mut targets = HashMap::new();
        let mut targeted = HashSet::new();
        let mut processed = HashSet::new();
        while processed.len() < groups.len() {
            let id = groups.iter()
                             .filter(|(k, _v)| !processed.contains(*k))
                             .max_by_key(|(_k, v)| (v.power(), v.initiative))
                             .map(|(k, _v)| *k).unwrap();
            let tgt = groups[&id].find_target(groups, &targeted);

            if tgt != None {
                targets.insert(id, tgt.unwrap());
                targeted.insert(tgt.unwrap());
            }
            processed.insert(id);
        }

        let mut processed = HashSet::new();
        let mut total_kills = 0;
        loop {
            let id = groups.iter()
                           .filter(|(k, _v)| !processed.contains(*k))
                           .max_by_key(|(_k, v)| v.initiative)
                           .map(|(k, _v)| *k);
            if id == None {
                break;
            }
            let id = id.unwrap();
            processed.insert(id);

            if !targets.contains_key(&id) {
                continue;
            }

            let target = targets[&id];
            let units_killed = groups[&target].damage_taken(&groups[&id])/groups[&target].hit_points;
            total_kills += units_killed;
            if groups[&target].units <= units_killed {
                groups.remove(&target);
            } else {
                groups.entry(target).and_modify(|g| g.units -= units_killed);
            }
        }

        if total_kills == 0 || finished_battle(groups) {
            break;
        }
    }
}

fn total_units(groups: &HashMap<u64, Group>) -> u64 {
    groups.iter().map(|(_k, v)| v.units).sum()
}

fn find_minimum_boost(groups: &HashMap<u64, Group>) -> u64 {

    let mut boost = 1;
    let mut step = 1000;
    loop {
        let mut mod_groups = groups.clone();
        for (_k, g) in &mut mod_groups {
            if !g.is_infection {
                g.damage += boost;
            }
        }
        run_battle(&mut mod_groups);
        if finished_battle(&mod_groups) &&
           !mod_groups.iter().next().unwrap().1.is_infection {
            if step == 1 {
                return total_units(&mod_groups);
            } else {
                boost -= step;
                step /= 10;
            }
        }
        boost += step;
    }
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let mut groups = parse_input(input);
    let units_after_boost = find_minimum_boost(&groups);
    run_battle(&mut groups);
    println!("Combat ends with {} units", total_units(&groups));
    println!("Minimum boost leaves {} units", units_after_boost);
}
