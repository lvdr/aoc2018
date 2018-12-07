use std::io::prelude::*;
use std::fs::File;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;


#[derive(Eq, Copy, Clone, Hash)]
struct Node {
    id: u8
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        // Reverse order; to make min heap
        other.id.cmp(&self.id)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
}

struct Relations {
    antidependencies: HashSet<Node>,
    dependencies: HashSet<Node>,
}

impl Relations {
    fn new() -> Relations {
        Relations { antidependencies: HashSet::new(),
                    dependencies: HashSet::new() }
    }
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let lines : Vec<&str> = input.trim()
                                 .split("\n")
                                 .map(|x| x.trim())
                                 .collect();

    let mut graph = HashMap::new();
    for line in lines {
        let dependency = Node { id: line.as_bytes()[5 as usize] };
        let node = Node { id: line.as_bytes()[36 as usize] };
        graph.entry(node).or_insert( Relations::new() )
             .dependencies.insert(dependency);
        graph.entry(dependency).or_insert( Relations::new() )
             .antidependencies.insert(node);
    }

    let mut queue = BinaryHeap::new();
    for (node, rels) in &graph {
        if rels.dependencies.is_empty() {
            queue.push(*node);
        }
    }

    let mut exec_order = String::new();
    while !queue.is_empty() {
        let next_task = queue.pop().unwrap();
        exec_order.push(next_task.id as char);

        for antidep in &graph.get(&next_task).unwrap().antidependencies.clone() {
            graph.get_mut(antidep).unwrap().dependencies.remove(&next_task);
            if graph.get(antidep).unwrap().dependencies.is_empty() {
                queue.push(*antidep);
            }
        }
    }

    println!("{}", exec_order);
}
